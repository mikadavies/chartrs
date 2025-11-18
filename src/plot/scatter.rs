use raqote::{DrawOptions, PathBuilder, StrokeStyle};

use crate::{
    colours::SolidColour,
    plot::Plot,
    shapes::{Point2d, circle, cross, diamond, plus, square, triangle_down, triangle_up},
};

/// Aesthetic configuration for a scatter plot
pub struct ScatterConfig {
    /// The shape of the scatter plot marker
    pub marker: Marker,
    /// The size of the scatter plot marker (in canvas coordinates)
    pub marker_size: f32,
    /// The colour of the scatter plot marker
    pub colour: SolidColour,
    /// Whether the markers should be filled or outlined. The `Cross` and `Plus` marker shapes only support outlined (`false`).
    pub fill: bool,
}

/// The different marker shapes
#[derive(PartialEq, Eq)]
pub enum Marker {
    ///  ●
    Circle,
    /// ■
    Square,
    /// ▲
    TriangleUp,
    /// ▼
    TriangleDown,
    /// ×
    Cross,
    /// +
    Plus,
    /// ◆
    Diamond,
}

/// Draw a scatter plot based on a collection of points (in plot/data coordinates)
pub fn scatter(plot: &mut Plot, data: &[Point2d], config: ScatterConfig) {
    let [min, max]: [Point2d; 2] = get_data_range(data);

    // Update plot extents if unset
    if plot.extent.xmin.is_none() {
        plot.extent.xmin = Some(min.x);
    }
    if plot.extent.xmax.is_none() {
        plot.extent.xmax = Some(max.x);
    }
    if plot.extent.ymin.is_none() {
        plot.extent.ymin = Some(min.y);
    }
    if plot.extent.ymax.is_none() {
        plot.extent.ymax = Some(max.y);
    }

    let mut pb: PathBuilder = PathBuilder::new();

    for point in data {
        // let point: Point2d = scale_to_canvas(point, &min, &max, &canvas_dims);
        let point: Point2d = to_plot_coordinates(plot, point, &min, &max);
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

/// Convert plot/data coordinates to canvas coordinates
pub fn to_plot_coordinates(
    plot: &Plot,
    point: &Point2d,
    min_value: &Point2d,
    max_value: &Point2d,
) -> Point2d {
    let [top_left, bottom_right]: [Point2d; 2] = plot.bbox();

    let xval_min: f32 = plot.extent.xmin.unwrap_or(min_value.x);
    let yval_min: f32 = plot.extent.ymin.unwrap_or(min_value.y);
    let xval_max: f32 = plot.extent.xmax.unwrap_or(max_value.x);
    let yval_max: f32 = plot.extent.ymax.unwrap_or(max_value.y);

    let offset: Point2d = plot.origin();
    let scaling_factor: Point2d = Point2d::new(
        (bottom_right.x - top_left.x) / (xval_max - xval_min),
        (top_left.y - bottom_right.y) / (yval_max - yval_min),
    );

    Point2d::new(
        scaling_factor.x * (point.x - xval_min) + offset.x,
        scaling_factor.y * (point.y - yval_min) + offset.y,
    )
}
