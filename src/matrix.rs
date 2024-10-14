use deno_core::op2;
use euclid::default::{Transform2D, Transform3D};

use super::css::transform::ComputedTransform;
use super::css::{self, FromCss as _};
use super::error::Canvas2DError;

#[derive(Clone, Copy, Debug)]
pub enum Matrix {
    _2D(Transform2D<f32>),
    _3D(Transform3D<f32>),
}

impl From<Transform2D<f32>> for Matrix {
    fn from(value: Transform2D<f32>) -> Self {
        Self::_2D(value.cast())
    }
}

impl From<Transform3D<f32>> for Matrix {
    fn from(value: Transform3D<f32>) -> Self {
        Self::_3D(value.cast())
    }
}

#[op2(fast)]
pub fn op_canvas_2d_parse_matrix(
    #[string] transform_list: &str,
    #[buffer] out: &mut [f64],
) -> Result<bool, Canvas2DError> {
    let transform = if transform_list.is_empty() {
        ComputedTransform::default()
    } else {
        ComputedTransform::from_css_string(transform_list).map_err(|e| Canvas2DError::ParseCss {
            css: transform_list.to_owned(),
            kind: css::ValueKind::TransformList,
            details: css::SyntaxError::from(e),
        })?
    };
    Ok(match transform.to_matrix() {
        Matrix::_2D(m) => {
            out[..6].copy_from_slice(&m.cast().to_array());
            true
        }
        Matrix::_3D(m) => {
            out[..16].copy_from_slice(&m.cast().to_array());
            false
        }
    })
}
