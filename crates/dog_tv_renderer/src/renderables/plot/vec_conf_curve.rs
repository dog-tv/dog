use std::collections::VecDeque;

use crate::renderables::color::Color;
use crate::renderables::plot::ClearCondition;
use crate::renderables::plot::CurveTrait;

/// VecConfCurve
#[derive(Clone, Debug)]
pub struct VecConfCurve<const N: usize> {
    /// data
    pub data: DataVecDeque<N>,
    /// style
    pub style: VecConfCurveStyle<N>,
    /// clear condition
    pub clear_cond: ClearCondition,

    /// v-line
    pub v_line: Option<f64>,
}

/// VecConfCurve style
#[derive(Copy, Clone, Debug)]
pub struct VecConfCurveStyle<const N: usize> {
    /// color
    pub color: [Color; N],
    /// confidence interval color
    pub conf_color: [Color; N],
}

/// vec conf curve data
pub type DataVecDeque<const N: usize> = VecDeque<(f64, ([f64; N], [f64; N]))>;

impl<const N: usize> VecConfCurve<N> {
    /// Create a new vec curve with confidence intervals
    pub fn new(
        data: DataVecDeque<N>,
        color: [Color; N],
        conf_color: [Color; N],
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) -> Self {
        VecConfCurve {
            data,
            style: VecConfCurveStyle { color, conf_color },
            clear_cond,
            v_line,
        }
    }
}

impl<const N: usize> CurveTrait<([f64; N], [f64; N]), VecConfCurveStyle<N>> for VecConfCurve<N> {
    fn mut_tuples(&mut self) -> &mut std::collections::VecDeque<(f64, ([f64; N], [f64; N]))> {
        &mut self.data
    }

    fn update_vline(&mut self, v_line: Option<f64>) {
        self.v_line = v_line;
    }

    fn assign_style(&mut self, style: VecConfCurveStyle<N>) {
        self.style = style;
    }
}

/// NamedVecConfCurve
#[derive(Clone, Debug)]
pub struct NamedVecConfCurve<const N: usize> {
    /// plot name
    pub plot_name: String,
    /// graph name
    pub graph_name: String,
    /// scalar curve
    pub scalar_curve: VecConfCurve<N>,
}
