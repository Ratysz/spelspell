use chrono::Duration;
use nalgebra as na;
use specs::{Dispatcher, DispatcherBuilder, World};

use types::Direction;

pub mod physics;

pub struct GameState<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    world: World,
}

impl<'a, 'b> GameState<'a, 'b> {
    pub fn new() -> GameState<'a, 'b> {
        let mut world = World::new();
        world.register::<physics::Position>();
        world.add_resource(Duration::zero());

        world
            .create_entity()
            .with(physics::Position::new(50, 50, Direction::None));

        let dispatcher = DispatcherBuilder::new().build();

        GameState { dispatcher, world }
    }

    pub fn update(&mut self, d_time: Duration) {
        *self.world.write_resource::<Duration>() = d_time;
        self.dispatcher.dispatch(&mut self.world.res);
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }
}
