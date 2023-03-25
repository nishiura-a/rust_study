use crate::{
    block::{block_kind, BlockKind, BlockShape, BLOCKS, COLOR_TABLE},
    block_control::{ghost_pos, is_collision},
    field::{Field, FIELD_HEIGHT, FIELD_WIDTH, TEMPLATE_FIELD},
    position::Position,
};

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: TEMPLATE_FIELD,
            pos: Position::init(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
        }
    }
}
// ブロックを生成する
// 生成に失敗した場合は`Err(())`を返す
pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    // posの座標を初期値へ
    game.pos = Position::init();
    // ブロックをランダム生成
    game.block = BLOCKS[rand::random::<BlockKind>() as usize];
    // 衝突チェック
    if is_collision(&game.field, &game.pos, &game.block) {
        Err(())
    } else {
        Ok(())
    }
}

// ゲームオーバー処理
pub fn gameover(game: &Game) {
    draw(game);
    println!("GAMEOVER");
    println!("press `q` key to exit");
}

// 終了処理
pub fn quit() {
    // カーソルを再表示
    println!("\x1b[?25h");
}

#[allow(clippy::needless_range_loop)]
// フィールドを描画する
pub fn draw(Game { field, pos, block }: &Game) {
    // 描画用フィールドの生成
    let mut field_buf = *field;
    // 描画用フィールドにゴーストブロックを書き込む
    let ghost_pos = ghost_pos(field, pos, block);
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y + ghost_pos.y][x + ghost_pos.x] = block_kind::GHOST;
            }
        }
    }
    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] != block_kind::NONE {
                field_buf[y + pos.y][x + pos.x] = block[y][x];
            }
        }
    }
    // フィールドを描画
    println!("\x1b[H"); // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT - 1 {
        for x in 1..FIELD_WIDTH - 1 {
            print!("{}", COLOR_TABLE[field_buf[y][x]]);
        }
        println!();
    }
}
