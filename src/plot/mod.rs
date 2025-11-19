use raqote::{DrawOptions, DrawTarget, PathBuilder, StrokeStyle};
use rusttype::Font;

use crate::{
    colours::{SolidColour, TextColour},
    shapes::Point2d,
};

/// Scatter plot functions
pub mod scatter;

/// Generic `Plot` struct on which to draw.
pub struct Plot {
    /// The canvas on which the plot is drawn
    pub canvas: DrawTarget,
    /// The range of the visible plot. Will automatically scale to fit the data if not specified
    pub extent: PlotExtent,
    /// The margins between the plot area and the edge of the canvas (as a fraction of the entire canvas)
    pub margins: Margins,
    /// The background colour for the canvas
    pub background_colour: SolidColour,
    max_label_offset: Point2d,
}

/// The extents of a plot in plot (data) coordinates.
pub struct PlotExtent {
    pub xmin: Option<f32>,
    pub xmax: Option<f32>,
    pub ymin: Option<f32>,
    pub ymax: Option<f32>,
}
impl Default for PlotExtent {
    fn default() -> Self {
        Self {
            xmin: None,
            xmax: None,
            ymin: None,
            ymax: None,
        }
    }
}

/// The margins of a plot as a fraction of the canvas dimensions.
pub struct Margins {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
impl Default for Margins {
    fn default() -> Self {
        Self {
            left: 0.1,
            right: 0.05,
            top: 0.05,
            bottom: 0.1,
        }
    }
}

impl Plot {
    /// Create a new plot with a specified width, height, background colour, plotted coordinate range, and margins.
    pub fn new(
        width: i32,
        height: i32,
        bg_colour: Option<SolidColour>,
        extent: PlotExtent,
        margins: Margins,
    ) -> Self {
        let mut canvas: DrawTarget = DrawTarget::new(width, height);
        let background_colour: SolidColour = bg_colour.unwrap_or(SolidColour::default(
            crate::colours::ColourContent::Background,
        ));

        canvas.clear(background_colour.src);

        Self {
            canvas,
            extent,
            margins,
            max_label_offset: Point2d::zero(),
            background_colour: background_colour,
        }
    }

    /// Get the canvas coordinates of the plot origin.
    pub fn origin(&self) -> Point2d {
        // Origin coordinates (slightly offset from 0,0 on graph)
        Point2d::new(
            self.margins.left * self.canvas.width() as f32,
            (1.0 - self.margins.bottom) * self.canvas.height() as f32,
        )
    }

    /// Get the plot bounding box in canvas coordinates as two points: `[top_left, bottom_right]`
    pub fn bbox(&self) -> [Point2d; 2] {
        [
            Point2d::new(
                self.margins.left * self.canvas.width() as f32,
                self.margins.top * self.canvas.height() as f32,
            ),
            Point2d::new(
                (1.0 - self.margins.right) * self.canvas.width() as f32,
                (1.0 - self.margins.bottom) * self.canvas.height() as f32,
            ),
        ]
    }
}

/// Axis configuration struct.
pub struct AxisConfig<'a> {
    /// Whether or not the axis should be drawn
    pub show: bool,
    /// The axis label, if applicable (requires `text` feature to display)
    pub label: Option<&'a str>,
    /// The colour of the axis, if applicable
    pub axis_colour: Option<SolidColour>,
    /// The colour of the axis text (e.g. axis label, tick labels)
    pub text_colour: Option<TextColour>,
    /// Axis ticks options
    pub ticks: AxisTicks<'a>,
    /// Axis tick size, in canvas coordinates, if applicable
    pub tick_size: Option<f32>,
    #[cfg(feature = "text")]
    /// The font with which to draw the text, if applicable (requires `text` feature)
    pub font: Option<&'a Font<'a>>,
}

/// Axis ticks configuration enum
pub enum AxisTicks<'a> {
    /// No axis ticks
    None,
    /// A specific number of ticks
    Count(usize),
    /// Ticks every so often (interval in plot/data coordinates)
    Interval(f32),
    /// Ticks at specified x/y values (specified in plot/data coordinates)
    Values(&'a [f32]),
}

