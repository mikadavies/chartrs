/// Basic colour structs and constants
pub mod colours;
/// Rendering to a window using `minifb`, requires the `interactive` feature
pub mod display;
/// All plotting functions and plot types
pub mod plot;
/// Text rendering functions, requires `text` feature
pub mod raqote_text;
/// Shape primitives for plots
pub mod shapes;

pub extern crate euclid;
pub extern crate raqote;
