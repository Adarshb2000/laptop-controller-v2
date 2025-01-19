use enigo::Key;
use gilrs::Button;

pub fn get_gamepad_button_mapping(button: Button) -> Option<Key> {
    match button {
        Button::DPadDown => Some(Key::DownArrow),
        Button::DPadUp => Some(Key::UpArrow),
        Button::DPadLeft => Some(Key::LeftArrow),
        Button::DPadRight => Some(Key::RightArrow),
        Button::East => Some(Key::Escape),
        Button::RightTrigger2 => Some(Key::Meta),
        Button::RightTrigger => Some(Key::Alt),
        Button::LeftTrigger => Some(Key::Control),
        Button::LeftThumb => Some(Key::Tab),
        _ => None,
    }
}
