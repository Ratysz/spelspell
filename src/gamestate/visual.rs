use ggez::graphics::Color;
use specs::prelude::*;

use assets::DrawableHandle;

#[derive(Component, Debug)]
pub struct BaseSprite {
    pub drawable: DrawableHandle,
    pub color: Color,
}
