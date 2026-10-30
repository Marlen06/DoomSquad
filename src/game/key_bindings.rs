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
            godmode: Key::G,
            left: Key::A,
            right: Key::D,
            jump: Key::Space,
            big_jump: Key::R,
            current_enemies: Key::L,
            fire_arrow: Key::RightShift,
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
}

fn key_to_string(key:Key) -> String {
    format!("{:?}", key)
}

fn string_to_key(key: &str) -> Option<Key> {
    match key {
        "W" => Some(Key::W),
        "S" => Some(Key::S),
        "LeftShift" => Some(Key::LeftShift),
        "C" => Some(Key::C),
        "F" => Some(Key::F),
        "Down" => Some(Key::Down),
        "Up" => Some(Key::Up),
        "Right" => Some(Key::Right),
        "Left" => Some(Key::Left),
        "RightCtrl" => Some(Key::RightCtrl),
        "G" => Some(Key::G),
        "A" => Some(Key::A),
        "D" => Some(Key::D),
        "Space" => Some(Key::Space),
        "R" => Some(Key::R),
        "L" => Some(Key::L),
        "RightShift" => Some(Key::RightShift),
        "E" => Some(Key::E),
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
            godmode: string_to_key(&self.godmode)?,
            left: string_to_key(&self.left)?,
            right: string_to_key(&self.right)?,
            jump: string_to_key(&self.jump)?,
            big_jump: string_to_key(&self.big_jump)?,
            current_enemies: string_to_key(&self.current_enemies)?,
            fire_arrow: string_to_key(&self.fire_arrow)?,
            toggle_fine_aiming: string_to_key(&self.toggle_fine_aiming)?,

        } )
        
    }
    
}
/*später noch hinzufügen und checken + löschen
if window.is_key_pressed(key_bindings.aim_mode, KeyRepeat::No) {
    match self.aim_mode {
        true => self.aim_mode = false,
        false => self.aim_mode = true,
    }
}
    */
