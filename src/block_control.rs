use crate::constant::{
    BlockKind, Field, Position, BLOCKS, FIELD_HEIGHT, FIELD_WIDTH, TEMPLATE_FIELD,
};

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockKind,
}

impl Game {
    pub fn new() -> Game {
        Game {
            field: TEMPLATE_FIELD,
            pos: Position::init(),
            block: rand::random::<BlockKind>(),
        }
    }
}

#[allow(clippy::needless_range_loop)]
// フィールドを描画する
pub fn draw(Game { field, pos, block }: &Game) {
    // 描画用フィールドの生成
    let mut field_buf = field.clone();
    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field_buf[y + pos.y][x + pos.x] = 1;
            }
        }
    }
    // フィールドを描画
    println!("\x1b[H"); // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

// ブロックがフィールドに衝突する場合は`true`を返す
pub fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    return false;
}

// ブロックをフィールドに固定する
pub fn fix_block(Game { field, pos, block }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            if BLOCKS[*block as usize][y][x] == 1 {
                field[y + pos.y][x + pos.x] = 1;
            }
        }
    }
}

// 消せるラインがあるなら削除し、段を下げる
pub fn erase_line(field: &mut Field) {
    for y in 1..FIELD_HEIGHT - 1 {
        let mut can_erase = true;
        for x in 1..FIELD_WIDTH - 1 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
}

// ブロックを指定した座標へ移動できるなら移動する
pub fn move_block(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, game.block) {
        // posの座標を更新
        game.pos = new_pos;
    }
}
