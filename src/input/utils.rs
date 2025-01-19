use gilrs::{Axis, Gamepad};

pub fn get_gamepad_axis_data(gamepad: Gamepad, axis: Axis) -> f32 {
    return gamepad.axis_data(axis).map_or(0.0, |data| data.value());
}
