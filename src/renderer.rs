use ggez::graphics;
use ggez::{Context, GameResult};
use specs::{Join, World};

use assets::Assets;
use gamestate::physics::Position;
use gamestate::visual::BaseSprite;

pub fn render(ctx: &mut Context, world: &World, assets: &Assets) -> GameResult {
    let pos_s = world.read_storage::<Position>();
    let vis_s = world.read_storage::<BaseSprite>();
    for (pos, vis) in (&pos_s, &vis_s).join() {
        graphics::draw(
            ctx,
            assets.fetch_drawable(vis.drawable),
            (pos.point(), vis.color),
        )?;
    }
    Ok(())
}
