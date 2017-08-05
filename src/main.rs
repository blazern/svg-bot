extern crate svg;

use svg::node::element::path::{Command, Data, Position, Parameters};
use svg::parser::Event;
use std::time::Duration;
use std::thread;

mod point;
mod rect;

use point::SvgPoint;
use point::ScreenPoint;
use rect::Rect;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    println!("path: {}", path);

    let (top_left_x, top_left_y) = ask_screen_coord("Top left");
    let (bottom_right_x, bottom_right_y) = ask_screen_coord("Bottom right");
    let paint_area = Rect::new(top_left_x, top_left_y, bottom_right_x - top_left_x, bottom_right_y - top_left_y);

    assert!(top_left_x < bottom_right_x);
    assert!(top_left_y < bottom_right_y);

    let svg_area = get_svg_area(path);
    println!("SVG's width: {}, height: {}", svg_area.width(), svg_area.height());
    
    for event in svg::open(path).unwrap() {
        match event {
            Event::Tag(svg::node::element::tag::Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                let mut current_point: Option<SvgPoint> = None;
                for command in data.iter() {
                    match command {
                        &Command::Move(ref position_type, ref params) => {
                            let result_point = perform_move(&position_type, &params.to_vec(), &svg_area, &paint_area);
                            current_point = Some(result_point);
                        },
                        &Command::Line(ref position_type, ref params) => {
                            let result_point = perform_line(&position_type, &params.to_vec(), &svg_area, &paint_area);
                            current_point = Some(result_point);
                        },
                        &Command::CubicCurve(ref position_type, ref params) => {
                            let result_point = perform_cubic_curve(&current_point.as_ref().unwrap(), &position_type, &params.to_vec(), &svg_area, &paint_area);
                            current_point = Some(result_point);
                        },
                        &Command::QuadraticCurve(_, ref params) => print_params("QuadraticCurve", &params),
                        &Command::HorizontalLine(_, ref params) => print_params("HorizontalLine", &params),
                        &Command::VerticalLine(_, ref params) => print_params("VerticalLine", &params),
                        &Command::SmoothQuadraticCurve(_, ref params) => print_params("SmoothQuadraticCurve", &params),
                        &Command::SmoothCubicCurve(_, ref params) => print_params("SmoothCubicCurve", &params),
                        &Command::EllipticalArc(_, ref params) => print_params("EllipticalArc", &params),
                        &Command::Close => println!("Close!"),
                    }
                    
                }
            }
            _ => {}
        }
        up_mouse();
    }
}

fn ask_screen_coord(location_name: &str) -> (f32, f32) {
    println!("{} location will be read in:", location_name);
    println!("3");
    thread::sleep(Duration::from_millis(1000));
    println!("2");
    thread::sleep(Duration::from_millis(1000));
    println!("1");
    thread::sleep(Duration::from_millis(1000));
    
    let output = std::process::Command::new("xdotool")
                         .arg("getmouselocation")
                         .output()
                         .expect("Failed to read top left");

    assert!(output.status.success());
    let output = output.stdout;
    let mut top_left_strs = std::str::from_utf8(&output).unwrap().split(" ");

    let top_left_x_str = top_left_strs.nth(0).unwrap();
    let top_left_y_str = top_left_strs.nth(0).unwrap();

    let top_left_x_str = &top_left_x_str[2..];
    let top_left_y_str = &top_left_y_str[2..];

    let x = top_left_x_str.parse::<f32>().unwrap();
    let y = top_left_y_str.parse::<f32>().unwrap();
    println!("Location of {} is {} {}", location_name, x, y);
    (x, y)
}

fn get_svg_area(path_to_svg: &str) -> Rect {
     for event in svg::open(path_to_svg).unwrap() {
        match event {
            Event::Tag(svg::node::element::tag::SVG, _, attributes) => {
                let view_box = attributes.get("viewBox");
                if view_box.is_none() {
                    continue;
                }
                let view_box = view_box.unwrap().split(" ");
                let view_box = view_box.collect::<Vec<_>>();
                let x =  view_box[0].parse::<f32>().unwrap();
                let y =  view_box[1].parse::<f32>().unwrap();
                let width = view_box[2].parse::<f32>().unwrap();
                let height = view_box[3].parse::<f32>().unwrap();
                return Rect::new(x, y, width, height);
            }
            _ => {}
        }
    }

    for event in svg::open(path_to_svg).unwrap() {
        match event {
            Event::Tag(svg::node::element::tag::SVG, _, attributes) => {
                let width = attributes.get("width");
                let height = attributes.get("height");
                if width.is_none() || height.is_none() {
                    continue;
                }
                match (width, height) {
                    (Some(width), Some(height)) => {
                        return Rect::new(0f32, 0f32, width.parse::<f32>().unwrap(), height.parse::<f32>().unwrap());
                    }
                    (_, _) => {}
                }
            }
            _ => {}
        }
    }
    panic!("Couldn't find size of SVG!");
}

