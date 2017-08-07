extern crate svg;

use svg::node::element::path::Data;
use svg::parser::Event;
use std::time::Duration;
use std::thread;

mod point;
mod rect;
mod mouse;
mod my_error;
mod painter;

use rect::Rect;
use painter::Painter;

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
                let mut painter = Painter::new(svg_area.clone(), paint_area.clone());

                for command in data.iter() {
                    painter.perform_command(command).unwrap();
                }
            }
            _ => {}
        }
        mouse::up().unwrap();
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
    
    mouse::coords().unwrap()
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
