use std::collections::{HashMap, VecDeque};

use ggez::event::{KeyCode, MouseButton};
use ggez::{Context, GameResult};

use types::{Direction, KeyMod};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Input {
    Key(KeyCode),
    Mouse(MouseButton),
    #[cfg(test)]
    Test,
}

#[derive(Debug)]
pub enum InputValue {
    Key(bool),
    XY(f32, f32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Action {
    App(Control),
    Direction(Direction),
    #[cfg(test)]
    Test(usize),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Control {
    Exit,
    Pause,
}

type Bindings = HashMap<Input, Vec<(KeyMod, Action)>>;

pub struct InputHandler {
    bindings: Bindings,
}

impl Default for InputHandler {
    fn default() -> InputHandler {
        let mut handler = InputHandler::new();
        handler.bind(
            Input::Key(KeyCode::Q),
            KeyMod::CTRL | KeyMod::ALT,
            Action::App(Control::Exit),
        );
        handler
    }
}

impl InputHandler {
    pub fn new() -> InputHandler {
        let mut handler = InputHandler {
            bindings: Bindings::new(),
        };
        handler
    }

    pub fn bind(&mut self, input: Input, keymods: KeyMod, action: Action) -> &mut InputHandler {
        {
            let mut done = false;
            for mut bound_actions in self.bindings.entry(input).or_insert_with(Vec::new) {
                if bound_actions.0 == keymods {
                    bound_actions.1 = action;
                    done = true;
                    break;
                }
            }
            if !done {
                let bound_actions_bunch = self.bindings.entry(input).or_insert_with(Vec::new);
                let count = keymods.count();
                let mut index = 0;
                for bound_logicals in bound_actions_bunch.iter() {
                    if count >= bound_logicals.0.count() {
                        break;
                    }
                    index += 1;
                }
                bound_actions_bunch.insert(index, (keymods, action));
            }
        }
        self
    }

    pub fn resolve(&self, input: Input, keymods: KeyMod) -> Option<Action> {
        if let Some(bound_actions_bunch) = self.bindings.get(&input) {
            for bound_actions in bound_actions_bunch {
                if keymods.contains(bound_actions.0) {
                    return Some(bound_actions.1);
                }
            }
        }
        None
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMod,
        repeat: bool,
    ) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_resolution() {
        let mut handler = InputHandler::new();
        handler
            .bind(Input::Test, KeyMod::SHIFT, Action::Test(0))
            .bind(Input::Test, KeyMod::NONE, Action::Test(1))
            .bind(Input::Test, KeyMod::SHIFT | KeyMod::CTRL, Action::Test(2));
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::NONE),
            Some(Action::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::SHIFT),
            Some(Action::Test(0))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT),
            Some(Action::Test(2))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL),
            Some(Action::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT | KeyMod::ALT),
            Some(Action::Test(2))
        );

        let mut handler = InputHandler::new();
        handler
            .bind(Input::Test, KeyMod::SHIFT | KeyMod::CTRL, Action::Test(2))
            .bind(Input::Test, KeyMod::SHIFT, Action::Test(0))
            .bind(Input::Test, KeyMod::NONE, Action::Test(1));
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::NONE),
            Some(Action::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::SHIFT),
            Some(Action::Test(0))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT),
            Some(Action::Test(2))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL),
            Some(Action::Test(1))
        );
        assert_eq!(
            handler.resolve(Input::Test, KeyMod::CTRL | KeyMod::SHIFT | KeyMod::ALT),
            Some(Action::Test(2))
        );
    }
}
