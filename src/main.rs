use enigo::*;
use gilrs::{Event, Gilrs};

// Mouse movement control
mod mouse;

// Gamepad
mod input;
use input::handle_gamepad_input;

// Control
mod control;
use control::{handle_mouse_movement_control, handle_scroll_control};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mut remote_active = false;

    let poll_interval = std::time::Duration::from_millis(16); // 60fps

    loop {
        if let Some(Event { event, .. }) = gilrs.next_event() {
            handle_gamepad_input(event, &mut enigo, &mut remote_active);
        }

        if remote_active {
            for (_, gamepad) in gilrs.gamepads() {
                if !gamepad.is_connected() {
                    continue;
                }

                handle_mouse_movement_control(gamepad, &mut enigo);
                handle_scroll_control(gamepad, &mut enigo);
                break;
            }
        }

        std::thread::sleep(poll_interval);
    }
}
