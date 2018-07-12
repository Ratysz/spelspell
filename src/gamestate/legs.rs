use specs::prelude::*;
use std::marker::PhantomData;

use super::physics::Direction;
use super::command::{GameCommand, GameCommandQueue};

#[derive(Component)]
pub struct Legs;