use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;
use specs::{Join, World};

use assets::Assets;
use gamestate::BaseSprite;
use gamestate::Position;

pub const TILE_SIZE_PX: (f32, f32) = (10.0, 10.0);

pub fn render(ctx: &mut Context, world: &World, assets: &Assets) -> GameResult {
    let pos_s = world.read_storage::<Position>();
    let vis_s = world.read_storage::<BaseSprite>();
    for (pos, vis) in (&pos_s, &vis_s).join() {
        graphics::draw(
            ctx,
            assets.fetch_drawable(vis.drawable),
            (
                na::Point2::new(
                    pos.x() as f32 * TILE_SIZE_PX.0,
                    pos.y() as f32 * TILE_SIZE_PX.1,
                ),
                vis.color,
            ),
        )?;
    }
    Ok(())
}
