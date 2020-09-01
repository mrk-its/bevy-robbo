use crate::components::{Destroyable, Position, Wall, Bomb, MovingDir, Int2Ops, Kind};
use crate::game_events::{GameEvents, GameEvent};
use crate::frame_cnt::FrameCnt;
use crate::inventory::Inventory;
use bevy::prelude::*;

pub fn game_event_system(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut events: ResMut<GameEvents>,
    mut inventory: ResMut<Inventory>,
    mut items: Query<Without<Wall, (Entity, &Position, &Kind)>>,
    bombs: Query<&Bomb>,
    destroyable: Query<&Destroyable>,
) {
    if !frame_cnt.do_it() {
        return;
    }

    for event in events.take().iter() {
        match *event {
            GameEvent::Damage(position, is_bomb) => {
                for (entity, pos, kind) in &mut items.iter() {
                    if position == *pos {
                        if bombs.get::<Bomb>(entity).is_ok() {
                            for ky in -1..=1 {
                                for kx in -1..=1 {
                                    if kx != 0 || ky != 0 {
                                        let damage_pos = pos.add(&MovingDir::new(kx, ky));
                                        events.send(GameEvent::Damage(damage_pos, true));
                                    }
                                }
                            }
                        }
                        if destroyable.get::<Destroyable>(entity).is_ok() || is_bomb {
                            commands.despawn(entity);
                        }
                    }
                }
            }
            GameEvent::Remove(position) => {
                for (entity, pos, kind) in &mut items.iter() {
                    if position == *pos {
                        commands.despawn(entity);
                    }
                }
            }
            GameEvent::RemoveEntity(entity) => {
                commands.despawn(entity);
            }
            GameEvent::Use(position) => {
                for (entity, pos, kind) in &mut items.iter() {
                    if position == *pos {
                        match &kind {
                            Kind::Door => {
                                if inventory.keys > 0 {
                                    inventory.keys -= 1;
                                    commands.despawn(entity);
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}
