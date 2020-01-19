extern crate specs;
use specs::prelude::*;
use super::{Map, Position, BlocksTile, ApplyMove, EntityMoved,
    Viewshed, RunState};

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Map>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, BlocksTile>,
                        Entities<'a>,
                        WriteStorage<'a, ApplyMove>,
                        WriteStorage<'a, EntityMoved>,
                        WriteStorage<'a, Viewshed>,
                        ReadExpect<'a, Entity>,
                        WriteExpect<'a, RunState>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, mut position, blockers, entities, mut apply_move,
            mut moved,
            mut viewsheds, player_entity, mut runstate) = data;

        // Apply broad movement
        for (entity, movement, mut pos) in (&entities, &apply_move, &mut position).join() {
            let start_idx = map.xy_idx(pos.x, pos.y);
            let dest_idx = movement.dest_idx as usize;
            let is_blocking = blockers.get(entity);
            if is_blocking.is_some() {
                map.blocked[start_idx] = false;
                map.blocked[dest_idx] = true;
            }
            pos.x = movement.dest_idx % map.width;
            pos.y = movement.dest_idx / map.width;
            if let Some(vs) = viewsheds.get_mut(entity) {
                vs.dirty = true;
            }
            moved.insert(entity, EntityMoved{}).expect("Unable to insert");
        }
        apply_move.clear();
    }
}
