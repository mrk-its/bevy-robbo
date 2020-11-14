use crate::components::prelude::*;
use crate::entities::gun_set_shooting_dir;
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;
use bevy::prelude::*;

pub fn tick_system(
    commands: &mut Commands,
    frame_cnt: Res<FrameCnt>,
    mut game_events: ResMut<Events<GameEvent>>,
    mut items: Query<(Entity, &Position, &mut Tiles), Without<Wall>>,
    animations: Query<&Animation>,
    shooting_dirs: Query<(&Rotatable, &mut ShootingDir)>,
    rotatables: Query<&Rotatable>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    for (entity, _position, mut tiles) in items.iter_mut() {
        tiles.current = (tiles.current + 1) % tiles.tiles.len();
        if let (true, Ok(animation)) = (
            tiles.current == 0 && tiles.tiles.len() > 0,
            animations.get_component::<Animation>(entity),
        ) {
            info!("animation end");
            commands.despawn(entity);
            if let Some(event) = animation.0.as_ref() {
                game_events.send(*event);
            }
        } else if let Ok(rotatable) = rotatables.get_component::<Rotatable>(entity) {
            if rand::random::<f32>() < 0.25 {
                let shooting_dir = shooting_dirs.get_component::<ShootingDir>(entity).unwrap();
                match *rotatable {
                    Rotatable::Regular => {
                        gun_set_shooting_dir(
                            commands,
                            entity,
                            shooting_dir.rotate_clockwise(),
                        );
                    }
                    Rotatable::Random => {
                        gun_set_shooting_dir(
                            commands,
                            entity,
                            ShootingDir::by_index(rand::random::<usize>() % 4),
                        );
                    }
                };
            }
        }
    }
}
