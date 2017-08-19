/// Handlers for various events: render, input, etc.
use ::cgmath::{Rad, Point2, Vector2};
use ::ggez::{Context, GameResult};
use ::ggez::event::{Mod, MouseButton, MouseState, Keycode};
use ::ecs::{components, resources, systems};
use ::std::sync::Arc;

/// Creates initial world with entities
fn init_game_world(ctx: &mut Context) -> (::specs::World, ::specs::Entity) {
    use ::ggez::graphics::Image;
    let mut world = ::ecs::registered_world();

    // Load image assets
    let image = Arc::new(Image::new(ctx, "/rust.png").unwrap());

    // Create entities with these assets
    let player = world.create_entity()
        .with(components::Transform {
            pos: Point2 { x: 3.0, y: 0.0 },
            rot: Rad(1.0),
            scale: 0.5,
        })
        .with(components::Velocity {
            spatial: Vector2 { x: 0.0, y: 0.0},
            angular: Rad(0.0),
        })
        .with(components::Render {
            tex: image.clone()
        })
        .with(components::Control {
            move_speed: 100.0,
        })
        .build();

    let reference = world.create_entity()
        .with(components::Transform {
            pos: Point2 { x: 100.0, y: 100.0 },
            rot: Rad(0.0),
            scale: 0.5,
        })
        .with(components::Velocity {
            spatial: Vector2 { x: 0.0, y: 0.0},
            angular: Rad(0.0),
        })
        .with(components::Render {
            tex: image.clone()
        })
        .build();

    let cam = world.create_entity()
        .with(components::Transform {
            pos: Point2 { x: 0.0, y: 0.0 },
            rot: Rad(0.0),
            scale: 1.0,
        })
        // Since an entity is just an index this clone should be ok
        .with(components::Camera::new(player.clone()))
        .build();

    (world, cam)
}

pub struct MainState<'a, 'b> {
    world: ::specs::World,
    cam: ::specs::Entity,
    dispatcher: ::specs::Dispatcher<'a, 'b>
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {
        // Init ECS and world
        let (mut world, mut cam) = init_game_world(ctx);
        let mut dispatcher = ::specs::DispatcherBuilder::new()
            .add(systems::velocity::System, "velocity", &[])
            .add(systems::control::System, "control", &[])
            .add(systems::camera_focus::System, "camera_focus", &[])
            .add(systems::camera_zoom::System, "camera_zoom", &[])
            .build();

        Ok(MainState {
            world: world,
            cam: cam,
            dispatcher: dispatcher
        })
    }
}

impl<'a, 'b> ::ggez::event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context, dt: ::std::time::Duration) -> GameResult<()> {
        {
            let dt = dt.as_secs() as f64 + dt.subsec_nanos() as f64 * 1e-9;
            let mut delta = self.world.write_resource::<resources::DeltaTime>();
            *delta = resources::DeltaTime(dt);
        }

        self.dispatcher.dispatch(&mut self.world.res);
        //world.maintain();

        // Reset the mouse wheel after the system has used the event
        let mut input = self.world.write_resource::<resources::Input>();
        input.mouse_wheel = None;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics;
        use ggez::graphics::{Point, DrawParam};
        use specs::Join;

        graphics::clear(ctx);

        // Render all entities that have the render & transform component.
        let cameras = self.world.read::<components::Camera>();
        let renderables = self.world.read::<components::Render>();
        let transforms = self.world.read::<components::Transform>();

        // Use the same transform storage to get the camera's transform
        let cam_trans = transforms.get(self.cam).unwrap();
        let cam_zooms = cameras.get(self.cam).unwrap();

        // We've arbitrarily chosen 6 as pixel to screen ratio
        // A value of 3 zoom would have things twice as large.
        let zoom_ratio = cam_zooms.zoom / 6.0;

        let screen = graphics::get_screen_coordinates(ctx);

        for (render, trans) in (&renderables, &transforms).join() {
            // Object of focus is in center.
            // Screen height is negative due to ggez's flipped vertical axis.
            // We subtract cam_relative_pos so that our vertical axis becomes flipped.
            // Positive Y is up.
            let location = {
                let cam_relative_pos = trans.pos - cam_trans.pos;
                let zoom_scaled_pos = Point2 {
                    x: cam_relative_pos.x * zoom_ratio,
                    y: cam_relative_pos.y * zoom_ratio
                };

                Point::new(zoom_scaled_pos.x as f32 + screen.w / 2.0,
                           screen.h / -2.0 - zoom_scaled_pos.y as f32)
            };

            // Our usage of scale is the same on both the X and Y axis
            let scale = trans.scale * zoom_ratio;

            let draw_param = DrawParam {
                dest: location,
                rotation: trans.rot.0 as f32,
                scale: Point::new(scale as f32, scale as f32),
                ..Default::default()
            };
            graphics::draw_ex(ctx, &*render.tex, draw_param);
        }

        graphics::present(ctx);
        Ok(())
    }

    /// One would typically modify game structures in these input handlers,
    /// but because we are using specs we simply add input information to the world
    /// as resources and allow the control system to manage itself.
    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        let mut input = self.world.write_resource::<resources::Input>();
        input.mouse_click = Some((x, y));
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        let mut input = self.world.write_resource::<resources::Input>();
        input.mouse_click = None;
    }

    /// This event doesn't have a clear end like button stop does
    fn mouse_motion_event(&mut self, _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        let mut input = self.world.write_resource::<resources::Input>();
        input.mouse_location = (x, y);
    }

    fn mouse_wheel_event(&mut self, x: i32, y: i32) {
        let mut input = self.world.write_resource::<resources::Input>();
        input.mouse_wheel = Some((x, y));
    }


    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<resources::Input>();
        match keycode {
            Keycode::W => {
                input.up = true;
            },
            Keycode::A => {
                input.left = true;
            },
            Keycode::S => {
                input.down = true;
            },
            Keycode::D => {
                input.right = true;
            },
            _ => {}
        }
    }

    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<resources::Input>();
        match keycode {
            Keycode::W => {
                input.up = false;
            },
            Keycode::A => {
                input.left = false;
            },
            Keycode::S => {
                input.down = false;
            }
            Keycode::D => {
                input.right = false;
            },
            _ => {}
        }
    }

    fn focus_event(&mut self, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }

    fn resize_event(&mut self, _ctx: &mut Context, w: u32, h: u32) {
        println!("New size: {} {}", w, h);
    }
}
