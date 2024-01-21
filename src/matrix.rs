use deno_core::error::custom_error;
use deno_core::{anyhow, op2};
use euclid::default::{Transform2D, Transform3D};

use super::css::transform::{parse_and_compute_transform, ComputedTransform};

#[derive(Clone, Copy, Debug)]
pub enum Matrix {
    _2D(Transform2D<f64>),
    _3D(Transform3D<f64>),
}

impl From<Transform2D<f64>> for Matrix {
    fn from(value: Transform2D<f64>) -> Self {
        Self::_2D(value)
    }
}

impl From<Transform3D<f64>> for Matrix {
    fn from(value: Transform3D<f64>) -> Self {
        Self::_3D(value)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_parse_matrix(
    #[string] transform_list: &str,
    #[buffer] out: &mut [f64],
) -> anyhow::Result<bool> {
    let transform = if transform_list.is_empty() {
        ComputedTransform::none()
    } else {
        parse_and_compute_transform(transform_list).map_err(|err| {
            custom_error(
                "DOMExceptionSyntaxError",
                format!(
                    "Invalid CSS transform list '{transform_list}': {} at {}:{}",
                    err.kind,
                    err.location.line + 1,
                    err.location.column
                ),
            )
        })?
    };
    Ok(match transform.to_matrix() {
        Matrix::_2D(m) => {
            out[..6].copy_from_slice(&m.to_array());
            true
        }
        Matrix::_3D(m) => {
            out[..16].copy_from_slice(&m.to_array());
            false
        }
    })
}
