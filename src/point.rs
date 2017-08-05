use std::convert::From;
use rect::Rect;

#[derive(Clone, Debug)]
pub struct SvgPoint {
    x: f32,
    y: f32,
    svg_area: Rect,
    screen_area: Rect,
}

#[derive(Clone, Debug)]
pub struct ScreenPoint {
    x: f32,
    y: f32,
    svg_area: Rect,
    screen_area: Rect,
}

impl SvgPoint {
    pub fn new(x: f32, y: f32, svg_area: &Rect, screen_area: &Rect) -> SvgPoint {
        SvgPoint{ x: x, y: y, svg_area: svg_area.clone(), screen_area: screen_area.clone() }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn svg_area(&self) -> &Rect {
        &self.svg_area
    }

    pub fn screen_area(&self) -> &Rect {
        &self.screen_area
    }

    pub fn offset(self, x: f32, y: f32) -> SvgPoint {
        SvgPoint::new(self.x + x, self.y + y, &self.svg_area, &self.screen_area)
    }
}

impl From<SvgPoint> for ScreenPoint {
    fn from(svg_point: SvgPoint) -> Self {
        let (svg_x, svg_y, screen_area, svg_area) = (svg_point.x, svg_point.y, svg_point.screen_area, svg_point.svg_area);
        let new_x = svg_x * screen_area.width()/svg_area.width() + screen_area.x();
        let new_y = svg_y * screen_area.height()/svg_area.height() + screen_area.y();
        ScreenPoint::new(new_x, new_y, &svg_area, &screen_area)
    }
}

impl From<ScreenPoint> for SvgPoint {
    fn from(screen_point: ScreenPoint) -> Self {
        let (screen_x, screen_y, screen_area, svg_area) = (screen_point.x, screen_point.y, screen_point.screen_area, screen_point.svg_area);
        let new_x = (screen_x - screen_area.x())*(svg_area.width()/screen_area.width());
        let new_y = (screen_y - screen_area.y())*(svg_area.height()/screen_area.height());
        SvgPoint::new(new_x, new_y, &svg_area, &screen_area)
    }
}

impl ScreenPoint {
    pub fn new(x: f32, y: f32, svg_area: &Rect, screen_area: &Rect) -> ScreenPoint {
        ScreenPoint{ x: x, y: y, svg_area: svg_area.clone(), screen_area: screen_area.clone() }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn svg_area(&self) -> &Rect {
        &self.svg_area
    }

    pub fn screen_area(&self) -> &Rect {
        &self.screen_area
    }

    pub fn offset(self, x: f32, y: f32) -> ScreenPoint {
        ScreenPoint::new(self.x + x, self.y + y, &self.svg_area, &self.screen_area)
    }
}