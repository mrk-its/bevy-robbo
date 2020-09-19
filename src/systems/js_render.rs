use crate::components::{Int2Ops, Position, Tiles};
use crate::consts::*;
use bevy::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::inventory::Inventory;


pub fn js_render(
    frame_cnt: Res<FrameCnt>,
    current_level: Res<LevelInfo>,
    inventory: Res<Inventory>,
    mut items: Query<(&Position, &Tiles)>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let items: std::collections::HashMap<(i32, i32), u32> = items
        .iter()
        .iter()
        .map(|(&pos, &tiles)| (pos.as_tuple(), tiles.tiles[tiles.current]))
        .collect();

    let mut board: Vec<u8> = Vec::with_capacity((MAX_BOARD_WIDTH * MAX_BOARD_HEIGHT) as usize);
    for y in (0..MAX_BOARD_HEIGHT).rev() {
        for x in 0..MAX_BOARD_WIDTH {
            let tile = items.get(&(x, y));
            board.push(*tile.unwrap_or(&255) as u8);
        }
    }
    unsafe {
        render(
            current_level.screws - inventory.screws,
            inventory.keys,
            inventory.bullets,
            current_level.current_level + 1,
            board,
        );
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/assets/render.js")]
extern "C" {
    fn render(screws: usize, keys: usize, bullets: usize, level: usize, board: Vec<u8>);
}
