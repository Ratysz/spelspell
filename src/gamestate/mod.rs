use specs::prelude::*;
use specs::storage::{GenericReadStorage, MaskedStorage, UnprotectedStorage};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

mod brains;
mod command;
mod physics;
mod time;
mod visual;

pub use self::command::GameCommand;
pub use self::physics::{Direction, Position};
pub use self::visual::BaseSprite;

pub struct GameState<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
    world: World,
}

struct DispatcherBuilderWrapper<'a, 'b>(DispatcherBuilder<'a, 'b>);

impl<'a, 'b> DispatcherBuilderWrapper<'a, 'b> {
    fn with<T>(mut self, loader: T) -> DispatcherBuilderWrapper<'a, 'b>
    where
        T: Fn(DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b>,
    {
        DispatcherBuilderWrapper(loader(self.0))
    }

    fn build(mut self) -> Dispatcher<'a, 'b> {
        self.0.build()
    }
}

impl<'a, 'b> GameState<'a, 'b> {
    pub fn new() -> GameState<'a, 'b> {
        let mut world = World::new();
        world.register::<physics::Position>();
        world.register::<visual::BaseSprite>();

        let mut dispatcher = DispatcherBuilderWrapper(DispatcherBuilder::new())
            .with(brains::module_systems)
            .with(physics::module_systems)
            .build();
        dispatcher.setup(&mut world.res);

        {
            use self::brains::*;
            use self::physics::*;
            use self::visual::*;
            use assets::DrawableHandle;
            use ggez::graphics::Color;

            world
                .create_entity()
                .with(Position::new(5, 5, Direction::None))
                .with(Movable::default())
                .with(BaseSprite {
                    drawable: DrawableHandle::Circle,
                    color: Color::from([0.0, 1.0, 1.0, 1.0]),
                })
                .with(PlayerBrain {})
                .build();

            world
                .create_entity()
                .with(Position::new(10, 5, Direction::None))
                .with(BaseSprite {
                    drawable: DrawableHandle::Box,
                    color: Color::from([1.0, 0.0, 1.0, 1.0]),
                })
                .build();
        }

        GameState { dispatcher, world }
    }

    pub fn update(&mut self, d_time: Duration) {
        self.world
            .write_resource::<time::Timekeeper>()
            .update_real_time(d_time);
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn queue_command(&self, command: Option<command::GameCommand>) {
        if let Some(command) = command {
            self.world
                .write_resource::<command::GameCommandQueue>()
                .queue(command);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_once() {
        let mut state = GameState::new();
        state.update(Duration::from_secs(1));
    }
}
