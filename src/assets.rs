use ggez::graphics::{self, DrawMode, Drawable, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use nalgebra as na;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DrawableHandle {
    Circle,
    Box,
}

pub struct Assets {
    drawables: Vec<Box<Drawable>>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut drawables = Vec::<Box<Drawable>>::with_capacity(DrawableHandle::Box as usize + 1);

        drawables.push(Box::new(Mesh::new_circle(
            ctx,
            DrawMode::Fill,
            na::Point2::origin(),
            5.0,
            0.1,
        )?));

        drawables.push(Box::new(Mesh::new_polygon(
            ctx,
            DrawMode::Fill,
            &[
                na::Point2::new(-5.0, -5.0),
                na::Point2::new(5.0, -5.0),
                na::Point2::new(5.0, 5.0),
                na::Point2::new(-5.0, 5.0),
            ],
        )?));

        Ok(Assets { drawables })
    }

    pub fn fetch_drawable(&self, handle: DrawableHandle) -> &Drawable {
        self.drawables[handle as usize].as_ref()
    }
}
