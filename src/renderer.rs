use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;
use specs::{Join, World};

use gamestate::physics::Position;

pub fn render(ctx: &mut Context, world: &World) -> GameResult {
    let pos_s = world.read_storage::<Position>();
    for pos in (&pos_s).join() {
        graphics::circle(
            ctx,
            graphics::Color::from([0.0, 1.0, 1.0, 1.0]),
            graphics::DrawMode::Fill,
            na::Point2::new(pos.x() as f32, pos.y() as f32),
            10.0,
            0.1,
        )?;
    }
    Ok(())
}
