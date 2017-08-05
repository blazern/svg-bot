#[macro_use]
extern crate assert_approx_eq;
extern crate svg_bot;

use svg_bot::rect::Rect;
use svg_bot::point::SvgPoint;
use svg_bot::point::ScreenPoint;

#[test]
fn svg_point_constructs_correctly() {
    let x = 1f32;
    let y = 3f32;
    let svg_area = Rect::new(2f32, 4f32, 5f32, 6f32);
    let screen_area = Rect::new(6f32, 5f32, 4f32, 3f32);

    let point = SvgPoint::new(x, y, &svg_area, &screen_area);

    assert_eq!(x, point.x());
    assert_eq!(y, point.y());
    assert_eq!(svg_area, *point.svg_area());
    assert_eq!(screen_area, *point.screen_area());
}

#[test]
fn screen_point_constructs_correctly() {
    let x = 1f32;
    let y = 3f32;
    let svg_area = Rect::new(2f32, 4f32, 5f32, 6f32);
    let screen_area = Rect::new(6f32, 5f32, 4f32, 3f32);

    let point = ScreenPoint::new(x, y, &svg_area, &screen_area);

    assert_eq!(x, point.x());
    assert_eq!(y, point.y());
    assert_eq!(svg_area, *point.svg_area());
    assert_eq!(screen_area, *point.screen_area());
}

#[test]
fn svg_point_constructed_from_screen_point_has_different_coords() {
    let x = 1f32;
    let y = 3f32;
    let svg_area = Rect::new(2f32, 4f32, 5f32, 6f32);
    let screen_area = Rect::new(6f32, 5f32, 4f32, 3f32);

    let svg_point = ScreenPoint::new(x, y, &svg_area, &screen_area);
    let screen_point = SvgPoint::from(svg_point.clone());

    assert_ne!(svg_point.x(), screen_point.x());
    assert_ne!(svg_point.y(), screen_point.y());
}

#[test]
fn screen_point_constructed_from_svg_point_has_different_coords() {
    let x = 1f32;
    let y = 3f32;
    let svg_area = Rect::new(2f32, 4f32, 5f32, 6f32);
    let screen_area = Rect::new(6f32, 5f32, 4f32, 3f32);

    let svg_point = SvgPoint::new(x, y, &svg_area, &screen_area);
    let screen_point = ScreenPoint::from(svg_point.clone());

    assert_ne!(svg_point.x(), screen_point.x());
    assert_ne!(svg_point.y(), screen_point.y());
}

#[test]
fn svg_point_constructed_from_screen_point_has_same_areas() {
    let x = 1f32;
    let y = 3f32;
    let svg_area = Rect::new(2f32, 4f32, 5f32, 6f32);
    let screen_area = Rect::new(6f32, 5f32, 4f32, 3f32);

    let svg_point = ScreenPoint::new(x, y, &svg_area, &screen_area);
    let screen_point = SvgPoint::from(svg_point.clone());

    assert_eq!(*svg_point.svg_area(), *screen_point.svg_area());
    assert_eq!(*svg_point.screen_area(), *screen_point.screen_area());
}

#[test]
fn screen_point_constructed_from_svg_point_has_same_areas() {
    let x = 1f32;
    let y = 3f32;
    let svg_area = Rect::new(2f32, 4f32, 5f32, 6f32);
    let screen_area = Rect::new(6f32, 5f32, 4f32, 3f32);

    let svg_point = SvgPoint::new(x, y, &svg_area, &screen_area);
    let screen_point = ScreenPoint::from(svg_point.clone());

    assert_eq!(*svg_point.svg_area(), *screen_point.svg_area());
    assert_eq!(*svg_point.screen_area(), *screen_point.screen_area());
}

#[test]
fn points_recalculation_works() {
    let x = 1f32;
    let y = 3f32;
    let svg_area = Rect::new(2f32, 4f32, 5f32, 6f32);
    let screen_area = Rect::new(6f32, 5f32, 4f32, 3f32);

    let svg_point = SvgPoint::new(x, y, &svg_area, &screen_area);
    let screen_point = ScreenPoint::from(svg_point.clone());
    let recalculated_svg_point = SvgPoint::from(screen_point.clone());

    assert_approx_eq!(svg_point.x(), recalculated_svg_point.x());
    assert_approx_eq!(svg_point.y(), recalculated_svg_point.y());
}

#[test]
fn can_offset_svg_point() {
    let point = SvgPoint::new(1f32, 3f32, &Rect::new(2f32, 4f32, 5f32, 6f32), &Rect::new(6f32, 5f32, 4f32, 3f32));

    let offset_x = 123f32;
    let offset_y = 321f32;
    let moved_point = point.clone().offset(offset_x, offset_y);
    assert_approx_eq!(moved_point.x(), point.x() + offset_x);
    assert_approx_eq!(moved_point.y(), point.y() + offset_y);
}

#[test]
fn can_offset_screen_point() {
    let point = ScreenPoint::new(1f32, 3f32, &Rect::new(2f32, 4f32, 5f32, 6f32), &Rect::new(6f32, 5f32, 4f32, 3f32));

    let offset_x = 123f32;
    let offset_y = 321f32;
    let moved_point = point.clone().offset(offset_x, offset_y);
    assert_approx_eq!(moved_point.x(), point.x() + offset_x);
    assert_approx_eq!(moved_point.y(), point.y() + offset_y);
}