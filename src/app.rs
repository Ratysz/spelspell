use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, GameResult};

use input::InputHandler;

pub struct App {
    input_handler: InputHandler,
}

impl App {
    pub fn new(ctx: &mut Context) -> App {
        App {
            input_handler: InputHandler::default(),
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        self.input_handler
            .key_down_event(ctx, keycode, keymods.into(), repeat);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(0.0, 0.0, width as f32, height as f32),
        ).unwrap();
    }
}
