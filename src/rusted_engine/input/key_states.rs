use glfw::{Action, Key, WindowEvent};
use std::collections::HashMap;

// Define the states a key can be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyState {
    Pressed,
    Held,
    Released,
}

pub struct KeyStates {
    keys: HashMap<Key, KeyState>, // Track the state of each key
}

impl KeyStates {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    // Update all pressed keys to held keys (before processing new press and release events)
    pub fn update_pressed_to_held(&mut self) {
        let pressed_keys: Vec<Key> = self
            .keys
            .iter()
            .filter(|(_, &state)| state == KeyState::Pressed)
            .map(|(&key, _)| key)
            .collect();

        for key in pressed_keys {
            self.keys.insert(key, KeyState::Held);
        }
    }

    // Handle window events to update the key states
    pub fn handle_key_event(&mut self, event: WindowEvent) {
        if let WindowEvent::Key(key, _, action, _) = event {
            match action {
                Action::Press => {
                    // Mark the key as pressed immediately
                    self.keys.insert(key, KeyState::Pressed);
                }
                Action::Release => {
                    // Mark the key as released
                    self.keys.insert(key, KeyState::Released);
                }
                _ => {}
            }
        }
    }

    // Returns true if the key was just pressed (not held from the previous frame)
    pub fn is_key_pressed(&self, key: Key) -> bool {
        match self.keys.get(&key) {
            Some(KeyState::Pressed) => true,
            _ => false,
        }
    }

    // Returns true if the key is pressed (either newly pressed or held)
    pub fn is_key_pressed_raw(&self, key: Key) -> bool {
        match self.keys.get(&key) {
            Some(KeyState::Pressed) | Some(KeyState::Held) => true,
            _ => false,
        }
    }
}
