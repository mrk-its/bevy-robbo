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
    static TILE_CHARS: &[&[char]] = &[
        &['M', 'M', '░', '▒', 'T', 'a', '#', 'k', 'Ó', 'D', '#', ' '],
        &['?', '@', '@', '^', '^', 'C', 'c', '▓', '#', '░', '▒', ' '],
        &['~', ' ', ' ', ' ', ' ', '▒', '@', '@', '0', '%', '%', ' '],
        &['~', '~', '|', '|', 'T', '|', ' ', ' ', ' ', ' ', ' ', ' '],
        &['▣', '▧', ' ', ' ', ' ', 'G', 'G', 'G', 'G', ' ', ' ', ' '],
        &['R', 'R', 'R', 'R', 'R', 'R', 'R', 'R', '░', '▓', ' ', ' '],
        &['M', 'M', ' ', ' ', ' ', '.', '#', 'Ó', 'G', 'G', ' ', ' '],
        &[',', ';', '%', ';', ',', '~', '|', 'Ó', 'G', 'G', ' ', ' '],
    ];

    let mut board_str = String::with_capacity((MAX_BOARD_WIDTH * MAX_BOARD_HEIGHT) as usize);
    for y in (0..MAX_BOARD_HEIGHT).rev() {
        for x in 0..MAX_BOARD_WIDTH {
            let tile = items.get(&(x, y));
            if let Some(&tile) = tile {
                let tile = tile as usize;
                board_str.push(TILE_CHARS[tile / 12][tile % 12]);
            } else {
                board_str += " ";
            }
        }
        board_str += "\n";
    }
    unsafe {
        render(
            current_level.screws - inventory.screws,
            inventory.keys,
            inventory.bullets,
            current_level.current_level + 1,
            board_str,
        );
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/wasm/render.js")]
extern "C" {
    fn render(screws: usize, keys: usize, bullets: usize, level: usize, board: String);
}
