use crate::{
    block::{block_kind, BlockKind, BlockShape, BLOCKS, COLOR_TABLE},
    block_control::{ghost_pos, is_collision},
    field::{Field, FIELD_HEIGHT, FIELD_WIDTH, TEMPLATE_FIELD},
    position::Position,
};

use std::collections::VecDeque;

// ネクストブロックを3つ表示
pub const NEXT_LENGTH: usize = 3;

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
    pub hold: Option<BlockShape>,
    pub holded: bool,
    pub next: VecDeque<BlockShape>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: TEMPLATE_FIELD,
            pos: Position::init(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
            hold: None,
            holded: false,
            next: {
                let mut deque = VecDeque::new();
                for _ in 0..NEXT_LENGTH {
                    deque.push_back(BLOCKS[rand::random::<BlockKind>() as usize]);
                }
                deque
            },
        }
    }
}
// ブロックを生成する
// 生成に失敗した場合は`Err(())`を返す
pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    // posの座標を初期値へ
    game.pos = Position::init();
    // ネクストキューから次のブロックを取り出す
    game.block = game.next.pop_front().unwrap();
    // ブロックをランダム生成して、ネクストキューに追加
    game.next
        .push_back(BLOCKS[rand::random::<BlockKind>() as usize]);
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
pub fn draw(
    Game {
        field,
        pos,
        block,
        hold,
        holded: _,
        next,
    }: &Game,
) {
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
    // ホールドを描画
    println!("\x1b[2;28HHOLD"); // カーソルをホールド位置に移動
    if let Some(hold) = hold {
        for y in 0..4 {
            print!("\x1b[{};28H", y + 3); // カーソルを移動
            for x in 0..4 {
                print!("{}", COLOR_TABLE[hold[y][x]]);
            }
            println!();
        }
    }
    // ネクストを描画
    println!("\x1b[8;28HNEXT"); // カーソルをネクスト位置に移動
    for (i, next) in next.iter().enumerate() {
        for y in 0..4 {
            print!("\x1b[{};28H", i * 4 + y + 9); // カーソルを移動
            for x in 0..4 {
                print!("{}", COLOR_TABLE[next[y][x]]);
            }
            println!();
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
    // 色情報をリセット
    println!("\x1b[0m");
}

// ホールド処理
// - 1回目のホールドは現在のブロックをホールド
// - 2回目以降のホールドは現在のブロックとホールドを交換
// - 現在のブロックに対して既にホールドしている場合は何もしない
pub fn hold(game: &mut Game) {
    if game.holded {
        // 現在のブロックに対して既にホールドしている場合は早期リターン
        return;
    }
    if let Some(mut hold) = game.hold {
        // ホールドの交換
        std::mem::swap(&mut hold, &mut game.block);
        game.hold = Some(hold);
        game.pos = Position::init();
    } else {
        // ホールドして、新たなブロックを生成
        game.hold = Some(game.block);
        spawn_block(game).ok();
    }
    // ホールド済のフラグを立てる
    game.holded = true;
}
