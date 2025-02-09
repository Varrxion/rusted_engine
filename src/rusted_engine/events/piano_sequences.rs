use crate::rusted_engine::input::piano::Piano;

use super::event_handler::EventHandler;

pub fn check_piano_sequences(piano: &mut Piano, event_handler: &EventHandler) {
    homebringer_sequence(piano, event_handler);
    gorbino_sequence(piano, event_handler);
    explosion_sequence(piano, event_handler);
}

fn homebringer_sequence(piano: &mut Piano, event_handler: &EventHandler) {
    let sequence = ["A4", "A4", "E5", "E5", "Db5", "Db5", "Ab5", "Ab5"];
    if piano.check_for_sequence_and_clear(&sequence) {
        println!("Found the Homebringer sequence! History cleared.");
        event_handler.homebringer_sequence();
    }
}

fn gorbino_sequence(piano: &mut Piano, event_handler: &EventHandler) {
    let sequence = ["G5", "A5", "E5", "C5", "B4", "A4", "D5", "F5"];
    if piano.check_for_sequence_and_clear(&sequence) {
        println!("Found the Gorbino sequence! History cleared.");
        event_handler.gorbino_sequence();
    }
}

fn explosion_sequence(piano: &mut Piano, event_handler: &EventHandler) {
    let sequence = ["F5", "A5", "A4", "C5", "G5", "D5", "E5", "B4"];
    if piano.check_for_sequence_and_clear(&sequence) {
        println!("Found the Explosion sequence! History cleared.");
        event_handler.explosion_sequence();
    }
}