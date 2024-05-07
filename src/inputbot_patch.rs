/// This file is currently only here for me to be able to set custom delays for KeySequence.send
/// Goal is to remove this when GitHub issue is resolved
/// https://github.com/obv-mikhail/InputBot/issues/101
use std::{thread::sleep, time::Duration};

use inputbot::{get_keybd_key, KeybdKey};

/// custom version of inputbot::KeySequence -- send has delay parameter
pub struct KeySequence<'a>(pub &'a str);

impl KeySequence<'_> {
    pub fn send(&self, delay: Duration) {
        for c in self.0.chars() {
            let mut uppercase = false;

            if let Some(keybd_key) = {
                if c.is_uppercase()
                    || [
                        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|',
                        ':', '"', '<', '>', '?', '~',
                    ]
                    .contains(&c)
                {
                    uppercase = true;
                }

                get_keybd_key(c)
            } {
                if uppercase {
                    KeybdKey::LShiftKey.press();
                }

                keybd_key.press();
                sleep(delay);
                keybd_key.release();

                if uppercase {
                    KeybdKey::LShiftKey.release();
                }
            };
        }
    }
}
