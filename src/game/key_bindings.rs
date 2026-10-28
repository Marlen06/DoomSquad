use minifb::Key;

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

impl KeyBinding{
    pub fn new() -> Self{
        Self{
            forward: Key:: W,
            backward: Key:: S,
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
            jump: Key:: Space,
            big_jump: Key::R,
            current_enemies: Key::L,
            fire_arrow: Key::RightShift,
            toggle_fine_aiming: Key::E,
        }
    }
}
