/// Updates transform based on velocity.
use ::specs;
use ::specs::{Fetch, Join, ReadStorage, WriteStorage};
use ecs::components::*;
use ecs::resources;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (Fetch<'a, resources::DeltaTime>,
                       WriteStorage<'a, Transform>,
                       ReadStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (dt, mut transform, velocity) = data;
        let dt = dt.0;

        for (trans, vel) in (&mut transform, &velocity).join() {
            trans.pos[0] += vel.spatial[0] * dt;
            trans.pos[1] += vel.spatial[1] * dt;
            trans.rot += vel.angular * dt;
        }
    }
}
