use crate::{
    colours::SolidColour,
    plot::renewed_framework::{BoundingBox, Plot, PlotConfig, PlotData},
    shapes::Point2d,
};

/// The various marker types/shapes
#[derive(Clone, Copy, Debug)]
pub enum MarkerType {
    /// ■
    Square,
    ///  ●
    Circle,
    /// ▲
    TriangleUp,
    /// ▼
    TriangleDown,
    /// ◆
    Diamond,
    /// +
    CrossVertical,
    /// ×
    CrossDiagonal,
}

/// Scatterplot configuration options
#[derive(Clone, Copy, Debug)]
pub struct ScatterConfig {
    marker: MarkerType,
    marker_size: Option<f32>,
    filled: bool,
    colour: SolidColour,
}

pub fn scatter_plot(
    plot: &mut Plot,
    data: &[PlotData],
    config: &PlotConfig,
    extents: &BoundingBox,
) {
}
