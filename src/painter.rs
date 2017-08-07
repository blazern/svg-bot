use svg::node::element::path::{Command, Position, Parameters};
use rect::Rect;
use point::{SvgPoint, ScreenPoint};
use my_error::MyError;
use mouse;

pub struct Painter {
    current_point: SvgPoint,
    subpath_initial_point: Option<SvgPoint>,
    svg_area: Rect,
    screen_area: Rect,
}

impl Painter {
    pub fn new(svg_area: Rect, screen_area: Rect) -> Painter {
        Painter {
            current_point: SvgPoint::new(0f32, 0f32, &svg_area, &screen_area),
            subpath_initial_point: None,
            svg_area: svg_area,
            screen_area: screen_area
        }
    }

    pub fn perform_command(&mut self, command: &Command) -> Result<(), MyError> {
        match command {
            &Command::Move(ref position_type, ref params) => {
                self.perform_move(&position_type, &params.to_vec())?;
            },
            &Command::Line(ref position_type, ref params) => {
                self.perform_line(&position_type, &params.to_vec())?;
            },
            &Command::CubicCurve(ref position_type, ref params) => {
                self.perform_cubic_curve(&position_type, &params.to_vec())?;
            },
            &Command::QuadraticCurve(_, ref params) => print_params("QuadraticCurve", &params),
            &Command::HorizontalLine(_, ref params) => print_params("HorizontalLine", &params),
            &Command::VerticalLine(_, ref params) => print_params("VerticalLine", &params),
            &Command::SmoothQuadraticCurve(_, ref params) => print_params("SmoothQuadraticCurve", &params),
            &Command::SmoothCubicCurve(_, ref params) => print_params("SmoothCubicCurve", &params),
            &Command::EllipticalArc(_, ref params) => print_params("EllipticalArc", &params),
            &Command::Close => println!("Close!"),
        }
        Ok(())
    }

    fn perform_move(&mut self, position_type: &Position, params: &Vec<f32>) -> Result<(), MyError> {
        assert!(params.len() >= 2);

        let new_initial_x: f32;
        let new_initial_y: f32;
        match position_type {
            &Position::Absolute => {
                new_initial_x = *params.get(0).unwrap();
                new_initial_y = *params.get(1).unwrap();
            }
            &Position::Relative => {
                new_initial_x = *params.get(0).unwrap() + self.current_point.x();
                new_initial_y = *params.get(1).unwrap() + self.current_point.y();
            }
        }
        self.subpath_initial_point = Some(SvgPoint::new(new_initial_x, new_initial_y, &self.svg_area, &self.screen_area));

        mouse::up()?;
        // If move has more than 2 points than they must be treated as implicit line.
        self.perform_line(position_type, params)
    }

    fn perform_line(&mut self, position_type: &Position, params: &Vec<f32>) -> Result<(), MyError> {
        assert!(params.len() >= 2);
        assert_eq!(params.len() % 2, 0);

        let svg_area = &self.svg_area;
        let screen_area = &self.screen_area;

        let mut current_point = SvgPoint::new(*params.get(0).unwrap(), *params.get(1).unwrap(), svg_area, screen_area);
        let current_screen_point = ScreenPoint::from(current_point.clone());
        mouse::move_to(current_screen_point.x(), current_screen_point.y())?;
        mouse::down()?;

        let mut index = 2;
        while index < params.len() {
            let x = *params.get(index).unwrap();
            let y = *params.get(index + 1).unwrap();
            match position_type {
                &Position::Relative => {
                    current_point = current_point.offset(x, y);
                }
                &Position::Absolute => {
                    current_point = SvgPoint::new(x, y, svg_area, screen_area);
                }
            }
            let current_screen_point = ScreenPoint::from(current_point.clone());
            mouse::move_to(current_screen_point.x(), current_screen_point.y())?;
            index += 2;
        }

        self.current_point = current_point;
        Ok(())
    }

    fn perform_cubic_curve(&mut self, position_type: &Position, params: &Vec<f32>) -> Result<(), MyError> {
        assert!(params.len() >= 6);

        let mut line_coords: Vec<f32> = Vec::new();
        {
            let pos0 = &self.current_point;
            let pos1: SvgPoint;
            let pos2: SvgPoint;
            let pos3: SvgPoint;
            match position_type {
                &Position::Absolute => {
                    pos1 = SvgPoint::new(*params.get(0).unwrap(), *params.get(1).unwrap(), &self.svg_area, &self.screen_area);
                    pos2 = SvgPoint::new(*params.get(2).unwrap(), *params.get(3).unwrap(), &self.svg_area, &self.screen_area);
                    pos3 = SvgPoint::new(*params.get(4).unwrap(), *params.get(5).unwrap(), &self.svg_area, &self.screen_area);
                }
                &Position::Relative => {
                    pos1 = SvgPoint::new(*params.get(0).unwrap() + pos0.x(), *params.get(1).unwrap() + pos0.y(), &self.svg_area, &self.screen_area);
                    pos2 = SvgPoint::new(*params.get(2).unwrap() + pos0.x(), *params.get(3).unwrap() + pos0.y(), &self.svg_area, &self.screen_area);
                    pos3 = SvgPoint::new(*params.get(4).unwrap() + pos0.x(), *params.get(5).unwrap() + pos0.y(), &self.svg_area, &self.screen_area);
                }
            }
            let mut index = 0i32;
            while index <= 100 {
                let t = (index as f32) / 100f32;
                let x = (1f32-t).powi(3) * pos0.x() + 3f32*(1f32-t).powi(2) * t * pos1.x() + 3f32*(1f32-t) * t.powi(2) * pos2.x() + t.powi(3) * pos3.x();
                let y = (1f32-t).powi(3) * pos0.y() + 3f32*(1f32-t).powi(2) * t * pos1.y() + 3f32*(1f32-t) * t.powi(2) * pos2.y() + t.powi(3) * pos3.y();
                line_coords.push(x);
                line_coords.push(y);
                index += 10;
            }
        }
        
        self.perform_line(&Position::Absolute, &line_coords)
    }
}

fn print_params(command_name: &str, params: &Parameters) {
    println!("{}! params:", command_name);
    for param in params.iter() {
        println!("\t{}", param);
    }
}