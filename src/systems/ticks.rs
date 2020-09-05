use crate::components::prelude::*;
use crate::entities::gun_set_shooting_dir;
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvents;
use bevy::prelude::*;

pub fn tick_system(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut game_events: ResMut<GameEvents>,
    mut items: Query<Without<Wall, (Entity, &Position, &mut Tiles)>>,
    all: Query<(Entity, &Position)>,
    shooting_dirs: Query<(&Rotatable, &mut ShootingDir)>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    for (entity, _position, mut tiles) in &mut items.iter() {
        tiles.current = (tiles.current + 1) % tiles.tiles.len();
        if let (true, Ok(animation)) = (
            tiles.current == 0 && tiles.tiles.len() > 0,
            all.get::<Animation>(entity),
        ) {
            commands.despawn(entity);
            if let Some(event) = animation.0.as_ref() {
                game_events.send(*event);
            }
        } else if let Ok(rotatable) = all.get::<Rotatable>(entity) {
            if rand::random::<f32>() < 0.25 {
                let shooting_dir = shooting_dirs.get::<ShootingDir>(entity).unwrap();
                match *rotatable {
                    Rotatable::Regular => {
                        gun_set_shooting_dir(
                            &mut commands,
                            entity,
                            shooting_dir.rotate_clockwise(),
                        );
                    }
                    Rotatable::Random => {
                        gun_set_shooting_dir(
                            &mut commands,
                            entity,
                            ShootingDir::by_index(rand::random::<usize>() % 4),
                        );
                    }
                };
            }
        }
    }
}
