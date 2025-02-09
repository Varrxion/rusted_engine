use glfw::Key;
use std::sync::{Arc, RwLock};
use crate::rusted_engine::audio::audio_manager::{AudioManager, AudioType}; // Assuming AudioManager is in a module named audio_manager
use crate::rusted_engine::input::key_states::KeyStates; // Assuming KeyStates is in a module named key_states

// Piano struct to encapsulate functionality
pub struct Piano {
    audio_manager: Arc<RwLock<AudioManager>>, // Reference to AudioManager to enqueue sounds
    key_states: Arc<RwLock<KeyStates>>, // Reference to KeyStates to track key presses
    octave: i32, // Track the current octave (default is 4)
}

impl Piano {
    // Constructor to create a new Piano instance
    pub fn new(audio_manager: Arc<RwLock<AudioManager>>, key_states: Arc<RwLock<KeyStates>>) -> Self {
        Self {
            audio_manager,
            key_states,
            octave: 4, // Default octave is 4 (A4)
        }
    }

    // Process the piano keys based on the numpad keys
    pub fn process_piano_keys(&mut self) {
        let audio_manager_write = self.audio_manager.write().unwrap();
        let key_states_read = self.key_states.read().unwrap();
    
        // Define key to note mappings for the current octave
        let key_to_note = [
            (Key::Kp1, "A4"), (Key::Kp2, "B4"), (Key::Kp3, "C5"),
            (Key::Kp4, "D5"), (Key::Kp5, "gorbino"), (Key::Kp6, "E5"),
            (Key::Kp7, "F5"), (Key::Kp8, "G5"), (Key::Kp9, "A5"),
        ];
    
        // Loop through the key-to-note mappings and check if any key is pressed
        for &(key, note_base) in key_to_note.iter() {
            if key_states_read.is_key_pressed(key) {
                let note = self.get_note_for_octave(note_base);
                audio_manager_write.enqueue_audio(&note, AudioType::Sound, 1.0, false);
            }
        }
    
        // Special key for stopping audio
        if key_states_read.is_key_pressed(Key::Kp0) {
            audio_manager_write.stop_audio();
        }
    
        // Check if the player pressed Numpad '+' or Numpad '-'
        if key_states_read.is_key_pressed(Key::KpAdd) {
            if self.octave < 7 {
                self.octave += 1; // Increase the octave if it's below the max (7)
            }
        }
        if key_states_read.is_key_pressed(Key::KpSubtract) {
            if self.octave > 0 {
                self.octave -= 1; // Decrease the octave if it's above the min (0)
            }
        }
    }
    

    // Helper function to get the note name based on the current octave
    fn get_note_for_octave(&self, note_base: &str) -> String {
        let note_with_octave = match note_base {
            "A4" => format!("A{}", self.octave),
            "B4" => format!("B{}", self.octave),
            "C5" => format!("C{}", self.octave+1),
            "D5" => format!("D{}", self.octave+1),
            "E5" => format!("E{}", self.octave+1),
            "F5" => format!("F{}", self.octave+1),
            "G5" => format!("G{}", self.octave+1),
            "A5" => format!("A{}", self.octave+1),
            _ => note_base.to_string(), // Fallback if note is not in the list
        };
        note_with_octave
    }
}
