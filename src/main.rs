#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate bitflags;
extern crate chrono;
extern crate fern;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate nalgebra;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod app;
mod assets;
mod gamestate;
mod input;
mod renderer;
mod types;

fn wrapped() -> GameResult {
    let w_dim = nalgebra::Vector2::new(640, 480);

    let (ctx, events_loop) = &mut ContextBuilder::new("SpelunkingSpellwright", "Ratys")
        .window_setup(WindowSetup::default().title("Spelunking Spellwright"))
        .window_mode(
            WindowMode::default()
                .dimensions(w_dim.x, w_dim.y)
                .max_dimensions(w_dim.x, w_dim.y)
                .min_dimensions(w_dim.x, w_dim.y),
        )
        .build()?;

    let state = &mut app::App::new(ctx)?;
    event::run(ctx, events_loop, state)
}

fn main() {
    #[cfg(debug_assertions)]
    {
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
    }

    if let Err(e) = wrapped() {
        error!("{}", e);
    }
}
