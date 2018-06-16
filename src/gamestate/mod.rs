use chrono::Duration;
use nalgebra as na;
use specs::{Dispatcher, DispatcherBuilder, World};

use types::Direction;

pub mod physics;
pub mod visual;

pub struct GameState<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    world: World,
}

impl<'a, 'b> GameState<'a, 'b> {
    pub fn new() -> GameState<'a, 'b> {
        let mut world = World::new();
        world.register::<physics::Position>();
        world.register::<visual::BaseSprite>();
        world.add_resource(Duration::zero());

        {
            use self::physics::Position;
            use self::visual::BaseSprite;
            use assets::DrawableHandle;
            use ggez::graphics::Color;

            world
                .create_entity()
                .with(Position::new(50, 50, Direction::None))
                .with(BaseSprite {
                    drawable: DrawableHandle::Circle,
                    color: Color::from([0.0, 1.0, 1.0, 1.0]),
                })
                .build();

            world
                .create_entity()
                .with(Position::new(100, 50, Direction::None))
                .with(BaseSprite {
                    drawable: DrawableHandle::Box,
                    color: Color::from([1.0, 0.0, 1.0, 1.0]),
                })
                .build();
        }

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