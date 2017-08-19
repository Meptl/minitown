/// Changes camera position to be the same as its focus.
use ::specs;
use ::specs::{Entities, Join, ReadStorage, WriteStorage};
use ecs::components::*;

pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Camera>,
                       WriteStorage<'a, Transform>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, cameras, mut transform) = data;

        for (ent, cam) in (&*entities, &cameras).join() {
            if let Some(focus) = cam.focus {
                let focus_pos = transform.get(focus).expect("Camera target has no transform").pos;
                let mut cam_trans = transform.get_mut(ent).unwrap();

                cam_trans.pos = focus_pos;
            }
        }
    }
}
