// ported from https://github.com/farm-ng/farm-ng-core/tree/main/rs/plotting/src/graphs

/// scalar curve
pub mod scalar_curve;
/// vec curve
pub mod vec_curve;
/// vec curve with confidence interval
pub mod vec_conf_curve;


/// Curve trait
pub trait CurveTrait<DataChunk, Style> {

    /// Get mutable reference to the data tuples
    fn mut_tuples(&mut self) -> &mut std::collections::VecDeque<(f64, DataChunk)>;

    /// Append new tuples to the data
    fn append(
        &mut self,
        mut new_tuples: std::collections::VecDeque<(f64, DataChunk)>,
        style: Style,
    ) {

        self.mut_tuples().append(&mut new_tuples);

        self.assign_style(style);
    }

    /// Assign style
    fn assign_style(&mut self, meta: Style);

}

/// Line type
#[derive(Copy, Clone, Debug, Default)]
pub enum LineType {
    #[default]
    /// Solid line
    LineStrip,
    /// Points
    Points,
}
