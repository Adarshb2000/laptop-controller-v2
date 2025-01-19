use crate::input::get_gamepad_button_mapping;
use enigo::{Direction, Enigo, Keyboard, Mouse};
use gilrs::{Button, EventType};

pub fn handle_gamepad_input(event: EventType, enigo: &mut Enigo, remote_active: &mut bool) {
    if !*remote_active {
        if let EventType::ButtonPressed(Button::Start, _) = event {
            *remote_active = !*remote_active;
        }
        return;
    }

    match event {
        EventType::AxisChanged(_, _, _) => {}
        EventType::ButtonChanged(Button::South, val, _) => {
            if val == 1.0 {
                let _ = enigo.button(enigo::Button::Left, Direction::Press);
            } else {
                let _ = enigo.button(enigo::Button::Left, Direction::Release);
            }
        }
        EventType::ButtonPressed(Button::RightThumb, _) => {
            let _ = enigo.button(enigo::Button::Right, Direction::Click);
        }
        EventType::ButtonPressed(Button::Start, _) => {
            *remote_active = !*remote_active;
        }
        EventType::ButtonChanged(button, val, _) => {
            if let Some(btn) = get_gamepad_button_mapping(button) {
                if val == 1.0 {
                    let _ = enigo.key(btn, Direction::Press);
                } else {
                    let _ = enigo.key(btn, Direction::Release);
                }
            }
        }
        _ => {
            println!("{:?}", event);
        }
    }
}
