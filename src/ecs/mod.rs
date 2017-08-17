pub mod components;
pub mod systems;

/// Create a world with all components
pub fn registered_world() -> ::specs::World {
    let mut world = ::specs::World::new();

    world.register::<components::Transform>();
    world.register::<components::Velocity>();
    world.register::<components::Render>();
    world.register::<components::Camera>();

    world
}
