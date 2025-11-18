#![cfg(feature = "text")]

// Solution for rendering text correctly in Raqote, inspired by the fix shared at: https://gist.github.com/J-Cake/ddccf99d3f7d6fc947fc60204aa41e09
// Unlike that solution however, there is no texture chaching here, as we don't expect much repeating text.

use raqote::{DrawOptions, DrawTarget};
use rusttype::{Font, PositionedGlyph, VMetrics};

use crate::{colours::TextColour, shapes::Point2d};

/// A struct containing text rendering information
pub struct TextRenderBuffer<'a> {
    pub width: i32,
    pub height: i32,
    pub baseline: i32,
    pub data: Vec<u32>,
    pub text: &'a str,
}

impl TextRenderBuffer<'_> {
    /// Turn a `TextRenderBuffer` into an image
    pub fn into_image(&'_ self) -> raqote::Image<'_> {
        raqote::Image {
            width: self.width,
            height: self.height,
            data: &self.data,
        }
    }

    /// Render a `TextRenderBuffer` on a canvas at a given position
    pub fn render(&self, canvas: &mut DrawTarget, pos: Point2d) {
        canvas.draw_image_at(
            pos.x,
            pos.y,
            &self.into_image(),
            &DrawOptions {
                blend_mode: raqote::BlendMode::Add,
                alpha: 1.0,
                antialias: raqote::AntialiasMode::Gray,
            },
        );
    }
}

impl std::fmt::Debug for TextRenderBuffer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RenderBuffer {{ {}x{}_{} '{}' }}",
            self.width, self.height, self.baseline, self.text
        )
        .unwrap();
        Ok(())
    }
}

/// The default font used.
pub const DEFAULT_FONT_PATH: &str = "./fonts/hack-regular.ttf"; // Hack font, see: https://sourcefoundry.org/hack/

/// Load a font from a file
pub fn load_font<'a>(path: &str) -> Option<Font<'a>> {
    rusttype::Font::try_from_vec(
        std::fs::read(path)
            .map_err(|err| println!("[ERROR] Failed to read font file: {err}"))
            .ok()?,
    )
}

impl<'a> TextRenderBuffer<'a> {
    /// Create a new `TextRenderBuffer` from a `&str`
    pub fn new(text: &'a str, size: i32, colour: TextColour, font: &Font) -> Option<Self> {
        let scale: rusttype::Scale = rusttype::Scale::uniform(size as f32 * 1.333);
        let vmetrics: VMetrics = font.v_metrics(scale);

        let glyphs: Vec<PositionedGlyph> =
            Vec::from_iter(font.layout(text, scale, rusttype::point(0., 0. + vmetrics.ascent)));

        let mut image: TextRenderBuffer = {
            let width: i32 = {
                let x_min: i32 = glyphs
                    .first()
                    .map(|glyph| glyph.pixel_bounding_box().unwrap().min.x)?;
                let x_max: i32 = glyphs
                    .last()
                    .map(|glyph| glyph.pixel_bounding_box().unwrap().max.x)?;
                x_max - x_min + 1
            };

            let height: i32 = (vmetrics.ascent - vmetrics.descent).ceil() as i32;

            TextRenderBuffer {
                width,
                height,
                baseline: vmetrics.ascent as i32,
                data: vec![0u32; (width * height) as usize],
                text,
            }
        };

        for glyph in glyphs {
            if let Some(bbox) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let idx: usize = ((y as usize + bbox.min.y as usize) * image.width as usize)
                        + (x as usize + bbox.min.x as usize);

                    image.data[idx] = u32::from_le_bytes([
                        (colour.src.a() as f32 * v) as u8,
                        (colour.src.r() as f32 * v) as u8,
                        (colour.src.g() as f32 * v) as u8,
                        (colour.src.b() as f32 * v) as u8,
                    ]);
                });
            }
        }
        Some(image)
    }
}
