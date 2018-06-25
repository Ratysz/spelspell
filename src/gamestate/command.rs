use specs::prelude::*;
use std::collections::VecDeque;

use super::physics::Direction;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameCommand {
    Move(Direction),
}

pub struct GameCommandQueue {
    queue: VecDeque<GameCommand>,
}

impl Default for GameCommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl GameCommandQueue {
    pub fn new() -> GameCommandQueue {
        GameCommandQueue {
            queue: VecDeque::new(),
        }
    }

    pub fn queue(&mut self, command: GameCommand) {
        self.queue.push_back(command);
    }

    pub fn pop(&mut self) -> Option<GameCommand> {
        self.queue.pop_front()
    }
}
