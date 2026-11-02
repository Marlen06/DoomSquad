
use minifb::Key;
use serde:: {Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
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
        }
    }

    pub fn load() -> Self {
        let config = std::fs::read_to_string("key_bindings.ron");

        match config {
            Ok(contents) => {
                match ron::from_str::<KeyBindingsConfig>(&contents) {
                    Ok(config) => {
                        match config.to_key_binding() {
                            Some(bindings) => bindings,
                            None => Self::new()
                        }
                    }
                    Err(_) => Self::new()
                }
            }
            Err(_) => Self::new()
        }
    }

    pub fn save(&self) {
        let config = KeyBindingsConfig{
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
        };

        match ron:: ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()) {
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

fn key_to_string(key:Key) -> String {
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

        "Space" => Some(Key::Space),
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

        } )
        
    }
    
}