extern crate svg_bot;

use svg_bot::rect::Rect;

#[test]
fn rect_constructs_correctly() {
    let x = 123f32;
    let y = 321f32;
    let width = 10f32;
    let height = 20f32;
    let rect = Rect::new(x, y, width, height);

    assert_eq!(x, rect.x());
    assert_eq!(y, rect.y());
    assert_eq!(width, rect.width());
    assert_eq!(height, rect.height());
}
