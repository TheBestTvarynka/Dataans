use common::key_bindings::{KeyBinding, KeyModifiers};
use web_sys::KeyboardEvent;

pub trait MatchKeyBinding {
    fn matches(&self, event: &KeyboardEvent) -> bool;
}

impl MatchKeyBinding for KeyBinding {
    fn matches(&self, event: &KeyboardEvent) -> bool {
        let modifiers = KeyModifiers {
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            alt: event.alt_key(),
            meta: event.meta_key(),
        };

        self.modifiers == modifiers && self.key == event.key()
    }
}
