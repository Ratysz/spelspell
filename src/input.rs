use ggez::event::{KeyCode, MouseButton};
use ggez::Context;
use std::collections::HashMap;

use gamestate::Direction;
use gamestate::GameCommand;
use keymod::KeyMod;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Input {
    Key(KeyCode),
    Mouse(MouseButton),
    #[cfg(test)]
    Test,
}

#[derive(Debug)]
enum InputExtra {
    None,
    RepeatedKey(bool),
    XY(f32, f32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Command {
    App(AppCommand),
    Game(GameCommand),
    #[cfg(test)]
    Test(usize),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppCommand {
    Exit,
    Pause,
}

type Bindings = HashMap<Input, Vec<(KeyMod, Command)>>;

pub struct InputHandler {
    bindings: Bindings,
}

impl Default for InputHandler {
    fn default() -> InputHandler {
        let mut handler = InputHandler::new();
        handler
            .bind(
                Input::Key(KeyCode::Q),
                KeyMod::CTRL | KeyMod::ALT,
                Command::App(AppCommand::Exit),
            )
            .bind(
                Input::Key(KeyCode::W),
                KeyMod::NONE,
                Command::Game(GameCommand::Move(Direction::N)),
            )
            .bind(
                Input::Key(KeyCode::A),
                KeyMod::NONE,
                Command::Game(GameCommand::Move(Direction::W)),
            )
            .bind(
                Input::Key(KeyCode::S),
                KeyMod::NONE,
                Command::Game(GameCommand::Move(Direction::S)),
            )
            .bind(
                Input::Key(KeyCode::D),
                KeyMod::NONE,
                Command::Game(GameCommand::Move(Direction::E)),
            );
        handler
    }
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            bindings: Bindings::new(),
        }
    }

    pub fn bind(&mut self, input: Input, keymods: KeyMod, action: Command) -> &mut InputHandler {
        {
            let mut done = false;
            for mut bound_action in self.bindings.entry(input).or_insert_with(Vec::new) {
                if bound_action.0 == keymods {
                    bound_action.1 = action;
                    done = true;
                    break;
                }
            }
            if !done {
                let bound_action_bunch = self.bindings.entry(input).or_insert_with(Vec::new);
                let count = keymods.bits().count_ones();
                let mut index = 0;
                for bound_action in bound_action_bunch.iter() {
                    if count >= bound_action.0.bits().count_ones() {
                        break;
                    }
                    index += 1;
                }
                bound_action_bunch.insert(index, (keymods, action));
            }
        }
        self
    }

    fn resolve(&self, input: Input, keymods: KeyMod) -> Option<Command> {
        if let Some(bound_action_bunch) = self.bindings.get(&input) {
            for bound_action in bound_action_bunch {
                if keymods.contains(bound_action.0) {
                    return Some(bound_action.1);
                }
            }
        }
        None
    }

    fn execute(
        ctx: &mut Context,
        action: Option<Command>,
        input: InputExtra,
    ) -> Option<GameCommand> {
        if let Some(action) = action {
            trace!("Action: {:?}, input extra: {:?}", action, input);
            match action {
                Command::App(command) => match command {
                    AppCommand::Exit => ctx.quit(),
                    AppCommand::Pause => unimplemented!(),
                },
                Command::Game(command) => return Some(command),
                #[cfg(test)]
                Command::Test(_) => unimplemented!(),
            }
        }
        None
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        mods: KeyMod,
        repeat: bool,
    ) -> Option<GameCommand> {
        let command = self.resolve(Input::Key(key), mods);
        InputHandler::execute(ctx, command, InputExtra::RepeatedKey(repeat))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_resolution() {
        let mut handler = InputHandler::new();
        handler
            .bind(Input::Test, KeyMod::SHIFT, Command::Test(0))
            .bind(Input::Test, KeyMod::NONE, Command::Test(1))
            .bind(Input::Test, KeyMod::SHIFT | KeyMod::CTRL, Command::Test(2));
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::NONE),
            Some(Command::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::SHIFT),
            Some(Command::Test(0))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT),
            Some(Command::Test(2))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL),
            Some(Command::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT | KeyMod::ALT),
            Some(Command::Test(2))
        );

        let mut handler = InputHandler::new();
        handler
            .bind(Input::Test, KeyMod::SHIFT | KeyMod::CTRL, Command::Test(2))
            .bind(Input::Test, KeyMod::SHIFT, Command::Test(0))
            .bind(Input::Test, KeyMod::NONE, Command::Test(1));
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::NONE),
            Some(Command::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::SHIFT),
            Some(Command::Test(0))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT),
            Some(Command::Test(2))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL),
            Some(Command::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT | KeyMod::ALT),
            Some(Command::Test(2))
        );
    }
}