/// Draw the axes on a plot
pub fn draw_axes(plot: &mut Plot, xconfig: &AxisConfig, yconfig: &AxisConfig) {
    let origin: Point2d = plot.origin();
    let [top_left, bottom_right]: [Point2d; 2] = plot.bbox();

    let mut pb: PathBuilder = PathBuilder::new();
    if xconfig.show {
        // Go to plot origin
        pb.move_to(origin.x, origin.y);
        // Make line
        pb.line_to(bottom_right.x, bottom_right.y);
    }
    let path: raqote::Path = pb.finish();
    plot.canvas.stroke(
        &path,
        &raqote::Source::Solid(
            xconfig
                .axis_colour
                .unwrap_or(SolidColour::default(
                    crate::colours::ColourContent::Foreground1,
                ))
                .src,
        ),
        &StrokeStyle::default(),
        &DrawOptions::default(),
    );

    let mut pb: PathBuilder = PathBuilder::new();
    if yconfig.show {
        // Go to plot origin
        pb.move_to(origin.x, origin.y);
        // Make line
        pb.line_to(top_left.x, top_left.y);
    }
    let path: raqote::Path = pb.finish();
    plot.canvas.stroke(
        &path,
        &raqote::Source::Solid(
            yconfig
                .axis_colour
                .unwrap_or(SolidColour::default(
                    crate::colours::ColourContent::Foreground1,
                ))
                .src,
        ),
        &StrokeStyle::default(),
        &DrawOptions::default(),
    );

    plot_axis_ticks(plot, xconfig, yconfig);
    plot.max_label_offset = Point2d::new(
        plot.max_label_offset.x + 0.5 * xconfig.tick_size.unwrap_or(0.0),
        plot.max_label_offset.y + 0.5 * yconfig.tick_size.unwrap_or(0.0),
    );

    #[cfg(feature = "text")]
    {
        use crate::raqote_text::TextRenderBuffer;

        let width: f32 = plot.canvas.width() as f32;
        let height: f32 = plot.canvas.height() as f32;

        if let Some(xlabel) = xconfig.label
            && let Some(font) = xconfig.font
        {
            let textbuf: TextRenderBuffer =
                TextRenderBuffer::new(xlabel, 14, xconfig.text_colour.unwrap_or_default(), font)
                    .expect("Failed to create text buffer for xlabel");

            textbuf.render(
                &mut plot.canvas,
                Point2d::new(
                    width * 0.5 - textbuf.width as f32 * 0.5,
                    origin.y + 2.0 * textbuf.height as f32 * 0.5 + plot.max_label_offset.x,
                ),
            );
        }

        if let Some(ylabel) = yconfig.label
            && let Some(font) = yconfig.font
        {
            use euclid::{Angle, Transform2D, Vector2D};

            let textbuf: TextRenderBuffer =
                TextRenderBuffer::new(ylabel, 14, yconfig.text_colour.unwrap_or_default(), font)
                    .expect("Failed to create text buffer for xlabel");

            plot.canvas
                .set_transform(&Transform2D::rotation(Angle::degrees(-90.)).then_translate(
                    Vector2D::new(
                        origin.x - 1.5 * textbuf.height as f32 - plot.max_label_offset.y,
                        height * 0.5 + textbuf.width as f32 * 0.5,
                    ),
                ));

            textbuf.render(&mut plot.canvas, Point2d::zero());

            plot.canvas
                .set_transform(&Transform2D::rotation(Angle::zero()));
            plot.canvas.set_transform(&Transform2D::translation(0., 0.));
        }
    }
}

// This should only be called if all plot.extent values have been set to Some, or if a plotting functions has already been called.
// Currently does not support dynamic resizing if multiple series are plotted
// Iterator and mutable variable fuckery going on here, so temporarily resorted to collecting iterators into `Vec`s. TODO: Fix and use iterators directly
// Currently clips through axis labels, as they are not considered. TODO: Fix that.
/// Plot ticks on axes
pub fn plot_axis_ticks(plot: &mut Plot, xconfig: &AxisConfig, yconfig: &AxisConfig) {
    // X-axis
    let mut pb: PathBuilder = PathBuilder::new();
    if let Some(ticks_plot_coords) = ticks_in_plot_coordinates(plot, true, false, xconfig) {
        let tick_size: f32 = xconfig.tick_size.unwrap_or(10.0);
        let ticks_canvas_coords =
            Vec::from_iter(ticks_in_canvas_coordinates(plot, ticks_plot_coords.iter()));
        for (tick_canvas, tick_plot) in ticks_canvas_coords.iter().zip(ticks_plot_coords) {
            render_tick_label(plot, true, false, &tick_plot, tick_canvas, xconfig);
            draw_axis_tick(&mut pb, true, false, *tick_canvas, tick_size);
        }
    }

    let path: raqote::Path = pb.finish();
    plot.canvas.stroke(
        &path,
        &raqote::Source::Solid(
            xconfig
                .axis_colour
                .unwrap_or(SolidColour::default(
                    crate::colours::ColourContent::Foreground1,
                ))
                .src,
        ),
        &StrokeStyle::default(),
        &DrawOptions::default(),
    );

    // Y-axis
    let mut pb: PathBuilder = PathBuilder::new();

    if let Some(ticks_plot_coords) = ticks_in_plot_coordinates(plot, false, true, yconfig) {
        let tick_size: f32 = yconfig.tick_size.unwrap_or(10.0);
        let ticks_canvas_coords =
            Vec::from_iter(ticks_in_canvas_coordinates(plot, ticks_plot_coords.iter()));
        for (tick_canvas, tick_plot) in ticks_canvas_coords.iter().zip(ticks_plot_coords) {
            render_tick_label(plot, false, true, &tick_plot, tick_canvas, yconfig);
            draw_axis_tick(&mut pb, false, true, *tick_canvas, tick_size);
        }
    }
    let path: raqote::Path = pb.finish();
    plot.canvas.stroke(
        &path,
        &raqote::Source::Solid(
            xconfig
                .axis_colour
                .unwrap_or(SolidColour::default(
                    crate::colours::ColourContent::Foreground1,
                ))
                .src,
        ),
        &StrokeStyle::default(),
        &DrawOptions::default(),
    );
}

