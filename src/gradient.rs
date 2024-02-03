use std::cell::RefCell;
use std::f64::consts::TAU;
use std::ffi::c_void;
use std::rc::Rc;

use deno_core::anyhow::Context as _;
use deno_core::{anyhow, op2, v8, OpState};
use euclid::default::Point2D;
use euclid::{point2, Angle};

use super::css::color::{AbsoluteColor, ComputedColor};
use super::css::{FromCss as _, SyntaxError};
use super::gc::{borrow_v8, into_v8};
use super::{raqote_ext, resolve_color_for_canvas, to_raqote_color, CanvasColorSpace};

#[derive(Clone, Copy, Debug)]
pub enum CanvasGradientStyle {
    Linear {
        start: Point2D<f64>,
        end: Point2D<f64>,
    },
    Radial {
        start_center: Point2D<f64>,
        start_radius: f64,
        end_center: Point2D<f64>,
        end_radius: f64,
    },
    Conic {
        start_angle: Angle<f64>,
        center: Point2D<f64>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct CanvasGradientStop {
    offset: f64,
    color: AbsoluteColor,
}

impl CanvasGradientStop {
    fn to_raqote(self, destination_color_space: CanvasColorSpace) -> raqote::GradientStop {
        raqote::GradientStop {
            position: self.offset as f32,
            color: to_raqote_color(self.color, destination_color_space),
        }
    }
}

#[derive(Debug)]
pub struct CanvasGradient {
    style: CanvasGradientStyle,
    stops: RefCell<Vec<CanvasGradientStop>>,
}

impl CanvasGradient {
    pub fn new_linear(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self {
            style: CanvasGradientStyle::Linear {
                start: point2(x0, y0),
                end: point2(x1, y1),
            },
            stops: RefCell::new(Vec::new()),
        }
    }

    pub fn new_radial(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> Self {
        Self {
            style: CanvasGradientStyle::Radial {
                start_center: point2(x0, y0),
                start_radius: r0,
                end_center: point2(x1, y1),
                end_radius: r1,
            },
            stops: RefCell::new(Vec::new()),
        }
    }

    pub fn new_conic(start_angle: f64, x: f64, y: f64) -> Self {
        Self {
            style: CanvasGradientStyle::Conic {
                start_angle: Angle::radians(start_angle.rem_euclid(TAU)),
                center: point2(x, y),
            },
            stops: RefCell::new(Vec::new()),
        }
    }

    pub fn add_color_stop(&self, offset: f64, color: AbsoluteColor) {
        let mut stops = self.stops.borrow_mut();
        let pos = stops.partition_point(|stop| stop.offset <= offset);
        stops.insert(pos, CanvasGradientStop { offset, color });
    }

    pub fn to_raqote(
        &self,
        destination_color_space: CanvasColorSpace,
    ) -> Option<raqote_ext::OwnedSource> {
        let gradient = {
            let stops = self.stops.borrow();
            if stops.is_empty() {
                return None;
            }
            raqote::Gradient {
                stops: stops
                    .iter()
                    .map(|stop| stop.to_raqote(destination_color_space))
                    .collect(),
            }
        };
        match self.style {
            CanvasGradientStyle::Linear { start, end } => {
                if start == end {
                    return None;
                }
                Some(raqote_ext::OwnedSource::new_linear_gradient(
                    gradient,
                    start.cast(),
                    end.cast(),
                    raqote::Spread::Pad,
                ))
            }
            CanvasGradientStyle::Radial {
                start_center,
                start_radius,
                end_center,
                end_radius,
            } => {
                if start_center == end_center && start_radius == end_radius {
                    return None;
                }
                Some(raqote_ext::OwnedSource::new_two_circle_radial_gradient(
                    gradient,
                    start_center.cast(),
                    start_radius as f32,
                    end_center.cast(),
                    end_radius as f32,
                    raqote::Spread::Pad,
                ))
            }
            CanvasGradientStyle::Conic {
                start_angle,
                center,
            } => {
                let raqote_start_angle = -start_angle.to_degrees() as f32;
                let raqote_end_angle = raqote_start_angle + 360.0;
                Some(raqote_ext::OwnedSource::new_sweep_gradient(
                    gradient,
                    center.cast(),
                    raqote_start_angle,
                    raqote_end_angle,
                    raqote::Spread::Repeat,
                ))
            }
        }
    }
}

#[op2]
pub fn op_canvas_2d_gradient_new_linear<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) -> v8::Local<'a, v8::External> {
    let result = Rc::new(CanvasGradient::new_linear(x0, y0, x1, y1));
    into_v8(state, scope, result)
}

#[op2]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_gradient_new_radial<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    x0: f64,
    y0: f64,
    r0: f64,
    x1: f64,
    y1: f64,
    r1: f64,
) -> v8::Local<'a, v8::External> {
    let result = Rc::new(CanvasGradient::new_radial(x0, y0, r0, x1, y1, r1));
    into_v8(state, scope, result)
}

#[op2]
pub fn op_canvas_2d_gradient_new_conic<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    start_angle: f64,
    x: f64,
    y: f64,
) -> v8::Local<'a, v8::External> {
    let result = Rc::new(CanvasGradient::new_conic(start_angle, x, y));
    into_v8(state, scope, result)
}

#[op2(fast)]
pub fn op_canvas_2d_gradient_add_color_stop(
    state: &OpState,
    this: *const c_void,
    offset: f64,
    #[string] color: &str,
) -> anyhow::Result<()> {
    let this = borrow_v8::<Rc<CanvasGradient>>(state, this);
    let color = ComputedColor::from_css_string(color)
        .map_err(SyntaxError::from)
        .with_context(|| format!("Invalid CSS color '{color}'"))?;
    this.add_color_stop(offset, resolve_color_for_canvas(color));
    Ok(())
}
