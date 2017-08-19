/// Backing structures for the game world.
/// Mostly consists of components for specs.

use ::cgmath::{Rad, Point2, Vector2};
use ::ggez::graphics::Image;
use ::specs::{Entity, HashMapStorage, VecStorage};
use ::std::sync::Arc;

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
/// `zoom` - The current zoom of the camera. Equivalent to a z height
/// `zoom_desired` - Goal zoom level. Used for interpolation.
/// `zoom_limits` - Zoom level clamps
/// `zoom_speed` - How much each unit of the mouse wheel effects zoom_desired.
/// `focus` - Entity that will always be at screen center.
///
/// With 45deg FOV a 1x1 object at zoom 2
pub struct Camera {
    pub zoom: f64,
    pub zoom_desired: f64,
    pub zoom_limits: (f64, f64),
    pub zoom_speed: f64,
    pub focus: Option<Entity>,
}

impl Camera {
    pub fn new(focus_ent: Entity) -> Camera {
        Camera {
            focus: Some(focus_ent),
            ..Default::default()
        }
    }

    /// Modifies desired zoom with the given increment.
    /// Clamps on zoom range
    pub fn incr_zoom(&mut self, inc: f64) {
        self.zoom_desired += inc;

        if self.zoom_desired > self.zoom_limits.1 {
            self.zoom_desired = self.zoom_limits.1;
        }
        if self.zoom_desired < self.zoom_limits.0 {
            self.zoom_desired = self.zoom_limits.0;
        }
    }

    /// Ideally performs linear interpolation on zoom
    /// (reason for zoom_desired)
    /// For now this just updates via dt
    pub fn update_zoom(&mut self, dt: f64) {
        let update = dt * if self.zoom_desired < self.zoom { -1.0 } else { 1.0 };
        self.zoom += update * self.zoom_speed;
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            zoom: 6.0,
            zoom_desired: 6.0,
            zoom_limits: (1.0, 10.0),
            zoom_speed: 6.0,
            focus: None
        }
    }
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
    pub move_speed: f64,
}

#[derive(Component)]
#[component(VecStorage)]
pub struct Render {
    pub tex: Arc<Image>,
}
