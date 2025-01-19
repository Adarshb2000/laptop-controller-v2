use enigo::{Coordinate, Enigo, Mouse};

pub fn calculate_mouse_movement_value(axis_val: f32) -> i32 {
    if axis_val == 0.0 {
        return 0;
    };
    let rounded_val = axis_val.abs().exp() as i32;
    let movement = rounded_val.pow(2);
    if axis_val.abs() < 0.99 {
        movement
    } else {
        movement * 8
    }
}

pub fn handle_mouse_move(x_axis_val: f32, y_axis_val: f32, enigo: &mut Enigo) {
    let x_direction = if x_axis_val > 0.0 { 1 } else { -1 };
    let y_direction = if y_axis_val > 0.0 { -1 } else { 1 };
    let x = calculate_mouse_movement_value(x_axis_val);
    let y = calculate_mouse_movement_value(y_axis_val);

    if x == 0 && y == 0 {
        return;
    }

    let _ = enigo.move_mouse(x * x_direction, y * y_direction, Coordinate::Rel);
}
