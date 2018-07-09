use specs::prelude::*;

use super::time::{DirectedTime, Duration, Instant, Timekeeper};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    U,
    D,
    None,
}

#[derive(Component, Debug)]
#[storage(FlaggedStorage)]
pub struct Position {
    x: i32,
    y: i32,
    r: Direction,
}

impl Position {
    pub fn new(x: i32, y: i32, r: Direction) -> Position {
        Position { x, y, r }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn r(&self) -> Direction {
        self.r
    }
}

pub struct Movement {
    direction: Direction,
    start_time: Instant,
    end_time: Instant,
}
