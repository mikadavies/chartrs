use raqote::{Color, SolidSource};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct SolidColour {
    pub src: SolidSource,
}

impl SolidColour {
    pub const fn default(content: ColourContent) -> Self {
        match content {
            ColourContent::Background => SolidColour {
                src: raqote::SolidSource {
                    r: 42,
                    g: 39,
                    b: 63,
                    a: u8::MAX,
                },
            },
            ColourContent::Foreground1 => SolidColour {
                src: raqote::SolidSource {
                    r: 144,
                    g: 140,
                    b: 170,
                    a: u8::MAX,
                },
            },
            ColourContent::Foreground2 => SolidColour {
                src: raqote::SolidSource {
                    r: 234,
                    g: 154,
                    b: 151,
                    a: u8::MAX,
                },
            },
        }
    }
}

pub enum ColourContent {
    Background,
    Foreground1,
    Foreground2,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TextColour {
    pub src: raqote::Color,
}

impl Default for TextColour {
    fn default() -> Self {
        Self {
            src: Color::new(u8::MAX, 144, 140, 170),
        }
    }
}
