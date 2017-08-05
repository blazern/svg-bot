#[macro_use]
extern crate assert_approx_eq;
extern crate svg_bot;

#[test]
fn can_get_mouse_coords() {
    let (_, _): (f32, f32) = svg_bot::mouse::coords().unwrap();
}

#[test]
fn can_perform_mouse_down() {
    svg_bot::mouse::down().unwrap();
}

#[test]
fn can_perform_mouse_up() {
    svg_bot::mouse::up().unwrap();
}

#[test]
fn can_move_mouse() {
    let (initial_x, initial_y) = svg_bot::mouse::coords().unwrap();

    let destination_x: f32;
    let destination_y: f32;
    match (initial_x, initial_y) {
        (0f32, 0f32) => {
            destination_x = 100f32;
            destination_y = 100f32;
        }
        (_, _) => {
            destination_x = 0f32;
            destination_y = 0f32;
        }
    }

    svg_bot::mouse::move_to(destination_x, destination_y).unwrap();
    let (final_x, final_y) = svg_bot::mouse::coords().unwrap();

    assert_approx_eq!(destination_x, final_x);
    assert_approx_eq!(destination_y, final_y);
}
