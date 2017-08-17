/// Backing structures for the game world.
/// Mostly consists of components for specs.

use ::cgmath::{Rad, Point2, Vector2};
use ::specs::{Entity, HashMapStorage, VecStorage, World};
use ::std::sync::Arc;

/// --- Resources ---

pub struct DeltaTime(pub f64);

/// --- Components ---

#[derive(Component, Clone)]
#[component(VecStorage)]
pub struct Transform {
    pub pos: Point2<f64>,
    pub rot: Rad<f64>,
    pub scale: f64,
}

/*
impl Transform {
    pub fn get_direction(&self) -> Vector2<f64> {
        let rot: Basis2<f64> = Rotation2::from_angle(self.orient);
        rot.rotate_vector(Vector2::unit_y())
    }
}
*/

#[derive(Component)]
#[component(VecStorage)]
pub struct Camera {
    pub focus: Entity
}

#[derive(Component, Clone)]
#[component(VecStorage)]
pub struct Velocity {
    pub spatial: Vector2<f64>,
    pub angular: Rad<f64>,
}

#[derive(Component)]
#[component(HashMapStorage)]
pub struct Control {
    pub thrust_speed: f64,
    pub turn_speed: f64,
}

#[derive(Component)]
#[component(VecStorage)]
pub struct Render {
    //pub tex: Arc<Texture>,
}

