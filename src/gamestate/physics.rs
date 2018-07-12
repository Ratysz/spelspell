use specs::prelude::*;

use super::time::*;

pub fn module_systems<'a, 'b>(builder: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    builder.with(
        TimingSystem::<Movable>::new(),
        "movable_timing",
        &["player_commands"],
    )
}

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

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}

impl Direction {
    pub fn invert(self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::NE => Direction::SW,
            Direction::E => Direction::W,
            Direction::SE => Direction::NW,
            Direction::S => Direction::N,
            Direction::SW => Direction::NE,
            Direction::W => Direction::E,
            Direction::NW => Direction::SE,
            _ => unimplemented!(),
        }
    }
}

#[derive(Component, Debug, Default)]
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

#[derive(Component, Debug, Default)]
pub struct Movable {
    direction: Direction,
}

impl Movable {
    pub fn new(direction: Direction) -> Movable {
        Movable { direction }
    }

    pub fn start_moving(
        &mut self,
        entity: &Entity,
        time: &Timekeeper,
        timing_data: &mut TimingData<Movable>,
        direction: Direction,
        duration: Duration,
    ) {
        if let DirectedTime::Past(_) = time.delta() {
            self.direction = direction.invert();
        } else {
            self.direction = direction;
        }
        self.schedule(entity, time, timing_data, duration);
    }
}

impl Timed for Movable {}

mod tests {
    use super::*;
}
