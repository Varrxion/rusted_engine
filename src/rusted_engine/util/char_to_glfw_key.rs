use glfw::Key;

pub fn char_to_glfw_key(c: char) -> Option<Key> {
    match c {
        'a' | 'A' => Some(Key::A),
        'b' | 'B' => Some(Key::B),
        'c' | 'C' => Some(Key::C),
        'd' | 'D' => Some(Key::D),
        'e' | 'E' => Some(Key::E),
        'f' | 'F' => Some(Key::F),
        'g' | 'G' => Some(Key::G),
        'h' | 'H' => Some(Key::H),
        'i' | 'I' => Some(Key::I),
        'j' | 'J' => Some(Key::J),
        'k' | 'K' => Some(Key::K),
        'l' | 'L' => Some(Key::L),
        'm' | 'M' => Some(Key::M),
        'n' | 'N' => Some(Key::N),
        'o' | 'O' => Some(Key::O),
        'p' | 'P' => Some(Key::P),
        'q' | 'Q' => Some(Key::Q),
        'r' | 'R' => Some(Key::R),
        's' | 'S' => Some(Key::S),
        't' | 'T' => Some(Key::T),
        'u' | 'U' => Some(Key::U),
        'v' | 'V' => Some(Key::V),
        'w' | 'W' => Some(Key::W),
        'x' | 'X' => Some(Key::X),
        'y' | 'Y' => Some(Key::Y),
        'z' | 'Z' => Some(Key::Z),

        '0' => Some(Key::Num0),
        '1' => Some(Key::Num1),
        '2' => Some(Key::Num2),
        '3' => Some(Key::Num3),
        '4' => Some(Key::Num4),
        '5' => Some(Key::Num5),
        '6' => Some(Key::Num6),
        '7' => Some(Key::Num7),
        '8' => Some(Key::Num8),
        '9' => Some(Key::Num9),

        '`' => Some(Key::GraveAccent),
        '-' => Some(Key::Minus),
        '=' => Some(Key::Equal),
        '[' => Some(Key::LeftBracket),
        ']' => Some(Key::RightBracket),
        '\\' => Some(Key::Backslash),
        ';' => Some(Key::Semicolon),
        '\'' => Some(Key::Apostrophe),
        ',' => Some(Key::Comma),
        '.' => Some(Key::Period),
        '/' => Some(Key::Slash),

        ' ' => Some(Key::Space),

        _ => None, // Return None for unsupported characters
    }
}
