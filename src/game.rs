use std::collections::VecDeque;

use crate::{
    block::{gen_block_7, BlockKind, BlockShape, BLOCKS},
    field::{Field, TEMPLATE_FIELD},
    game_control::spawn_block,
    position::Position,
};
pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
    pub hold: Option<BlockShape>,
    pub holded: bool,
    pub next: VecDeque<BlockShape>,
    pub next_buf: VecDeque<BlockShape>,
    pub score: usize,
    pub line: usize,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            field: TEMPLATE_FIELD,
            pos: Position::init(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
            hold: None,
            holded: false,
            next: gen_block_7().into(),
            next_buf: gen_block_7().into(),
            score: 0,
            line: 0,
        };
        // 初期ブロックを供給
        spawn_block(&mut game).ok();
        game
    }
}

// 得点表
pub const SCORE_TABLE: [usize; 5] = [
    0,   // 0段消し
    1,   // 1段消し
    5,   // 2段消し
    25,  // 3段消し
    100, // 4段消し
];

// ネクストブロックを3つ表示
pub const NEXT_LENGTH: usize = 3;
