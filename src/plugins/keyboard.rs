use bevy::prelude::*;

use crate::components::prelude::*;
use crate::game_events::GameEvent;
use crate::inventory::Inventory;
use crate::plugins::audio::Sound;
use crate::FrameCnt;

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

pub fn keyboard_system(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    (mut events, mut inventory, mut sounds): (ResMut<Events<GameEvent>>, ResMut<Inventory>, ResMut<Events<Sound>>),
    mut robbo_dir: ResMut<RobboDir>,
    mut query: Query<(Entity, &mut MovingDir, &mut Tiles), With<Robbo>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        events.send(GameEvent::KillRobbo);
        return;
    } else if keyboard_input.just_pressed(KeyCode::PageUp) {
        events.send(GameEvent::ReloadLevel(1));
        return;
    } else if keyboard_input.just_pressed(KeyCode::PageDown) {
        events.send(GameEvent::ReloadLevel(-1));
        return;
    }
    let is_shift =
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);

    let jp_right = (keyboard_input.just_pressed(KeyCode::Right)
        || keyboard_input.just_pressed(KeyCode::D)) as i32;
    let jp_left = (keyboard_input.just_pressed(KeyCode::Left)
        || keyboard_input.just_pressed(KeyCode::A)) as i32;
    let jp_up = (keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::W)) as i32;
    let jp_down = (keyboard_input.just_pressed(KeyCode::Down)
        || keyboard_input.just_pressed(KeyCode::S)) as i32;

    let right =
        (keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)) as i32;
    let left = (keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A)) as i32;
    let up = (keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W)) as i32;
    let down = (keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S)) as i32;

    for (entity, mut moving_dir, mut tiles) in query.iter_mut() {
        if is_shift {
            let kx = jp_right - jp_left;
            let ky = jp_up - jp_down;
            if (kx != 0 || ky != 0) && inventory.bullets > 0 {
                inventory.bullets -= 1;
                *moving_dir = MovingDir::zero();
                commands.insert_one(entity, ShootingDir::new(kx, ky));
                sounds.send(Sound::SHOT);
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
    if frame_cnt.is_keyframe() {
        robbo_dir.just_pressed = false;
    }
}

#[derive(Default, Debug)]
pub struct RobboDir {
    pub just_pressed: bool,
    pub dir: MovingDir,
}
