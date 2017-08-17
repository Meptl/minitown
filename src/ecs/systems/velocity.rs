/// Updates transform based on velocity.
use ::specs;
use ::specs::{Fetch, Join, ReadStorage, WriteStorage};
use ecs::components::*;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (Fetch<'a, ::std::time::Duration>,
                       WriteStorage<'a, Transform>,
                       ReadStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (duration, mut transform, velocity) = data;

        let dt = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;

        for (trans, vel) in (&mut transform, &velocity).join() {
            trans.pos[0] += vel.spatial[0] * dt;
            trans.pos[1] += vel.spatial[1] * dt;
            trans.rot += vel.angular * dt;
        }
    }
}
