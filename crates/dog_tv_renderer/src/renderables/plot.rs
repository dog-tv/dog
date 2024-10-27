// ported from https://github.com/farm-ng/farm-ng-core/tree/main/rs/plotting/src/graphs

/// scalar curve
pub mod scalar_curve;
/// vec curve with confidence interval
pub mod vec_conf_curve;
/// vec curve
pub mod vec_curve;

/// clear condition
#[derive(Copy, Clone, Debug)]
pub struct ClearCondition {
    /// max x range
    pub max_x_range: f64,
}

/// Curve trait
pub trait CurveTrait<DataChunk, Style> {
    /// mut tuples
    fn mut_tuples(&mut self) -> &mut std::collections::VecDeque<(f64, DataChunk)>;

    /// append
    fn append_to(
        &mut self,
        mut new_tuples: std::collections::VecDeque<(f64, DataChunk)>,
        style: Style,
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) {
        self.mut_tuples().append(&mut new_tuples);

        self.drain_filter(clear_cond);

        self.assign_style(style);

        self.update_vline(v_line);
    }

    /// update vline
    fn update_vline(&mut self, v_line: Option<f64>);

    /// assign
    fn assign_style(&mut self, meta: Style);

    /// drain filter
    fn drain_filter(&mut self, pred: ClearCondition) {
        let max_x = self
            .mut_tuples()
            .iter()
            .fold(f64::MIN, |max, p| max.max(p.0));

        self.mut_tuples()
            .retain(|pair| pair.0 + pred.max_x_range > max_x);
    }
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
