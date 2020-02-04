extern crate specs;
use crate::{MyTurn, Position, Speed};
use specs::prelude::*;

pub struct ActionPointSystem {}

impl<'a> System<'a> for ActionPointSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Speed>,
        WriteStorage<'a, MyTurn>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut speeds, mut turns, positions) = data;

        //regain move point
        for (_entity, speed) in (&entities, &mut speeds).join() {
            speed.add_move_point(speed.point_per_turn);
        }

        //reset turn for everyone
        for (entity, _pos) in (&entities, &positions).join() {
            turns
                .insert(entity, MyTurn {})
                .expect("Unable to insert turn");
        }
    }
}
