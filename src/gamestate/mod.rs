use specs::{Dispatcher, DispatcherBuilder, World};
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

impl<'a, 'b> GameState<'a, 'b> {
    pub fn new() -> GameState<'a, 'b> {
        let mut world = World::new();
        world.register::<physics::Position>();
        world.register::<visual::BaseSprite>();

        let mut dispatcher = DispatcherBuilder::new()
            .with(
                brains::BrainSystem::<brains::PlayerBrain>::new(),
                "player_brain",
                &[],
            )
            .with(brains::PlayerBrain {}, "player_brain_commands", &[])
            .build();
        dispatcher.setup(&mut world.res);

        {
            use self::brains::PlayerBrain;
            use self::physics::Direction;
            use self::physics::Position;
            use self::visual::BaseSprite;
            use assets::DrawableHandle;
            use ggez::graphics::Color;

            world
                .create_entity()
                .with(Position::new(5, 5, Direction::None))
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
