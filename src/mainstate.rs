/// Handlers for various events: render, input, etc.
use ::cgmath::{Rad, Point2, Vector2};
use ::ggez::{Context, GameResult};
use ::ggez::event::{Axis, Button, Mod, MouseButton, MouseState, Keycode};
use ::ecs::{components, systems};
use ::std::time::Duration;

fn init_game_world() -> (::specs::World, ::specs::Entity) {
    let mut world = ::ecs::registered_world();

    // Load assets
    /*let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = Arc::new(Texture::from_path(assets.join("rust.png")).unwrap());

    */
    // Create entities with these assets
    let player = world.create_entity()
        .with(components::Transform {
            pos: Point2 { x: 3.0, y: 0.0 },
            rot: Rad(0.0),
            scale: 0.5,
        })
        .with(components::Velocity {
            spatial: Vector2 { x: 30.0, y: 0.0},
            angular: Rad(0.0),
        })
        .build();

    let player2 = world.create_entity()
        .with(components::Transform {
            pos: Point2 { x: 0.0, y: 0.0 },
            rot: Rad(0.0),
            scale: 0.5,
        })
        .with(components::Velocity {
            spatial: Vector2 { x: 0.0, y: 0.0},
            angular: Rad(0.0),
        })
        .build();

    let cam = world.create_entity()
        .with(components::Transform {
            pos: Point2 { x: 0.0, y: 0.0 },
            rot: Rad(0.0),
            scale: 1.0,
        })
        .with(components::Camera {
            // Since an entity is just an index this should be ok
            focus: player.clone()
        })
        .build();

    // This will be updated per update tick
    world.add_resource(Duration::from_millis(50));

    (world, cam)
}

pub struct MainState<'a, 'b> {
    world: ::specs::World,
    cam: ::specs::Entity,
    dispatcher: ::specs::Dispatcher<'a, 'b>
}

impl<'a, 'b> MainState<'a, 'b> {
    pub fn new() -> GameResult<MainState<'a, 'b>> {
        // Init ECS and world
        let (mut world, mut cam) = init_game_world();
        let mut dispatcher = ::specs::DispatcherBuilder::new()
            .add(systems::velocity::System, "velocity", &[])
            .add(systems::camera::System, "camera_focus", &[])
            .build();

        Ok(MainState {
            world: world,
            cam: cam,
            dispatcher: dispatcher
        })
    }
}

impl<'a, 'b> ::ggez::event::EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        {
            let mut delta = self.world.write_resource::<Duration>();
            *delta = dt;
        }

        self.dispatcher.dispatch(&mut self.world.res);
        //world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use ggez::graphics;
        graphics::clear(ctx);

        // Render all entities that have the render & transform component.
        let renderables = self.world.read::<components::Render>();
        let transforms = self.world.read::<components::Transform>();

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(&mut self, _state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        println!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x,
            y,
            xrel,
            yrel
        );
    }

    fn mouse_wheel_event(&mut self, x: i32, y: i32) {
        println!("Mousewheel event, x: {}, y: {}", x, y);
    }


    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );
    }
    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );
    }

    fn controller_button_down_event(&mut self, btn: Button, instance_id: i32) {
        println!(
            "Controller button pressed: {:?} Controller_Id: {}",
            btn,
            instance_id
        );
    }

    fn controller_button_up_event(&mut self, btn: Button, instance_id: i32) {
        println!(
            "Controller button released: {:?} Controller_Id: {}",
            btn,
            instance_id
        );
    }

    fn controller_axis_event(&mut self, axis: Axis, value: i16, instance_id: i32) {
        println!(
            "Axis Event: {:?} Value: {} Controller_Id: {}",
            axis,
            value,
            instance_id
        );
    }


    fn focus_event(&mut self, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }
}
