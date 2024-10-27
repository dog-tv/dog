use crate::renderables::color::Color;
use crate::renderables::plot::ClearCondition;
use crate::renderables::plot::CurveTrait;
use crate::renderables::plot::LineType;
use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;

extern crate alloc;

/// VecCurve
#[derive(Clone, Debug)]
pub struct VecCurve<const N: usize> {
    /// data
    pub data: VecDeque<(f64, [f64; N])>,
    /// style
    pub style: VecCurveStyle<N>,
    /// clear condition
    pub clear_cond: ClearCondition,

    /// v-line
    pub v_line: Option<f64>,
}

/// VecCurve style
#[derive(Copy, Clone, Debug)]
pub struct VecCurveStyle<const N: usize> {
    /// color
    pub color: [Color; N],
    /// line type
    pub line_type: LineType,
}

impl<const N: usize> VecCurve<N> {
    /// Create a new vec curve
    pub fn new(
        data: VecDeque<(f64, [f64; N])>,
        color: [Color; N],
        line_type: LineType,
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) -> Self {
        VecCurve {
            data,
            style: VecCurveStyle { color, line_type },
            clear_cond,
            v_line,
        }
    }
}

impl<const N: usize> CurveTrait<[f64; N], VecCurveStyle<N>> for VecCurve<N> {
    fn mut_tuples(&mut self) -> &mut VecDeque<(f64, [f64; N])> {
        &mut self.data
    }

    fn update_vline(&mut self, v_line: Option<f64>) {
        self.v_line = v_line;
    }

    fn assign_style(&mut self, style: VecCurveStyle<N>) {
        self.style = style;
    }
}

/// NamedVecCurve
#[derive(Clone, Debug)]
pub struct NamedVecCurve<const N: usize> {
    /// plot name
    pub plot_name: String,
    /// graph name
    pub graph_name: String,
    /// scalar curve
    pub scalar_curve: VecCurve<N>,
}
