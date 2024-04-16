use std::cell::RefCell;
use std::f64::consts::TAU;

use deno_core::op2;
use euclid::default::{Box2D, Point2D, Transform2D};
use euclid::{point2, size2, vec2, Angle};
use lyon_geom::{Arc, ArcFlags, SvgArc};
use strum_macros::FromRepr;

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
    fn transform(self, mat: &Transform2D<f64>) -> Self {
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

    pub fn from_svg(path_data: &str) -> Self {
        use svgtypes::{PathParser, PathSegment};

        #[derive(Clone, Copy, Debug)]
        enum ControlPoint {
            None,
            Quad(Point2D<f64>),
            Cubic(Point2D<f64>),
        }

        let mut path = Self::new();
        let mut ctrl = ControlPoint::None;
        let mut cursor = Point2D::zero();
        let mut parser = PathParser::from(path_data);
        while let Some(Ok(segment)) = parser.next() {
            match segment {
                PathSegment::MoveTo { abs, x, y } => {
                    let p = if abs {
                        point2(x, y)
                    } else {
                        cursor + vec2(x, y)
                    };
                    path.do_move_to(p);
                    ctrl = ControlPoint::None;
                    cursor = p;
                }
                PathSegment::LineTo { abs, x, y } => {
                    let p = if abs {
                        point2(x, y)
                    } else {
                        cursor + vec2(x, y)
                    };
                    path.do_line_to(p);
                    ctrl = ControlPoint::None;
                    cursor = p;
                }
                PathSegment::HorizontalLineTo { abs, x } => {
                    let p = point2(if abs { x } else { cursor.x + x }, cursor.y);
                    path.do_line_to(p);
                    ctrl = ControlPoint::None;
                    cursor = p;
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    let p = point2(cursor.x, if abs { y } else { cursor.y + y });
                    path.do_line_to(p);
                    ctrl = ControlPoint::None;
                    cursor = p;
                }
                PathSegment::CurveTo {
                    abs,
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => {
                    let c1 = if abs {
                        point2(x1, y1)
                    } else {
                        cursor + vec2(x1, y1)
                    };
                    let c2 = if abs {
                        point2(x2, y2)
                    } else {
                        cursor + vec2(x2, y2)
                    };
                    let p = if abs {
                        point2(x, y)
                    } else {
                        cursor + vec2(x, y)
                    };
                    path.do_cubic_to(c1, c2, p);
                    ctrl = ControlPoint::Cubic(c2);
                    cursor = p;
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                    let c1 = match ctrl {
                        ControlPoint::Cubic(ctrl) => cursor.lerp(ctrl, -1.0),
                        _ => cursor,
                    };
                    let c2 = if abs {
                        point2(x2, y2)
                    } else {
                        cursor + vec2(x2, y2)
                    };
                    let p = if abs {
                        point2(x, y)
                    } else {
                        cursor + vec2(x, y)
                    };
                    path.do_cubic_to(c1, c2, p);
                    ctrl = ControlPoint::Cubic(c2);
                    cursor = p;
                }
                PathSegment::Quadratic { abs, x1, y1, x, y } => {
                    let c = if abs {
                        point2(x1, y1)
                    } else {
                        cursor + vec2(x1, y1)
                    };
                    let p = if abs {
                        point2(x, y)
                    } else {
                        cursor + vec2(x, y)
                    };
                    path.do_quad_to(c, p);
                    ctrl = ControlPoint::Quad(c);
                    cursor = p;
                }
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    let c = match ctrl {
                        ControlPoint::Quad(ctrl) => cursor.lerp(ctrl, -1.0),
                        _ => cursor,
                    };
                    let p = if abs {
                        point2(x, y)
                    } else {
                        cursor + vec2(x, y)
                    };
                    path.do_quad_to(c, p);
                    ctrl = ControlPoint::Quad(c);
                    cursor = p;
                }
                PathSegment::EllipticalArc {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x,
                    y,
                } => {
                    let arc = SvgArc {
                        from: cursor,
                        to: if abs {
                            point2(x, y)
                        } else {
                            cursor + vec2(x, y)
                        },
                        radii: vec2(rx, ry),
                        x_rotation: Angle::degrees(x_axis_rotation),
                        flags: ArcFlags { large_arc, sweep },
                    };
                    path.do_svg_arc(&arc);
                    ctrl = ControlPoint::None;
                    cursor = arc.to;
                }
                PathSegment::ClosePath { abs: _ } => {
                    path.do_close();
                    ctrl = ControlPoint::None;
                    cursor = path.first_point_in_subpath;
                }
            }
        }
        if !path.ops.is_empty() {
            path.do_move_to(cursor);
        }
        path
    }

    pub fn clear(&mut self) {
        self.ops.clear();
    }

    pub fn transform(self, mat: &Transform2D<f64>) -> Self {
        Self {
            ops: self.ops.into_iter().map(|op| op.transform(mat)).collect(),
            first_point_in_subpath: mat.transform_point(self.first_point_in_subpath),
        }
    }

    pub fn extend(&mut self, path: Path) {
        self.ops.extend(path.ops);
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

    fn do_arc(&mut self, arc: &Arc<f64>) {
        arc.for_each_quadratic_bezier(&mut |s| self.do_quad_to(s.ctrl, s.to));
    }

    fn do_svg_arc(&mut self, arc: &SvgArc<f64>) {
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
        self.do_arc(&arc);
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
        self.do_arc(&arc);
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
        self.do_arc(&Arc {
            center: point2(r.min.x + r1x, r.min.y + r1y),
            radii: vec2(-r1x, -r1y),
            start_angle: Angle::zero(),
            sweep_angle: Angle::frac_pi_2(),
            x_rotation: Angle::zero(),
        });
        self.do_line_to(point2(r.max.x - r2x, r.min.y));
        self.do_arc(&Arc {
            center: point2(r.max.x - r2x, r.min.y + r2y),
            radii: vec2(-r2x, -r2y),
            start_angle: Angle::frac_pi_2(),
            sweep_angle: Angle::frac_pi_2(),
            x_rotation: Angle::zero(),
        });
        self.do_line_to(point2(r.max.x, r.max.y - r3y));
        self.do_arc(&Arc {
            center: point2(r.max.x - r3x, r.max.y - r3y),
            radii: vec2(r3x, r3y),
            start_angle: Angle::zero(),
            sweep_angle: Angle::frac_pi_2(),
            x_rotation: Angle::zero(),
        });
        self.do_line_to(point2(r.min.x + r4x, r.max.y));
        self.do_arc(&Arc {
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
#[cppgc]
pub fn op_canvas_2d_path_new() -> RefCell<Path> {
    RefCell::new(Path::new())
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_path_from_svg(#[string] path_data: &str) -> RefCell<Path> {
    RefCell::new(Path::from_svg(path_data))
}

#[op2]
#[cppgc]
pub fn op_canvas_2d_path_clone(#[cppgc] this: &RefCell<Path>) -> RefCell<Path> {
    this.clone()
}

#[op2(fast)]
pub fn op_canvas_2d_path_clear(#[cppgc] this: &RefCell<Path>) {
    let mut this = this.borrow_mut();
    this.clear()
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_path_extend(
    #[cppgc] this: &RefCell<Path>,
    #[cppgc] path: &RefCell<Path>,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let path = path.borrow().clone();
    let mut this = this.borrow_mut();
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.extend(path.transform(&Transform2D::new(a, b, c, d, e, f)))
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_ensure_subpath(#[cppgc] this: &RefCell<Path>, x: f64, y: f64) {
    let mut this = this.borrow_mut();
    if [x, y].into_iter().all(f64::is_finite) {
        this.ensure_subpath(x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_move_to(#[cppgc] this: &RefCell<Path>, x: f64, y: f64) {
    let mut this = this.borrow_mut();
    if [x, y].into_iter().all(f64::is_finite) {
        this.move_to(x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_line_to(#[cppgc] this: &RefCell<Path>, x: f64, y: f64) {
    let mut this = this.borrow_mut();
    if [x, y].into_iter().all(f64::is_finite) {
        this.line_to(x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_quad_to(#[cppgc] this: &RefCell<Path>, cx: f64, cy: f64, x: f64, y: f64) {
    let mut this = this.borrow_mut();
    if [cx, cy, x, y].into_iter().all(f64::is_finite) {
        this.quad_to(cx, cy, x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_cubic_to(
    #[cppgc] this: &RefCell<Path>,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    x: f64,
    y: f64,
) {
    let mut this = this.borrow_mut();
    if [c1x, c1y, c2x, c2y, x, y].into_iter().all(f64::is_finite) {
        this.cubic_to(c1x, c1y, c2x, c2y, x, y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_arc_to(
    #[cppgc] this: &RefCell<Path>,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    r: f64,
) {
    let mut this = this.borrow_mut();
    if [c1x, c1y, c2x, c2y, r].into_iter().all(f64::is_finite) {
        this.arc_to(c1x, c1y, c2x, c2y, r)
    }
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_path_ellipse(
    #[cppgc] this: &RefCell<Path>,
    x: f64,
    y: f64,
    rx: f64,
    ry: f64,
    start_angle: f64,
    end_angle: f64,
    rotation: f64,
    direction: f64,
) {
    let mut this = this.borrow_mut();
    if [x, y, rx, ry, start_angle, end_angle, rotation, direction]
        .into_iter()
        .all(f64::is_finite)
    {
        this.ellipse(x, y, rx, ry, start_angle, end_angle, rotation, direction)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_rect(#[cppgc] this: &RefCell<Path>, x: f64, y: f64, w: f64, h: f64) {
    let mut this = this.borrow_mut();
    if [x, y, w, h].into_iter().all(f64::is_finite) {
        this.rect(x, y, w, h)
    }
}

#[op2(fast)]
#[allow(clippy::too_many_arguments)]
pub fn op_canvas_2d_path_round_rect(
    #[cppgc] this: &RefCell<Path>,
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
    let mut this = this.borrow_mut();
    if [x, y, w, h, r1x, r1y, r2x, r2y, r3x, r3y, r4x, r4y]
        .into_iter()
        .all(f64::is_finite)
    {
        this.round_rect(x, y, w, h, r1x, r1y, r2x, r2y, r3x, r3y, r4x, r4y)
    }
}

#[op2(fast)]
pub fn op_canvas_2d_path_close(#[cppgc] this: &RefCell<Path>) {
    let mut this = this.borrow_mut();
    this.close()
}
