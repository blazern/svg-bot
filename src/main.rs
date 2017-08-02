extern crate svg;

use svg::node::element::path::{Command, Data, Position, Parameters};
use svg::parser::Event;
use std::time::Duration;
use std::thread;

#[derive(Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug)]
struct Size {
    width: f32,
    height: f32,
}

#[derive(Clone, Debug)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect{ x: x, y: y, width: width, height: height }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    println!("path: {}", path);

    let top_left = ask_screen_coord("Top left");
    let bottom_right = ask_screen_coord("Bottom right");
    let paint_area = Rect::new(top_left.x, top_left.y, bottom_right.x - top_left.x, bottom_right.y - top_left.y);

    assert!(top_left.x < bottom_right.x);
    assert!(top_left.y < bottom_right.y);

    let svg_area = get_svg_area(path);
    println!("SVG's width: {}, height: {}", svg_area.width, svg_area.height);
    
    for event in svg::open(path).unwrap() {
        match event {
            Event::Tag(svg::node::element::tag::Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                let mut current_point: Option<Point> = None;
                for command in data.iter() {
                    match command {
                        &Command::Move(ref position_type, ref params) => {
                            let result_point = perform_move(&current_point, &position_type, &params.to_vec(), &svg_area, &paint_area);
                            current_point = Some(result_point);
                        },
                        &Command::Line(ref position_type, ref params) => {
                            let result_point = perform_line(&current_point, &position_type, &params.to_vec(), &svg_area, &paint_area);
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

fn ask_screen_coord(location_name: &str) -> Point {
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

    let result = Point{ x: top_left_x_str.parse::<f32>().unwrap(), y: top_left_y_str.parse::<f32>().unwrap() };
    println!("Location of {} is {:?}", location_name, result);
    result
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

fn perform_line(current_point: &Option<Point>, position_type: &Position, params: &Vec<f32>, svg_area: &Rect, paint_area: &Rect) -> Point {
    assert!(params.len() >= 2);
    assert_eq!(params.len() % 2, 0);

    let mut current_point = current_point.clone();
    let mut last_point: Option<Point>;

    let point_to_recalculate = Point{ x: *params.get(0).unwrap(), y: *params.get(1).unwrap() };
    current_point = Some(recalculate_coords(&point_to_recalculate, &svg_area, &paint_area));
    move_mouse(current_point.as_ref().unwrap());
    down_mouse();

    let mut index = 2;
    while index < params.len() {
        let x = *params.get(index).unwrap();
        let y = *params.get(index + 1).unwrap();
        match position_type {
            &Position::Relative => {
                let offset = Point{ x: x, y: y };
                let offset = recalculate_offset(&offset, &svg_area, &paint_area);
                current_point = Some(Point{
                    x: current_point.as_ref().unwrap().x + offset.x,
                    y: current_point.as_ref().unwrap().y + offset.y });
            }
            &Position::Absolute => {
                current_point = Some(recalculate_coords(&Point{ x: x, y: y }, &svg_area, &paint_area));
            }
        }
        last_point = current_point.clone();
        move_mouse(current_point.as_ref().unwrap());
        index += 2;
    }

    current_point.unwrap()
}

fn perform_move(current_point: &Option<Point>, position_type: &Position, params: &Vec<f32>, svg_area: &Rect, paint_area: &Rect) -> Point {
    up_mouse();
    // If move has more than 2 points than they must be treated as implicit line.
    perform_line(current_point, position_type, params, svg_area, paint_area)
}

fn perform_cubic_curve(current_point: &Point, position_type: &Position, params: &Vec<f32>, svg_area: &Rect, paint_area: &Rect) -> Point {
    if params.len() < 6 {
        panic!("Smooth curveto not supported!");
    }

    let pos0 = recalculate_coords_back(&current_point, &svg_area, &paint_area);
    let pos1: Point;
    let pos2: Point;
    let pos3: Point;
    match position_type {
        &Position::Absolute => {
            pos1 = Point{ x: *params.get(0).unwrap(), y: *params.get(1).unwrap() };
            pos2 = Point{ x: *params.get(2).unwrap(), y: *params.get(3).unwrap() };
            pos3 = Point{ x: *params.get(4).unwrap(), y: *params.get(5).unwrap() };
        }
        &Position::Relative => {
            pos1 = Point{ x: *params.get(0).unwrap() + pos0.x, y: *params.get(1).unwrap() + pos0.y };
            pos2 = Point{ x: *params.get(2).unwrap() + pos0.x, y: *params.get(3).unwrap() + pos0.y };
            pos3 = Point{ x: *params.get(4).unwrap() + pos0.x, y: *params.get(5).unwrap() + pos0.y };
        }
    }
    let mut line_coords: Vec<f32> = Vec::new();
    let mut index = 0i32;
    while index <= 100 {
        let t = (index as f32) / 100f32;
        let x = (1f32-t).powi(3) * pos0.x + 3f32*(1f32-t).powi(2) * t * pos1.x + 3f32*(1f32-t) * t.powi(2) * pos2.x + t.powi(3) * pos3.x;
        let y = (1f32-t).powi(3) * pos0.y + 3f32*(1f32-t).powi(2) * t * pos1.y + 3f32*(1f32-t) * t.powi(2) * pos2.y + t.powi(3) * pos3.y;
        line_coords.push(x);
        line_coords.push(y);
        index += 10;
    }
    
    perform_line(&Some(current_point.clone()), &Position::Absolute, &line_coords, &svg_area, &paint_area)
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

fn move_mouse(point: &Point) {
    let status = std::process::Command::new("xdotool")
                     .arg("mousemove")
                     .arg(point.x.to_string())
                     .arg(point.y.to_string())
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

fn recalculate_coords(point: &Point, svg_area: &Rect, paint_area: &Rect) -> Point {
    let new_x = point.x * paint_area.width/svg_area.width + paint_area.x;
    let new_y = point.y * paint_area.height/svg_area.height + paint_area.y;
    Point{ x: new_x, y: new_y }
}

fn recalculate_offset(offset: &Point, svg_area: &Rect, paint_area: &Rect) -> Point {
    let new_offset_x = offset.x * paint_area.width/svg_area.width;
    let new_offset_y = offset.y * paint_area.height/svg_area.height;
    Point{ x: new_offset_x, y: new_offset_y }
}

fn recalculate_coords_back(point: &Point, svg_area: &Rect, paint_area: &Rect) -> Point {
    let new_x = (point.x - paint_area.x)*(svg_area.width/paint_area.width);
    let new_y = (point.y - paint_area.y)*(svg_area.height/paint_area.height);
    Point{ x: new_x, y: new_y }
}