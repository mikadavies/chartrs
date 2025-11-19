use raqote::{Color, SolidSource};

/// Wrapper for `raqote`'s `SolidSource`
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
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

/// THe type of content, used to select colours from colour palettes (todo)
pub enum ColourContent {
    Background,
    Foreground1,
    Foreground2,
}

/// A wrapper for `raqote`'s `Color`
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

pub struct ColourPalette {
    pub background: SolidColour,
    pub axes: SolidColour,
    pub series1: SolidColour,
    pub series2: SolidColour,
    pub series3: SolidColour,
    pub text1: TextColour,
    pub text2: TextColour,
}

impl Default for ColourPalette {
    fn default() -> Self {
        Self {
            background: SolidColour {
                src: raqote::SolidSource {
                    r: 42,
                    g: 39,
                    b: 63,
                    a: u8::MAX,
                },
            },

            axes: SolidColour {
                src: raqote::SolidSource {
                    r: 144,
                    g: 140,
                    b: 170,
                    a: u8::MAX / 2,
                },
            },
            series1: SolidColour {
                src: raqote::SolidSource {
                    r: 144,
                    g: 140,
                    b: 170,
                    a: u8::MAX,
                },
            },
            series2: SolidColour {
                src: raqote::SolidSource {
                    r: 234,
                    g: 154,
                    b: 151,
                    a: u8::MAX,
                },
            },
            series3: SolidColour {
                src: raqote::SolidSource {
                    r: 234,
                    g: 154,
                    b: 151,
                    a: u8::MAX,
                },
            },
            text1: Default::default(),
            text2: Default::default(),
        }
    }
}
