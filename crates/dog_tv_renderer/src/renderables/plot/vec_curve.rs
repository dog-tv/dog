use std::collections::VecDeque;

use crate::renderables::color::Color;
use crate::renderables::plot::CurveTrait;
use crate::renderables::plot::LineType;

/// VecCurve
#[derive(Clone, Debug)]
pub struct VecCurve<const N: usize> {
   /// data
    pub data: VecDeque<(f64, [f64; N])>,
    /// style
    pub style: VecCurveStyle<N>,
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
    pub fn new(data: VecDeque<(f64, [f64; N])>, color: [Color; N], line_type: LineType) -> Self {
        VecCurve {
            data,
            style: VecCurveStyle { color, line_type },
        }
    }
}

impl<const N: usize> CurveTrait<[f64; N], VecCurveStyle<N>> for VecCurve<N> {
    fn mut_tuples(&mut self) -> &mut std::collections::VecDeque<(f64, [f64; N])> {
        &mut self.data
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
