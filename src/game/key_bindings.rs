use minifb::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct KeyBindingsConfig {
    forward: String,
    backward: String,
    sprint: String,
    crouch: String,
    interact: String,
    crosshair_down: String,
    corsshair_up: String,
    turn_right: String,
    turn_left: String,
    fire_bullet: String,
    fire_arrow: String,
    godmode: String,
    left: String,
    right: String,
    jump: String,
    big_jump: String,
    current_enemies: String,
    toggle_fine_aiming: String,
    escape: String,
}

#[derive(Clone)]
pub struct KeyBinding {
    pub forward: Key,
    pub backward: Key,
    pub sprint: Key,
    pub crouch: Key,
    pub interact: Key,
    pub crosshair_down: Key,
    pub corsshair_up: Key,
    pub turn_right: Key,
    pub turn_left: Key,
    pub fire_bullet: Key,
    pub fire_arrow: Key,
    pub godmode: Key,
    pub left: Key,
    pub right: Key,
    pub jump: Key,
    pub big_jump: Key,
    pub current_enemies: Key,
    pub toggle_fine_aiming: Key,
    pub escape: Key,
}

impl KeyBinding {
    pub fn new() -> Self {
        Self {
            forward: Key::W,
            backward: Key::S,
            sprint: Key::LeftShift,
            crouch: Key::C,
            interact: Key::F,
            crosshair_down: Key::Down,
            corsshair_up: Key::Up,
            turn_right: Key::Right,
            turn_left: Key::Left,
            fire_bullet: Key::RightCtrl,
            fire_arrow: Key::RightShift,
            godmode: Key::G,
            left: Key::A,
            right: Key::D,
            jump: Key::Space,
            big_jump: Key::R,
            current_enemies: Key::L,
            toggle_fine_aiming: Key::E,
            escape: Key::Escape,
        }
    }

    pub fn load() -> Self {
        let config = std::fs::read_to_string("key_bindings.ron");

        match config {
            Ok(contents) => match ron::from_str::<KeyBindingsConfig>(&contents) {
                Ok(config) => match config.to_key_binding() {
                    Some(bindings) => bindings,
                    None => Self::new(),
                },
                Err(_) => Self::new(),
            },
            Err(_) => Self::new(),
        }
    }

    pub fn save(&self) {
        let config = KeyBindingsConfig {
            forward: key_to_string(self.forward),
            backward: key_to_string(self.backward),
            sprint: key_to_string(self.sprint),
            crouch: key_to_string(self.crouch),
            interact: key_to_string(self.interact),
            crosshair_down: key_to_string(self.crosshair_down),
            corsshair_up: key_to_string(self.corsshair_up),
            turn_right: key_to_string(self.turn_right),
            turn_left: key_to_string(self.turn_left),
            fire_bullet: key_to_string(self.fire_bullet),
            fire_arrow: key_to_string(self.fire_arrow),
            godmode: key_to_string(self.godmode),
            left: key_to_string(self.left),
            right: key_to_string(self.right),
            jump: key_to_string(self.jump),
            big_jump: key_to_string(self.big_jump),
            current_enemies: key_to_string(self.current_enemies),
            toggle_fine_aiming: key_to_string(self.toggle_fine_aiming),
            escape: key_to_string(self.escape),
        };

        match ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()) {
            Ok(contents) => {
                if let Err(error) = std::fs::write("key_bindings.ron", contents) {
                    eprintln!("Fehler bei Speichern der Tasten: {}", error);
                }
            }
            Err(error) => {
                eprintln!("Fehler beim Erstellen der RON-Konfiguration: {}", error)
            }
        }
    }
}

fn key_to_string(key: Key) -> String {
    format!("{:?}", key)
}

