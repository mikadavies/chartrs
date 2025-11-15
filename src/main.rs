use crate::{
    colours::SolidColour,
    display::plot_interactive,
    plot::{AxisConfig, draw_axes, scatter::ScatterConfig},
    raqote_text::{DEFAULT_FONT_PATH, load_font},
    shapes::Point2d,
};

pub mod colours;
pub mod display;
pub mod plot;
pub mod raqote_text;
pub mod shapes;

fn main() {
    let mut plot: plot::Plot = plot::Plot::new(1000, 800, None);

    plot::scatter::scatter(
        &mut plot,
        &[
            Point2d::new(5., 10.),
            Point2d::new(6., 9.),
            Point2d::new(4., 5.),
        ],
        ScatterConfig {
            marker: plot::scatter::Marker::Diamond,
            marker_size: 10.0,
            colour: SolidColour::default(colours::ColourContent::Foreground1),
            fill: true,
        },
    );

    let font: rusttype::Font<'_> = load_font(DEFAULT_FONT_PATH).unwrap();

    draw_axes(
        &mut plot,
        &AxisConfig {
            show: true,
            label: Some("X-axis"),
            range: None,
            axis_colour: None,
            text_colour: None,
            font: Some(&font),
        },
        &AxisConfig {
            show: true,
            label: Some("Y-axis"),
            range: None,
            axis_colour: None,
            text_colour: None,
            font: Some(&font),
        },
    );

    plot_interactive(&plot).unwrap();
}
