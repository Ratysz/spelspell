use nalgebra as na;
use specs::prelude::*;

use types::Direction;

#[derive(Component, Debug)]
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

    pub fn point(&self) -> na::Point2<f32> {
        na::Point2::new(self.x as f32, self.y as f32)
    }

    pub fn r(&self) -> Direction {
        self.r
    }
}
