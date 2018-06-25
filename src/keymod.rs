use ggez::event::KeyMods;

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
}
