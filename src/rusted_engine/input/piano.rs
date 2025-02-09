use std::collections::VecDeque;
use glfw::Key;
use std::sync::{Arc, RwLock};
use crate::rusted_engine::audio::audio_manager::{AudioManager, AudioType}; 
use crate::rusted_engine::input::key_states::KeyStates; 

pub struct Piano {
    audio_manager: Arc<RwLock<AudioManager>>,
    key_states: Arc<RwLock<KeyStates>>,
    octave: i32,
    note_history: VecDeque<String>, // Stores the last 100 played notes
}

impl Piano {
    pub fn new(audio_manager: Arc<RwLock<AudioManager>>, key_states: Arc<RwLock<KeyStates>>) -> Self {
        Self {
            audio_manager,
            key_states,
            octave: 4,
            note_history: VecDeque::with_capacity(100), // Preallocate space for 100 notes
        }
    }

    /// Returns true if there was any piano key newly pressed
    pub fn process_piano_keys(&mut self) -> bool {
        let mut note: String;
        note = "".to_owned();
        // Scope this so we can record the note at the end
        {
            let audio_manager_write = self.audio_manager.write().unwrap();
            let key_states_read = self.key_states.read().unwrap();
            
            let shift_held = key_states_read.is_key_pressed_raw(Key::LeftShift) || key_states_read.is_key_pressed_raw(Key::RightShift);

            let key_to_note = [
                (Key::Kp1, "A"), (Key::Kp2, "B"), (Key::Kp3, "C"),
                (Key::Kp4, "D"), (Key::Kp6, "E"),
                (Key::Kp7, "F"), (Key::Kp8, "G"), (Key::Kp9, "AH"),
            ];
        
            for &(key, note_base) in key_to_note.iter() {
                if key_states_read.is_key_pressed(key) {
                    note = if shift_held {
                        self.get_flat_note_for_octave(note_base)
                    } else {
                        self.get_note_for_octave(note_base)
                    };

                    audio_manager_write.enqueue_audio(&note, AudioType::Sound, 1.0, false);
                }
            }

            if key_states_read.is_key_pressed(Key::Kp0) {
                audio_manager_write.stop_audio();
                self.print_note_history();
            }
        
            if key_states_read.is_key_pressed(Key::KpAdd) && self.octave < 7 {
                self.octave += 1;
            }
            if key_states_read.is_key_pressed(Key::KpSubtract) && self.octave > 0 {
                self.octave -= 1;
            }
        }

        if note != "" {
            self.record_note(&note); // Store note in history
            return true;
        }
        return false;
    }

    fn record_note(&mut self, note: &str) {
        if self.note_history.len() >= 100 {
            self.note_history.pop_front(); // Remove the oldest note if history exceeds 100
        }
        self.note_history.push_back(note.to_string());
    }

    pub fn get_note_history(&self) -> Vec<String> {
        self.note_history.iter().cloned().collect()
    }

    /// Debug method to print the note history
    pub fn print_note_history(&self) {
        println!("Note History:");
        for (i, note) in self.note_history.iter().enumerate() {
            println!("{}: {}", i + 1, note);
        }
    }

    /// Checks if the note history contains a specific sequence of notes
    pub fn check_for_sequence_and_clear(&mut self, sequence: &[&str]) -> bool {
        let history: Vec<String> = self.note_history.iter().cloned().collect();

        // Check if the history contains the sequence
        for window in history.windows(sequence.len()) {
            if window.iter().map(|s| s.as_str()).eq(sequence.iter().map(|s| *s)) {
                self.note_history.clear(); // Clear history after finding the sequence
                return true;
            }
        }
        false
    }

    fn get_note_for_octave(&self, note_base: &str) -> String {
        match note_base {
            "A" => format!("A{}", self.octave),
            "B" => format!("B{}", self.octave),
            "C" => format!("C{}", self.octave+1),
            "D" => format!("D{}", self.octave+1),
            "E" => format!("E{}", self.octave+1),
            "F" => format!("F{}", self.octave+1),
            "G" => format!("G{}", self.octave+1),
            "AH" => format!("A{}", self.octave+1),
            _ => note_base.to_string(),
        }
    }

    fn get_flat_note_for_octave(&self, note_base: &str) -> String {
        match note_base {
            "A" => format!("Ab{}", self.octave),
            "B" => format!("Bb{}", self.octave),
            "C" => format!("B{}", self.octave),
            "D" => format!("Db{}", self.octave + 1),
            "E" => format!("Eb{}", self.octave + 1),
            "F" => format!("E{}", self.octave + 1),
            "G" => format!("Gb{}", self.octave + 1),
            "AH" => format!("Ab{}", self.octave+1),
            _ => note_base.to_string(),
        }
    }
}
