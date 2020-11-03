use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::resources::DamageMap;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn force_field_system(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut damage_map: ResMut<DamageMap>,
    mut force_field: Query<(&ForceField, &ForceFieldBounds, &mut Position)>,
    all: Query<Without<ForceField, Without<Wall, (&Position, Entity)>>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }

    let entity_by_pos = all
        .iter()
        .map(|(pos, entity)| (*pos, entity))
        .collect::<HashMap<_, _>>();

    let force_field = force_field.iter_mut();
    let mut force_field = force_field.into_iter().collect::<Vec<_>>();
    force_field.sort_by_cached_key(|(_, _, pos)| (pos.x(), pos.y()));

    for (ff, bounds, pos) in &mut force_field.iter_mut() {
        **pos = pos.add(&ff.0);
        if pos.y() < bounds.0 {
            **pos = Position::new(pos.x(), bounds.1 - 1);
        } else if pos.y() >= bounds.1 {
            **pos = Position::new(pos.x(), bounds.0);
        }
        if let Some(&entity) = entity_by_pos.get(&**pos) {
            if all.get_component::<Bullet>(entity).is_ok() {
                damage_map.do_damage(pos, false);
            }
            commands.despawn(entity);
        }
    }
}
