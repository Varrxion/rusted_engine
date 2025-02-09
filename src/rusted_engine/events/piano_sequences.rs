use crate::rusted_engine::input::piano::Piano;

use super::event_handler::EventHandler;

pub fn check_piano_sequences(piano: &mut Piano, event_handler: &EventHandler) {
    homebringer_sequence(piano, event_handler);
}

fn homebringer_sequence(piano: &mut Piano, event_handler: &EventHandler) {
    let sequence = ["A4", "A4", "E5", "E5", "Db5", "Db5", "Ab5", "Ab5"];
    if piano.check_for_sequence_and_clear(&sequence) {
        println!("Found the sequence! History cleared.");
        event_handler.homebringer_sequence();
    } else {
        println!("Sequence not found.");
    }
}