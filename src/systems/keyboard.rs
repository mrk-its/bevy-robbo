use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::Level;
use crate::game_events::{GameEvent, GameEvents};
use bevy::prelude::*;

pub fn keyboard_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    (current_level_handle, mut events): (Res<Option<Handle<Level>>>, ResMut<GameEvents>),
    mut robbo_dir: ResMut<RobboDir>,
    mut query: Query<With<Robbo, (Entity, &mut MovingDir, &mut Tiles)>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Some(handle) = *current_level_handle {
            events.send(GameEvent::ReloadLevel(handle));
        }
        return;
    }
    let is_shift =
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);

    let jp_right = keyboard_input.just_pressed(KeyCode::Right) as i32;
    let jp_left = keyboard_input.just_pressed(KeyCode::Left) as i32;
    let jp_up = keyboard_input.just_pressed(KeyCode::Up) as i32;
    let jp_down = keyboard_input.just_pressed(KeyCode::Down) as i32;

    let right = keyboard_input.pressed(KeyCode::Right) as i32;
    let left = keyboard_input.pressed(KeyCode::Left) as i32;
    let up = keyboard_input.pressed(KeyCode::Up) as i32;
    let down = keyboard_input.pressed(KeyCode::Down) as i32;

    for (entity, mut moving_dir, mut tiles) in &mut query.iter() {
        if is_shift {
            let kx = jp_right - jp_left;
            let ky = jp_up - jp_down;
            if kx != 0 || ky != 0 {
                *moving_dir = MovingDir::zero();
                commands.insert_one(entity, ShootingDir::new(kx, ky));
            }
        } else {
            let kx = (right | jp_right) - (left | jp_left);
            let ky = (up | jp_up) - (down | jp_down);

            let cur_dir = MovingDir::new(kx, ky);

            robbo_dir.just_pressed =
                robbo_dir.just_pressed || (jp_left | jp_right | jp_up | jp_down) > 0;

            if !cur_dir.is_empty() || !robbo_dir.just_pressed {
                robbo_dir.dir = cur_dir;
            }

            if *moving_dir != robbo_dir.dir {
                *moving_dir = robbo_dir.dir;
                *tiles = match (kx, ky) {
                    (-1, 0) => Tiles::new(&[64, 65]),
                    (1, 0) => Tiles::new(&[60, 61]),
                    (0, -1) => Tiles::new(&[62, 63]),
                    (0, 1) => Tiles::new(&[66, 67]),
                    _ => Tiles::new(&tiles.tiles[0..1]),
                }
            }
        }
    }
}
pub fn robbo_dir_reset(mut robbo_dir: ResMut<RobboDir>, frame_cnt: Res<FrameCnt>) {
    if frame_cnt.do_it() {
        robbo_dir.just_pressed = false;
        // println!("reset just pressed: {:?}", robbo_dir.just_pressed);
    }
}

#[derive(Default, Debug)]
pub struct RobboDir {
    pub just_pressed: bool,
    pub dir: MovingDir,
}

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(KeyboardPlugin)
            .add_resource(RobboDir::default())
            .add_stage_before(stage::PRE_UPDATE, "keyboard")
            .add_stage_after(stage::POST_UPDATE, "robbo_dir_reset")
            .add_system_to_stage("keyboard", keyboard_system.system())
            .add_system_to_stage("robbo_dir_reset", robbo_dir_reset.system());
    }
}
