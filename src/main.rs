#[macro_use]
extern crate bitflags;
extern crate chrono;
extern crate fern;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate nalgebra as na;

use std::env;
use std::ops::Deref;
use std::path;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

mod input;
mod types;

struct App {
    input_handler: input::InputHandler,
}

impl App {
    pub fn new(ctx: &mut Context) -> App {
        App {
            input_handler: input::InputHandler::new(),
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

fn wrapped() -> GameResult {
    let w_dim = na::Vector2::new(640, 480);

    let (ctx, events_loop) = &mut ContextBuilder::new("SpelunkingSpellwright", "Ratys")
        .window_setup(WindowSetup::default().title("Spelunking Spellwright"))
        .window_mode(
            WindowMode::default()
                .dimensions(w_dim.x, w_dim.y)
                .max_dimensions(w_dim.x, w_dim.y)
                .min_dimensions(w_dim.x, w_dim.y),
        )
        .build()?;

    let state = &mut App::new(ctx);
    event::run(ctx, events_loop, state)
}

fn main() {
    use fern::colors::{Color, ColoredLevelConfig};
    let colors = ColoredLevelConfig::default()
        .trace(Color::BrightBlue)
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<14}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                colors.color(record.level()).to_string(),
                record.target(),
                message
            ))
        })
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    if let Err(e) = wrapped() {
        error!("{}", e);
    }
}
