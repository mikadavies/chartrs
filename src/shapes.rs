use raqote::{DrawTarget, PathBuilder};

use crate::colours::SolidColour;

pub type Point2d = euclid::Point2D<f32, euclid::UnknownUnit>;

pub fn circle(pb: &mut PathBuilder, center: Point2d, radius: f32) {
    // See: https://stackoverflow.com/questions/1734745/how-to-create-circle-with-b%C3%A9zier-curves
    const CIRCLE_OPTIMAL_BEZIER: f32 = 4.0 * (std::f32::consts::SQRT_2 - 1.0) / 3.0;

    // This is effectively the same as the circle in the `raqote_utis` crate. See: https://github.com/sk337/raqote-utils

    let offset: f32 = CIRCLE_OPTIMAL_BEZIER * radius;

    pb.move_to(center.x, center.y + radius);

    pb.cubic_to(
        center.x + offset,
        center.y + radius,
        center.x + radius,
        center.y + offset,
        center.x + radius,
        center.y,
    );

    pb.cubic_to(
        center.x + radius,
        center.y - offset,
        center.x + offset,
        center.y - radius,
        center.x,
        center.y - radius,
    );

    pb.cubic_to(
        center.x - offset,
        center.y - radius,
        center.x - radius,
        center.y - offset,
        center.x - radius,
        center.y,
    );

    pb.cubic_to(
        center.x - radius,
        center.y + radius - offset,
        center.x - offset,
        center.y + radius,
        center.x,
        center.y + radius,
    );
}

pub fn square(canvas: &mut DrawTarget, center: Point2d, width: f32, colour: SolidColour) {
    canvas.fill_rect(
        center.x - 0.5 * width,
        center.y + 0.5 * width,
        width,
        width,
        &raqote::Source::Solid(colour.src),
        &raqote::DrawOptions::new(),
    );
}

pub fn triangle_up(pb: &mut PathBuilder, center: Point2d, width: f32) {
    let vertices: [Point2d; 3] = [
        Point2d::new(center.x - 0.5 * width, center.y + 0.5 * width),
        Point2d::new(center.x + 0.5 * width, center.y + 0.5 * width),
        Point2d::new(center.x, center.y - 0.5 * width),
    ];

    pb.move_to(vertices[2].x, vertices[2].y);
    for point in vertices {
        pb.line_to(point.x, point.y);
    }
}

pub fn triangle_down(pb: &mut PathBuilder, center: Point2d, width: f32) {
    let vertices: [Point2d; 3] = [
        Point2d::new(center.x - 0.5 * width, center.y - 0.5 * width),
        Point2d::new(center.x + 0.5 * width, center.y - 0.5 * width),
        Point2d::new(center.x, center.y + 0.5 * width),
    ];

    pb.move_to(vertices[2].x, vertices[2].y);
    for point in vertices {
        pb.line_to(point.x, point.y);
    }
}

pub fn cross(pb: &mut PathBuilder, center: Point2d, width: f32) {
    pb.move_to(center.x - 0.5 * width, center.y - 0.5 * width);
    pb.line_to(center.x + 0.5 * width, center.y + 0.5 * width);
    pb.move_to(center.x + 0.5 * width, center.y - 0.5 * width);
    pb.line_to(center.x - 0.5 * width, center.y + 0.5 * width);
}

pub fn plus(pb: &mut PathBuilder, center: Point2d, width: f32) {
    pb.move_to(center.x - 0.5 * width, center.y);
    pb.line_to(center.x + 0.5 * width, center.y);
    pb.move_to(center.x, center.y - 0.5 * width);
    pb.line_to(center.x, center.y + 0.5 * width);
}

pub fn diamond(pb: &mut PathBuilder, center: Point2d, width: f32) {
    pb.move_to(center.x - 0.5 * width, center.y);
    pb.line_to(center.x, center.y - 0.5 * width);
    pb.line_to(center.x + 0.5 * width, center.y);
    pb.line_to(center.x, center.y + 0.5 * width);
    pb.line_to(center.x - 0.5 * width, center.y);
}
