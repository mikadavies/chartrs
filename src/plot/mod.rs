use raqote::{DrawOptions, DrawTarget, PathBuilder};
use rusttype::Font;

use crate::{
    colours::{SolidColour, TextColour},
    shapes::Point2d,
};

pub mod scatter;

pub struct Plot {
    pub canvas: DrawTarget,
    _background_colour: SolidColour,
}

impl Plot {
    pub fn new(width: i32, height: i32, bg_colour: Option<SolidColour>) -> Self {
        let mut canvas: DrawTarget = DrawTarget::new(width, height);
        let background_colour: SolidColour = bg_colour.unwrap_or(SolidColour::default(
            crate::colours::ColourContent::Background,
        ));

        canvas.clear(background_colour.src);

        Self {
            canvas,
            _background_colour: background_colour,
        }
    }
}

pub struct AxisConfig<'a> {
    pub show: bool,
    pub label: Option<&'a str>,
    pub range: Option<[f32; 2]>,
    pub axis_colour: Option<SolidColour>,
    pub text_colour: Option<TextColour>,
    #[cfg(feature = "text")]
    pub font: Option<&'a Font<'a>>,
}

pub fn draw_axes(plot: &mut Plot, xconfig: &AxisConfig, yconfig: &AxisConfig) {
    // Origin coordinates (slightly offset from 0,0 on graph)
    let origin: Point2d = Point2d::new(
        0.05 * plot.canvas.width() as f32,
        0.95 * plot.canvas.height() as f32,
    );
    let extent: Point2d = Point2d::new(
        0.95 * plot.canvas.width() as f32,
        0.05 * plot.canvas.height() as f32,
    );

    let mut pb: PathBuilder = PathBuilder::new();
    if xconfig.show {
        // Go to plot origin
        pb.move_to(origin.x, origin.y);
        // Make line
        pb.rect(origin.x, origin.y, extent.x - origin.x, 1.);
    }
    let path: raqote::Path = pb.finish();
    plot.canvas.fill(
        &path,
        &raqote::Source::Solid(
            xconfig
                .axis_colour
                .unwrap_or(SolidColour::default(
                    crate::colours::ColourContent::Foreground1,
                ))
                .src,
        ),
        &DrawOptions::default(),
    );

    let mut pb: PathBuilder = PathBuilder::new();
    if yconfig.show {
        // Go to plot origin
        pb.move_to(origin.x, origin.y);
        // Make line
        pb.rect(origin.x, origin.y, 1., extent.y - origin.y);
    }
    let path: raqote::Path = pb.finish();
    plot.canvas.fill(
        &path,
        &raqote::Source::Solid(
            xconfig
                .axis_colour
                .unwrap_or(SolidColour::default(
                    crate::colours::ColourContent::Foreground1,
                ))
                .src,
        ),
        &DrawOptions::default(),
    );

    #[cfg(feature = "text")]
    {
        use crate::raqote_text::TextRenderBuffer;

        let width: f32 = plot.canvas.width() as f32;
        let height: f32 = plot.canvas.height() as f32;

        if let Some(xlabel) = xconfig.label
            && let Some(font) = xconfig.font
        {
            let textbuf: TextRenderBuffer = TextRenderBuffer::new(
                xlabel,
                14,
                xconfig.text_colour.unwrap_or(TextColour::default()),
                font,
            )
            .expect("Failed to create text buffer for xlabel");

            textbuf.render(
                &mut plot.canvas,
                Point2d::new(
                    width * 0.5 - textbuf.width as f32 * 0.5,
                    origin.y + 1.05 * textbuf.height as f32 * 0.5,
                ),
            );
        }

        if let Some(ylabel) = yconfig.label
            && let Some(font) = yconfig.font
        {
            use euclid::{Angle, Transform2D, Vector2D};

            let textbuf: TextRenderBuffer = TextRenderBuffer::new(
                ylabel,
                14,
                yconfig.text_colour.unwrap_or(TextColour::default()),
                font,
            )
            .expect("Failed to create text buffer for xlabel");

            plot.canvas
                .set_transform(&Transform2D::rotation(Angle::degrees(-90.)).then_translate(
                    Vector2D::new(
                        origin.x - 1.1 * textbuf.height as f32,
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
