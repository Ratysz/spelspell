use std::cmp::{Ord, Ordering};

use ggez::event::KeyMods;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    U,
    D,
    None,
}

bitflags! {
    #[derive(Default)]
    pub struct KeyMod: u8 {
        const NONE  = 0b00000000;
        const SHIFT = 0b00000001;
        const CTRL  = 0b00000010;
        const ALT   = 0b00000100;
        const LOGO  = 0b00001000;
    }
}

impl KeyMod {
    /// Amount of flags set (Kernighan/Wegner/Lehmer method).
    pub fn count(&self) -> u8 {
        let mut num_set = 0;
        let mut bits = self.bits();
        loop {
            if num_set >= bits {
                break;
            }
            bits &= bits - 1;
            num_set += 1;
        }
        num_set
    }
}

impl From<KeyMods> for KeyMod {
    fn from(keymods: KeyMods) -> Self {
        let mut keymod = KeyMod::empty();
        if keymods.shift {
            keymod |= Self::SHIFT;
        }
        if keymods.ctrl {
            keymod |= Self::CTRL;
        }
        if keymods.alt {
            keymod |= Self::ALT;
        }
        if keymods.logo {
            keymod |= Self::LOGO;
        }
        keymod
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_mod_conversions() {
        assert_eq!(
            KeyMod::empty(),
            KeyMod::from(KeyMods {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false,
            })
        );
        assert_eq!(
            KeyMod::SHIFT,
            KeyMod::from(KeyMods {
                shift: true,
                ctrl: false,
                alt: false,
                logo: false,
            })
        );
        assert_eq!(
            KeyMod::SHIFT | KeyMod::ALT,
            KeyMod::from(KeyMods {
                shift: true,
                ctrl: false,
                alt: true,
                logo: false,
            })
        );
        assert_eq!(
            KeyMod::SHIFT | KeyMod::ALT | KeyMod::CTRL,
            KeyMod::from(KeyMods {
                shift: true,
                ctrl: true,
                alt: true,
                logo: false,
            })
        );
        assert_eq!(
            KeyMod::SHIFT - KeyMod::ALT,
            KeyMod::from(KeyMods {
                shift: true,
                ctrl: false,
                alt: false,
                logo: false,
            })
        );
        assert_eq!(
            (KeyMod::SHIFT | KeyMod::ALT) - KeyMod::ALT,
            KeyMod::from(KeyMods {
                shift: true,
                ctrl: false,
                alt: false,
                logo: false,
            })
        );
        assert_eq!(
            KeyMod::SHIFT - (KeyMod::ALT | KeyMod::SHIFT),
            KeyMod::from(KeyMods {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false,
            })
        );
    }

    #[test]
    fn key_mod_set_bit_count() {
        assert_eq!(KeyMod::empty().count(), 0);
        assert_eq!(KeyMod::SHIFT.count(), 1);
        assert_eq!(KeyMod::CTRL.count(), 1);
        assert_eq!(KeyMod::ALT.count(), 1);
        assert_eq!(KeyMod::LOGO.count(), 1);
        assert_eq!((KeyMod::SHIFT | KeyMod::CTRL).count(), 2);
        assert_eq!((KeyMod::SHIFT | KeyMod::LOGO).count(), 2);
        assert_eq!((KeyMod::LOGO | KeyMod::SHIFT).count(), 2);
        assert_eq!((KeyMod::LOGO | KeyMod::SHIFT | KeyMod::ALT).count(), 3);
        assert_eq!((!KeyMod::ALT).count(), 3);
    }
}