fn string_to_key(key: &str) -> Option<Key> {
    match key {
        // Buchstaben
        "A" => Some(Key::A),
        "B" => Some(Key::B),
        "C" => Some(Key::C),
        "D" => Some(Key::D),
        "E" => Some(Key::E),
        "F" => Some(Key::F),
        "G" => Some(Key::G),
        "H" => Some(Key::H),
        "I" => Some(Key::I),
        "J" => Some(Key::J),
        "K" => Some(Key::K),
        "L" => Some(Key::L),
        "M" => Some(Key::M),
        "N" => Some(Key::N),
        "O" => Some(Key::O),
        "P" => Some(Key::P),
        "Q" => Some(Key::Q),
        "R" => Some(Key::R),
        "S" => Some(Key::S),
        "T" => Some(Key::T),
        "U" => Some(Key::U),
        "V" => Some(Key::V),
        "W" => Some(Key::W),
        "X" => Some(Key::X),
        "Y" => Some(Key::Y),
        "Z" => Some(Key::Z),

        // Zahlenreihe
        "Key0" => Some(Key::Key0),
        "Key1" => Some(Key::Key1),
        "Key2" => Some(Key::Key2),
        "Key3" => Some(Key::Key3),
        "Key4" => Some(Key::Key4),
        "Key5" => Some(Key::Key5),
        "Key6" => Some(Key::Key6),
        "Key7" => Some(Key::Key7),
        "Key8" => Some(Key::Key8),
        "Key9" => Some(Key::Key9),

        // Funktionstasten
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "F13" => Some(Key::F13),
        "F14" => Some(Key::F14),
        "F15" => Some(Key::F15),

        // Pfeiltasten
        "Up" => Some(Key::Up),
        "Down" => Some(Key::Down),
        "Left" => Some(Key::Left),
        "Right" => Some(Key::Right),

        // Modifier-Tasten
        "LeftShift" => Some(Key::LeftShift),
        "RightShift" => Some(Key::RightShift),
        "LeftCtrl" => Some(Key::LeftCtrl),
        "RightCtrl" => Some(Key::RightCtrl),
        "LeftAlt" => Some(Key::LeftAlt),
        "RightAlt" => Some(Key::RightAlt),
        "LeftSuper" => Some(Key::LeftSuper),
        "RightSuper" => Some(Key::RightSuper),

        // Sonderzeichen
        "Apostrophe" => Some(Key::Apostrophe),
        "Backquote" => Some(Key::Backquote),
        "Backslash" => Some(Key::Backslash),
        "Comma" => Some(Key::Comma),
        "Equal" => Some(Key::Equal),
        "LeftBracket" => Some(Key::LeftBracket),
        "Minus" => Some(Key::Minus),
        "Period" => Some(Key::Period),
        "RightBracket" => Some(Key::RightBracket),
        "Semicolon" => Some(Key::Semicolon),
        "Slash" => Some(Key::Slash),

        "Backspace" => Some(Key::Backspace),
        "Delete" => Some(Key::Delete),
        "End" => Some(Key::End),
        "Enter" => Some(Key::Enter),
        "Escape" => Some(Key::Escape),
        "Home" => Some(Key::Home),
        "Insert" => Some(Key::Insert),
        "Menu" => Some(Key::Menu),
        "PageDown" => Some(Key::PageDown),
        "PageUp" => Some(Key::PageUp),
        "Pause" => Some(Key::Pause),
        "Space" => Some(Key::Space),
        "Tab" => Some(Key::Tab),

        // Nummernblock
        "NumPad0" => Some(Key::NumPad0),
        "NumPad1" => Some(Key::NumPad1),
        "NumPad2" => Some(Key::NumPad2),
        "NumPad3" => Some(Key::NumPad3),
        "NumPad4" => Some(Key::NumPad4),
        "NumPad5" => Some(Key::NumPad5),
        "NumPad6" => Some(Key::NumPad6),
        "NumPad7" => Some(Key::NumPad7),
        "NumPad8" => Some(Key::NumPad8),
        "NumPad9" => Some(Key::NumPad9),
        "NumPadDot" => Some(Key::NumPadDot),
        "NumPadSlash" => Some(Key::NumPadSlash),
        "NumPadAsterisk" => Some(Key::NumPadAsterisk),
        "NumPadMinus" => Some(Key::NumPadMinus),
        "NumPadPlus" => Some(Key::NumPadPlus),
        "NumPadEnter" => Some(Key::NumPadEnter),

        _ => None,
    }
}

impl KeyBindingsConfig {
    fn to_key_binding(&self) -> Option<KeyBinding> {
        Some(KeyBinding {
            forward: string_to_key(&self.forward)?,
            backward: string_to_key(&self.backward)?,
            sprint: string_to_key(&self.sprint)?,
            crouch: string_to_key(&self.crouch)?,
            interact: string_to_key(&self.interact)?,
            crosshair_down: string_to_key(&self.crosshair_down)?,
            corsshair_up: string_to_key(&self.corsshair_up)?,
            turn_right: string_to_key(&self.turn_right)?,
            turn_left: string_to_key(&self.turn_left)?,
            fire_bullet: string_to_key(&self.fire_bullet)?,
            fire_arrow: string_to_key(&self.fire_arrow)?,
            godmode: string_to_key(&self.godmode)?,
            left: string_to_key(&self.left)?,
            right: string_to_key(&self.right)?,
            jump: string_to_key(&self.jump)?,
            big_jump: string_to_key(&self.big_jump)?,
            current_enemies: string_to_key(&self.current_enemies)?,
            toggle_fine_aiming: string_to_key(&self.toggle_fine_aiming)?,
            escape: string_to_key(&self.escape)?,
        })
    }
}
