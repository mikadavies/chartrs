use raqote::{DrawOptions, PathBuilder, StrokeStyle};

use crate::{
    colours::SolidColour,
    plot::Plot,
    shapes::{Point2d, circle, cross, diamond, plus, square, triangle_down, triangle_up},
};

pub struct ScatterConfig {
    pub marker: Marker,
    pub marker_size: f32,
    pub colour: SolidColour,
    pub fill: bool,
}

#[derive(PartialEq, Eq)]
pub enum Marker {
    Circle,
    Square,
    TriangleUp,
    TriangleDown,
    Cross,
    Plus,
    Diamond,
}

pub fn scatter(plot: &mut Plot, data: &[Point2d], config: ScatterConfig) {
    let [min, max]: [Point2d; 2] = get_data_range(data);
    let canvas_dims: Point2d =
        Point2d::new(plot.canvas.width() as f32, plot.canvas.height() as f32);

    let mut pb: PathBuilder = PathBuilder::new();

    for point in data {
        let point: Point2d = scale_to_canvas(point, &min, &max, &canvas_dims);
        match config.marker {
            Marker::Circle => circle(&mut pb, point, 0.5 * config.marker_size),
            Marker::Square => square(&mut plot.canvas, point, config.marker_size, config.colour),
            Marker::TriangleUp => triangle_up(&mut pb, point, config.marker_size),
            Marker::TriangleDown => triangle_down(&mut pb, point, config.marker_size),
            Marker::Cross => cross(&mut pb, point, config.marker_size),
            Marker::Plus => plus(&mut pb, point, config.marker_size),
            Marker::Diamond => diamond(&mut pb, point, config.marker_size),
        }
    }

    let path: raqote::Path = pb.finish();
    if config.marker != Marker::Square {
        let src = &raqote::Source::Solid(config.colour.src);
        match config.fill {
            true => plot.canvas.fill(&path, src, &DrawOptions::new()),
            false => plot
                .canvas
                .stroke(&path, src, &StrokeStyle::default(), &DrawOptions::new()),
        }
    }
}

pub(crate) fn scale_to_canvas(
    point: &Point2d,
    min: &Point2d,
    max: &Point2d,
    canvas_dims: &Point2d,
) -> Point2d {
    // New point with 7.5% display margin
    Point2d::new(
        (point.x - min.x) / (max.x - min.x) * 0.85 * canvas_dims.x + 0.075 * canvas_dims.x,
        (point.y - min.y) / (max.y - min.y) * 0.85 * canvas_dims.y + 0.075 * canvas_dims.y,
    )
}

pub(crate) fn get_data_range(data: &[Point2d]) -> [Point2d; 2] {
    let min_x: f32 = data
        .iter()
        .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap())
        .unwrap()
        .x;
    let min_y: f32 = data
        .iter()
        .min_by(|a, b| a.y.partial_cmp(&b.y).unwrap())
        .unwrap()
        .y;
    let max_x: f32 = data
        .iter()
        .max_by(|a, b| a.x.partial_cmp(&b.x).unwrap())
        .unwrap()
        .x;
    let max_y: f32 = data
        .iter()
        .max_by(|a, b| a.y.partial_cmp(&b.y).unwrap())
        .unwrap()
        .y;

    [Point2d::new(min_x, min_y), Point2d::new(max_x, max_y)]
}
