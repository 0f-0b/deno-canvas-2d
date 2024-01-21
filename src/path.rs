use std::f64::consts::TAU;
use std::ffi::c_void;

use deno_core::{op2, v8, OpState};
use euclid::default::{Box2D, Point2D, Transform2D};
use euclid::{point2, size2, vec2, Angle};
use lyon_geom::Arc;
use strum_macros::FromRepr;

use super::gc::{borrow_v8, borrow_v8_mut, into_v8};

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CanvasFillRule {
    NonZero,
    EvenOdd,
}

impl CanvasFillRule {
    pub fn to_raqote(self) -> raqote::Winding {
        match self {
            Self::NonZero => raqote::Winding::NonZero,
            Self::EvenOdd => raqote::Winding::EvenOdd,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum PathOp {
    MoveTo {
        p: Point2D<f64>,
    },
    LineTo {
        p: Point2D<f64>,
    },
    QuadTo {
        c: Point2D<f64>,
        p: Point2D<f64>,
    },
    CubicTo {
        c1: Point2D<f64>,
        c2: Point2D<f64>,
        p: Point2D<f64>,
    },
    Close,
}

impl PathOp {
    fn transform(self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        let mat = Transform2D::new(a, b, c, d, e, f);
        match self {
            Self::MoveTo { p } => Self::MoveTo {
                p: mat.transform_point(p),
            },
            Self::LineTo { p } => Self::LineTo {
                p: mat.transform_point(p),
            },
            Self::QuadTo { c, p } => Self::QuadTo {
                c: mat.transform_point(c),
                p: mat.transform_point(p),
            },
            Self::CubicTo { c1, c2, p } => Self::CubicTo {
                c1: mat.transform_point(c1),
                c2: mat.transform_point(c2),
                p: mat.transform_point(p),
            },
            Self::Close => Self::Close,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    ops: Vec<PathOp>,
    first_point_in_subpath: Point2D<f64>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            first_point_in_subpath: Point2D::zero(),
        }
    }

    pub fn clear(&mut self) {
        self.ops.clear();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn extend(&mut self, path: Path, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.ops.extend(
            path.ops
                .into_iter()
                .map(move |op| op.transform(a, b, c, d, e, f)),
        );
    }

    fn do_move_to(&mut self, p: Point2D<f64>) {
        self.ops.push(PathOp::MoveTo { p });
        self.first_point_in_subpath = p;
    }

    fn do_line_to(&mut self, p: Point2D<f64>) {
        self.ops.push(PathOp::LineTo { p });
    }

    fn do_quad_to(&mut self, c: Point2D<f64>, p: Point2D<f64>) {
        self.ops.push(PathOp::QuadTo { c, p });
    }

    fn do_cubic_to(&mut self, c1: Point2D<f64>, c2: Point2D<f64>, p: Point2D<f64>) {
        self.ops.push(PathOp::CubicTo { c1, c2, p });
    }

    fn do_arc(&mut self, arc: Arc<f64>) {
        arc.for_each_quadratic_bezier(&mut |s| self.do_quad_to(s.ctrl, s.to));
    }

    fn do_close(&mut self) {
        self.ops.push(PathOp::Close);
    }

    fn last_point_in_subpath(&self) -> Point2D<f64> {
        match *self.ops.last().unwrap() {
            PathOp::MoveTo { p }
            | PathOp::LineTo { p }
            | PathOp::QuadTo { p, .. }
            | PathOp::CubicTo { p, .. } => p,
            PathOp::Close => self.first_point_in_subpath,
        }
    }

    pub fn ensure_subpath(&mut self, x: f64, y: f64) {
        if self.ops.is_empty() {
            self.do_move_to(point2(x, y));
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.do_move_to(point2(x, y));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        if self.ops.is_empty() {
            self.do_move_to(point2(x, y));
        } else {
            self.do_line_to(point2(x, y));
        }
    }

    pub fn quad_to(&mut self, cx: f64, cy: f64, x: f64, y: f64) {
        self.ensure_subpath(cx, cy);
        self.do_quad_to(point2(cx, cy), point2(x, y));
    }

    pub fn cubic_to(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64) {
        self.ensure_subpath(c1x, c1y);
        self.do_cubic_to(point2(c1x, c1y), point2(c2x, c2y), point2(x, y));
    }

    pub fn arc_to(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, r: f64) {
        self.ensure_subpath(c1x, c1y);
        let c1 = point2(c1x, c1y);
        if r == 0.0 {
            self.do_line_to(c1);
            return;
        }
        let c0 = self.last_point_in_subpath();
        let c2 = point2(c2x, c2y);
        let a = c1 - c0;
        let b = c2 - c1;
        let s = a.cross(b);
        if s == 0.0 {
            self.do_line_to(c1);
            return;
        }
        let arc = Arc {
            center: {
                let an = a.normalize();
                let bn = b.normalize();
                c1 + (bn - an) * (r / (1.0 - an.dot(bn).powi(2)).sqrt())
            },
            radii: vec2(r, r),
            start_angle: vec2(0.0, s).angle_to(a),
            sweep_angle: a.angle_to(b),
            x_rotation: Angle::zero(),
        };
        self.do_line_to(arc.from());
        self.do_arc(arc);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn ellipse(
        &mut self,
        x: f64,
        y: f64,
        rx: f64,
        ry: f64,
        start_angle: f64,
        end_angle: f64,
        rotation: f64,
        direction: f64,
    ) {
        fn get_sweep_angle(x: f64) -> f64 {
            if x < 0.0 {
                x % TAU + TAU
            } else {
                x.min(TAU)
            }
        }

        let arc = Arc {
            center: point2(x, y),
            radii: vec2(rx, ry),
            start_angle: Angle::radians(start_angle.rem_euclid(TAU)),
            sweep_angle: Angle::radians(
                get_sweep_angle((end_angle - start_angle) * direction) * direction,
            ),
            x_rotation: Angle::radians(rotation),
        };
        if self.ops.is_empty() {
            self.do_move_to(arc.from());
        } else {
            self.do_line_to(arc.from());
        }
        self.do_arc(arc);
    }

    pub fn rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        let r = Box2D::from_origin_and_size(point2(x, y), size2(w, h));
        self.do_move_to(r.min);
        self.do_line_to(point2(r.max.x, r.min.y));
        self.do_line_to(r.max);
        self.do_line_to(point2(r.min.x, r.max.y));
        self.do_close();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn round_rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        r1x: f64,
        r1y: f64,
        r2x: f64,
        r2y: f64,
        r3x: f64,
        r3y: f64,
        r4x: f64,
        r4y: f64,
    ) {
        let r = Box2D::from_origin_and_size(point2(x, y), size2(w, h));
        self.do_move_to(point2(r.min.x, r.min.y + r1y));
        self.do_arc(Arc {
            center: point2(r.min.x + r1x, r.min.y + r1y),
            radii: vec2(-r1x, -r1y),
            start_angle: Angle::zero(),
            sweep_angle: Angle::frac_pi_2(),
            x_rotation: Angle::zero(),
        });
        self.do_line_to(point2(r.max.x - r2x, r.min.y));
        self.do_arc(Arc {
            center: point2(r.max.x - r2x, r.min.y + r2y),
            radii: vec2(-r2x, -r2y),
            start_angle: Angle::frac_pi_2(),
            sweep_angle: Angle::frac_pi_2(),
            x_rotation: Angle::zero(),
        });
        self.do_line_to(point2(r.max.x, r.max.y - r3y));
        self.do_arc(Arc {
            center: point2(r.max.x - r3x, r.max.y - r3y),
            radii: vec2(r3x, r3y),
            start_angle: Angle::zero(),
            sweep_angle: Angle::frac_pi_2(),
            x_rotation: Angle::zero(),
        });
        self.do_line_to(point2(r.min.x + r4x, r.max.y));
        self.do_arc(Arc {
            center: point2(r.min.x + r4x, r.max.y - r4y),
            radii: vec2(r4x, r4y),
            start_angle: Angle::frac_pi_2(),
            sweep_angle: Angle::frac_pi_2(),
            x_rotation: Angle::zero(),
        });
        self.do_close();
        self.do_move_to(r.min);
    }

    pub fn close(&mut self) {
        if !self.ops.is_empty() {
            self.do_close();
        }
    }

    pub fn to_raqote_ops(&self) -> Vec<raqote::PathOp> {
        self.ops
            .iter()
            .map(|op| match *op {
                PathOp::MoveTo { p } => raqote::PathOp::MoveTo(p.cast()),
                PathOp::LineTo { p } => raqote::PathOp::LineTo(p.cast()),
                PathOp::QuadTo { c, p } => raqote::PathOp::QuadTo(c.cast(), p.cast()),
                PathOp::CubicTo { c1, c2, p } => {
                    raqote::PathOp::CubicTo(c1.cast(), c2.cast(), p.cast())
                }
                PathOp::Close => raqote::PathOp::Close,
            })
            .collect()
    }

    pub fn to_raqote(&self, fill_rule: CanvasFillRule) -> raqote::Path {
        raqote::Path {
            ops: self.to_raqote_ops(),
            winding: fill_rule.to_raqote(),
        }
    }
}

#[op2]
pub fn op_canvas_2d_path_new<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
) -> v8::Local<'a, v8::External> {
    let result = Path::new();
    into_v8(state, scope, result)
}

#[op2]
pub fn op_canvas_2d_path_clone<'a>(
    scope: &mut v8::HandleScope<'a>,
    state: &OpState,
    this: *const c_void,
) -> v8::Local<'a, v8::External> {
    let this = borrow_v8::<Path>(state, this);
    let result = this.clone();
    into_v8(state, scope, result)
}

#[op2(fast)]
pub fn op_canvas_2d_path_clear(state: &OpState, this: *const c_void) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    this.clear()
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_path_extend(
    state: &OpState,
    this: *const c_void,
    path: *const c_void,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let path = borrow_v8::<Path>(state, path).clone();
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.extend(path, a, b, c, d, e, f)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_ensure_subpath(state: &OpState, this: *const c_void, x: f64, y: f64) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [x, y].into_iter().all(f64::is_finite) {
        this.ensure_subpath(x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_move_to(state: &OpState, this: *const c_void, x: f64, y: f64) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [x, y].into_iter().all(f64::is_finite) {
        this.move_to(x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_line_to(state: &OpState, this: *const c_void, x: f64, y: f64) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [x, y].into_iter().all(f64::is_finite) {
        this.line_to(x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_quad_to(
    state: &OpState,
    this: *const c_void,
    cx: f64,
    cy: f64,
    x: f64,
    y: f64,
) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [cx, cy, x, y].into_iter().all(f64::is_finite) {
        this.quad_to(cx, cy, x, y)
    }
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_path_cubic_to(
    state: &OpState,
    this: *const c_void,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    x: f64,
    y: f64,
) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [c1x, c1y, c2x, c2y, x, y].into_iter().all(f64::is_finite) {
        this.cubic_to(c1x, c1y, c2x, c2y, x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_arc_to(
    state: &OpState,
    this: *const c_void,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    r: f64,
) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [c1x, c1y, c2x, c2y, r].into_iter().all(f64::is_finite) {
        this.arc_to(c1x, c1y, c2x, c2y, r)
    }
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_path_ellipse(
    state: &OpState,
    this: *const c_void,
    x: f64,
    y: f64,
    rx: f64,
    ry: f64,
    start_angle: f64,
    end_angle: f64,
    rotation: f64,
    direction: f64,
) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [x, y, rx, ry, start_angle, end_angle, rotation, direction]
        .into_iter()
        .all(f64::is_finite)
    {
        this.ellipse(x, y, rx, ry, start_angle, end_angle, rotation, direction)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_rect(
    state: &OpState,
    this: *const c_void,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [x, y, w, h].into_iter().all(f64::is_finite) {
        this.rect(x, y, w, h)
    }
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_path_round_rect(
    state: &OpState,
    this: *const c_void,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    r1x: f64,
    r1y: f64,
    r2x: f64,
    r2y: f64,
    r3x: f64,
    r3y: f64,
    r4x: f64,
    r4y: f64,
) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    if [x, y, w, h, r1x, r1y, r2x, r2y, r3x, r3y, r4x, r4y]
        .into_iter()
        .all(f64::is_finite)
    {
        this.round_rect(x, y, w, h, r1x, r1y, r2x, r2y, r3x, r3y, r4x, r4y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_close(state: &OpState, this: *const c_void) {
    let mut this = borrow_v8_mut::<Path>(state, this);
    this.close()
}
