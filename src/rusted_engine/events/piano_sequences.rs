use crate::rusted_engine::input::piano::Piano;

pub fn check_piano_sequences(piano: &mut Piano) {
    homebringer_sequence(piano);
}

fn homebringer_sequence(piano: &mut Piano) {
    let sequence = ["A4", "A4", "E5", "E5", "Db5", "Db5", "Ab5", "Ab5"];
    if piano.check_for_sequence_and_clear(&sequence) {
        println!("Found the sequence! History cleared.");
    } else {
        println!("Sequence not found.");
    }
}