fn draw_axis_tick(pb: &mut PathBuilder, xaxis: bool, yaxis: bool, coords: Point2d, size: f32) {
    if xaxis {
        pb.move_to(coords.x, coords.y - 0.5 * size);
        pb.line_to(coords.x, coords.y + 0.5 * size);
    }
    if yaxis {
        pb.move_to(coords.x - 0.5 * size, coords.y);
        pb.line_to(coords.x + 0.5 * size, coords.y);
    }
}

fn ticks_in_plot_coordinates(
    plot: &Plot,
    xaxis: bool,
    yaxis: bool,
    config: &AxisConfig,
) -> Option<Vec<Point2d>> {
    match config.ticks {
        AxisTicks::None => None,
        AxisTicks::Count(count) => {
            if !xaxis && !yaxis {
                return None;
            }

            let min: f32 = if xaxis {
                plot.extent.xmin.unwrap()
            } else {
                plot.extent.ymin.unwrap()
            };

            let max: f32 = if xaxis {
                plot.extent.xmax.unwrap()
            } else {
                plot.extent.ymax.unwrap()
            };

            let step: f32 = (max - min) / count as f32;
            Some(
                (0..=count)
                    .map(move |n| {
                        if xaxis {
                            Point2d::new(n as f32 * step, 0.0)
                        } else {
                            Point2d::new(0.0, n as f32 * step)
                        }
                    })
                    .collect(),
            )
        }
        AxisTicks::Interval(step) => {
            if !xaxis && !yaxis {
                return None;
            }

            let min: f32 = if xaxis {
                plot.extent.xmin.unwrap()
            } else {
                plot.extent.ymin.unwrap()
            };

            let max: f32 = if xaxis {
                plot.extent.xmax.unwrap()
            } else {
                plot.extent.ymax.unwrap()
            };

            let count: f32 = (max - min) / step;
            Some(
                (0..=(count as i32))
                    .map(move |n| {
                        if xaxis {
                            Point2d::new(n as f32 * step, 0.0)
                        } else {
                            Point2d::new(0.0, n as f32 * step)
                        }
                    })
                    .collect(),
            )
        }
        AxisTicks::Values(items) => {
            if xaxis || yaxis {
                Some(
                    items
                        .iter()
                        .map(move |val| {
                            if xaxis {
                                Point2d::new(*val, 0.0)
                            } else {
                                Point2d::new(0.0, *val)
                            }
                        })
                        .collect(),
                )
            } else {
                None
            }
        }
    }
}

fn ticks_in_canvas_coordinates<'a>(
    plot: &Plot,
    plot_coord_ticks: impl Iterator<Item = &'a Point2d>,
) -> impl Iterator<Item = Point2d> {
    let min: Point2d = Point2d::new(plot.extent.xmin.unwrap(), plot.extent.ymin.unwrap());
    let max: Point2d = Point2d::new(plot.extent.xmax.unwrap(), plot.extent.ymax.unwrap());

    plot_coord_ticks.map(move |point| scatter::to_plot_coordinates(plot, point, &min, &max))
}

#[cfg(feature = "text")]
fn render_tick_label(
    plot: &mut Plot,
    xaxis: bool,
    yaxis: bool,
    point_plot: &Point2d,
    point_canvas: &Point2d,
    config: &AxisConfig,
) {
    if let Some(font) = config.font
        && (xaxis || yaxis)
    {
        use crate::raqote_text::TextRenderBuffer;

        let label: String = format!("{:.2}", if xaxis { point_plot.x } else { point_plot.y });

        let textbuf: TextRenderBuffer =
            TextRenderBuffer::new(&label, 12, config.text_colour.unwrap_or_default(), font)
                .expect("Failed to create text buffer for xlabel");

        textbuf.render(
            &mut plot.canvas,
            if xaxis {
                plot.max_label_offset.x = plot.max_label_offset.x.max(textbuf.height as f32);
                Point2d::new(
                    point_canvas.x - 0.5 * textbuf.width as f32,
                    point_canvas.y
                        + 0.5 * textbuf.height as f32
                        + 0.5 * config.tick_size.unwrap_or(10.0),
                )
            } else {
                plot.max_label_offset.y = plot.max_label_offset.y.max(textbuf.width as f32);
                Point2d::new(
                    point_canvas.x
                        - 1.25 * textbuf.width as f32
                        - 0.5 * config.tick_size.unwrap_or(10.0),
                    point_canvas.y - 0.5 * textbuf.height as f32,
                )
            },
        );
    }
}

#[cfg(feature = "png")]
/// Exports a plot to a PNG file. Requires the `png` feature.
pub fn export_png(plot: &Plot, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(plot.canvas.write_png(path)?)
}
