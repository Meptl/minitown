/// Vertical and horizontal motion based on user input
use ::specs;
use ::specs::{Fetch, Join, ReadStorage, WriteStorage};
use ecs::components::*;
use ecs::resources;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (Fetch<'a, resources::Input>,
                       ReadStorage<'a, Control>,
                       WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (input, control, mut velocity) = data;

        for (control, vel) in (&control, &mut velocity).join() {
            vel.spatial[0] = control.move_speed * input.horizontal_axis();
            vel.spatial[1] = control.move_speed * input.vertical_axis();
        }
    }
}
