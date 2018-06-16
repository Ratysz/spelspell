use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, GameResult};

use assets::Assets;
use gamestate::GameState;
use input::InputHandler;
use renderer;

pub struct App<'a, 'b> {
    input_handler: InputHandler,
    game_state: GameState<'a, 'b>,
    assets: Assets,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<App<'a, 'b>> {
        Ok(App {
            input_handler: InputHandler::default(),
            game_state: GameState::new(),
            assets: Assets::new(ctx)?,
        })
    }
}

impl<'a, 'b> EventHandler for App<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
        renderer::render(ctx, self.game_state.get_world(), &self.assets)?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, rpt: bool) {
        self.input_handler
            .key_down_event(ctx, key, mods.into(), rpt);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(0.0, 0.0, width as f32, height as f32),
        ).unwrap();
    }
}
