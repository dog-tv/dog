use std::collections::VecDeque;

use crate::renderables::color::Color;
use crate::renderables::plot::CurveTrait;
use crate::renderables::plot::LineType;


/// Scalar curve style
#[derive(Copy, Clone, Debug)]
pub struct ScalarCurveStyle {
    /// color
    pub color: Color,
    /// line type
    pub line_type: LineType,
}

/// Scalar curve
#[derive(Clone, Debug)]
pub struct ScalarCurve {
    /// data
    pub data: VecDeque<(f64, f64)>,
    /// style
    pub style: ScalarCurveStyle,
}

impl ScalarCurve {
    /// Create a new scalar curve
    pub fn new(
        data: VecDeque<(f64, f64)>,
        color: Color,
        line_type: LineType,
    ) -> Self {
        ScalarCurve {
            data,
            style: ScalarCurveStyle { color, line_type },
        }
    }
}

impl CurveTrait<f64, ScalarCurveStyle> for ScalarCurve {
    fn mut_tuples(&mut self) -> &mut std::collections::VecDeque<(f64, f64)> {
        &mut self.data
    }

    fn assign_style(&mut self, style: ScalarCurveStyle) {
        self.style = style;
    }
}

/// Named scalar curve
#[derive(Clone, Debug)]
pub struct NamedScalarCurve {
    /// plot name
    pub plot_name: String,
    /// graph name
    pub graph_name: String,
    /// scalar curve
    pub scalar_curve: ScalarCurve,
}