fn perform_line(position_type: &Position, params: &Vec<f32>, svg_area: &Rect, paint_area: &Rect) -> SvgPoint {
    assert!(params.len() >= 2);
    assert_eq!(params.len() % 2, 0);

    let mut current_point = SvgPoint::new(*params.get(0).unwrap(), *params.get(1).unwrap(), &svg_area, &paint_area);
    move_mouse(&ScreenPoint::from(current_point.clone()));
    down_mouse();

    let mut index = 2;
    while index < params.len() {
        let x = *params.get(index).unwrap();
        let y = *params.get(index + 1).unwrap();
        match position_type {
            &Position::Relative => {
                current_point = current_point.offset(x, y);
            }
            &Position::Absolute => {
                current_point = SvgPoint::new(x, y, &svg_area, &paint_area);
            }
        }
        move_mouse(&ScreenPoint::from(current_point.clone()));
        index += 2;
    }

    current_point
}

fn perform_move(position_type: &Position, params: &Vec<f32>, svg_area: &Rect, paint_area: &Rect) -> SvgPoint {
    up_mouse();
    // If move has more than 2 points than they must be treated as implicit line.
    perform_line(position_type, params, svg_area, paint_area)
}

fn perform_cubic_curve(current_point: &SvgPoint, position_type: &Position, params: &Vec<f32>, svg_area: &Rect, paint_area: &Rect) -> SvgPoint {
    if params.len() < 6 {
        panic!("Smooth curveto not supported!");
    }

    let pos0 = &current_point;
    let pos1: SvgPoint;
    let pos2: SvgPoint;
    let pos3: SvgPoint;
    match position_type {
        &Position::Absolute => {
            pos1 = SvgPoint::new(*params.get(0).unwrap(), *params.get(1).unwrap(), &svg_area, &paint_area);
            pos2 = SvgPoint::new(*params.get(2).unwrap(), *params.get(3).unwrap(), &svg_area, &paint_area);
            pos3 = SvgPoint::new(*params.get(4).unwrap(), *params.get(5).unwrap(), &svg_area, &paint_area);
        }
        &Position::Relative => {
            pos1 = SvgPoint::new(*params.get(0).unwrap() + pos0.x(), *params.get(1).unwrap() + pos0.y(), &svg_area, &paint_area);
            pos2 = SvgPoint::new(*params.get(2).unwrap() + pos0.x(), *params.get(3).unwrap() + pos0.y(), &svg_area, &paint_area);
            pos3 = SvgPoint::new(*params.get(4).unwrap() + pos0.x(), *params.get(5).unwrap() + pos0.y(), &svg_area, &paint_area);
        }
    }
    let mut line_coords: Vec<f32> = Vec::new();
    let mut index = 0i32;
    while index <= 100 {
        let t = (index as f32) / 100f32;
        let x = (1f32-t).powi(3) * pos0.x() + 3f32*(1f32-t).powi(2) * t * pos1.x() + 3f32*(1f32-t) * t.powi(2) * pos2.x() + t.powi(3) * pos3.x();
        let y = (1f32-t).powi(3) * pos0.y() + 3f32*(1f32-t).powi(2) * t * pos1.y() + 3f32*(1f32-t) * t.powi(2) * pos2.y() + t.powi(3) * pos3.y();
        line_coords.push(x);
        line_coords.push(y);
        index += 10;
    }
    
    perform_line(&Position::Absolute, &line_coords, &svg_area, &paint_area)
}

fn print_params(command_name: &str, params: &svg::node::element::path::Parameters) {
    println!("{}! params:", command_name);
    for param in params.iter() {
        println!("\t{}", param);
    }
}

fn down_mouse() {
    let status = std::process::Command::new("xdotool")
                     .arg("mousedown")
                     .arg("1")
                     .status()
                     .expect("Failed to mousedown");
    assert!(status.success());
}

fn move_mouse(point: &ScreenPoint) {
    let status = std::process::Command::new("xdotool")
                     .arg("mousemove")
                     .arg(point.x().to_string())
                     .arg(point.y().to_string())
                     .status()
                     .expect("Failed to mousemove");
    assert!(status.success());
}

fn up_mouse() {
    let status = std::process::Command::new("xdotool")
                     .arg("mouseup")
                     .arg("1")
                     .status()
                     .expect("Failed to mousedown");
    assert!(status.success());
}
