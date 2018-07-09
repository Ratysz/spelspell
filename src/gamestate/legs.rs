use specs::prelude::*;
use std::marker::PhantomData;

use super::physics::Direction;
use super::command::{GameCommand, GameCommandQueue};

pub struct Legs {
    movement: Option<(Direction, Duration)>,

}