use enigo::*;
use gilrs::{Button, Event, EventType, Gilrs};

// Mouse movement control
mod mouse;

// Gamepad
mod input;
use input::get_gamepad_button_mapping;

// Control
mod control;
use control::{handle_mouse_movement_control, handle_scroll_control};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mut remote_active = false;

    let poll_interval = std::time::Duration::from_millis(16); // 60fps

    loop {
        if !remote_active {
            if let Some(Event {
                id: _,
                event,
                time: _,
                ..
            }) = gilrs.next_event()
            {
                if let EventType::ButtonPressed(Button::Start, _) = event {
                    remote_active = !remote_active;
                }
            }

            std::thread::sleep(poll_interval);
            continue;
        }

        while let Some(Event {
            id: _,
            event,
            time: _,
            ..
        }) = gilrs.next_event()
        {
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
                    remote_active = !remote_active;
                    break;
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

        for (_, gamepad) in gilrs.gamepads() {
            if !gamepad.is_connected() {
                continue;
            }

            handle_mouse_movement_control(gamepad, &mut enigo);
            handle_scroll_control(gamepad, &mut enigo);
            break;
        }

        std::thread::sleep(poll_interval);
    }
}
