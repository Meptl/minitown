/// Listens for mouse wheel events and updates camera's zoom level
use ::specs;
use ::specs::{Fetch, Join, ReadStorage, WriteStorage};
use ecs::components::*;
use ecs::resources;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (Fetch<'a, resources::Input>,
                       Fetch<'a, resources::DeltaTime>,
                       WriteStorage<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut input, dt, mut cameras) = data;
        let dt = dt.0;

        for cam in (&mut cameras).join() {
            if let Some(mouse_wheel) = input.mouse_wheel {
                let wheel_updown = mouse_wheel.1 as f64;
                cam.incr_zoom(wheel_updown);
            }
            cam.update_zoom(dt);
        }
    }
}
