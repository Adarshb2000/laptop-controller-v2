use enigo::{Enigo, Mouse};

pub fn handle_scroll(x_axis_val: f32, y_axis_val: f32, enigo: &mut Enigo) {
    let threshold = 0.2;
    let scroll_axis = if x_axis_val.abs() > y_axis_val.abs() {
        enigo::Axis::Horizontal
    } else {
        enigo::Axis::Vertical
    };

    let scroll_axis_val = if scroll_axis == enigo::Axis::Horizontal {
        x_axis_val
    } else {
        y_axis_val
    };

    let scroll = if scroll_axis_val.abs() > threshold {
        scroll_axis_val.signum() as i32
    } else {
        0
    };
    let scroll_direction = if scroll_axis == enigo::Axis::Vertical {
        -1
    } else {
        1
    };
    if scroll != 0 {
        let _ = enigo.scroll(scroll * scroll_direction, scroll_axis);
    }
}
