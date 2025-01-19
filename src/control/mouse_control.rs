use enigo::Enigo;
use gilrs::{Axis, Gamepad};

use crate::input::get_gamepad_axis_data;
use crate::mouse::{handle_mouse_move, handle_scroll};

pub fn handle_mouse_movement_control(gamepad: Gamepad, enigo: &mut Enigo) {
    let left_x_val = get_gamepad_axis_data(gamepad, Axis::LeftStickX);
    let left_y_val = get_gamepad_axis_data(gamepad, Axis::LeftStickY);
    handle_mouse_move(left_x_val, left_y_val, enigo);
}

pub fn handle_scroll_control(gamepad: Gamepad, enigo: &mut Enigo) {
    let right_x_val = get_gamepad_axis_data(gamepad, Axis::RightStickX);
    let right_y_val = get_gamepad_axis_data(gamepad, Axis::RightStickY);
    handle_scroll(right_x_val, right_y_val, enigo);
}
