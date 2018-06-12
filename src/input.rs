use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{Context, GameResult};

use types::*;

pub enum LogicalInput {
    App(AppControl),
    Direction(Direction),
}

pub enum AppControl {
    Exit,
    Pause,
}

pub struct InputHandler;

impl InputHandler {
    pub fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        trace!("{:?}", _keycode);
        debug!("{:?}", _keycode);
        info!("{:?}", _keycode);
        warn!("{:?}", _keycode);
        error!("{:?}", _keycode);
    }
}
