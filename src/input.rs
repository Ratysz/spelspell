use ggez::event::{EventHandler, KeyCode};
use ggez::{Context, GameResult};

use types::{Direction, KeyMod};

pub enum LogicalInput {
    App(AppControl),
    Direction(Direction),
}

pub enum AppControl {
    Exit,
    Pause,
}

pub struct InputHandler {}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {}
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMod,
        repeat: bool,
    ) {
        if !repeat {
            info!("Keycode: {:?}, modifiers: {:?}", keycode, keymods);
            if keymods.contains(KeyMod::SHIFT) && keycode == KeyCode::Escape {
                ctx.quit();
            }
        }
    }
}
