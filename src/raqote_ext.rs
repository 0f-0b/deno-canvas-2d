use std::rc::Rc;

use raqote::{
    ExtendMode, FilterMode, Gradient, Image, Point, SolidSource, Source, Spread, Transform,
};

#[derive(Clone, Debug)]
pub struct OwnedImage {
    pub width: i32,
    pub height: i32,
    pub data: Rc<[u32]>,
}

impl OwnedImage {
    pub fn borrow(&self) -> Image {
        Image {
            width: self.width,
            height: self.height,
            data: &self.data,
        }
    }
}

#[derive(Clone)]
pub enum OwnedSource {
    Solid(SolidSource),
    Image(OwnedImage, ExtendMode, FilterMode, Transform, bool, bool),
    #[allow(dead_code)]
    RadialGradient(Gradient, Spread, Transform),
    TwoCircleRadialGradient(Gradient, Spread, Point, f32, Point, f32, Transform),
    LinearGradient(Gradient, Spread, Transform),
    SweepGradient(Gradient, Spread, f32, f32, Transform),
}

impl OwnedSource {
    pub fn new_linear_gradient(
        gradient: Gradient,
        start: Point,
        end: Point,
        spread: Spread,
    ) -> Self {
        match Source::new_linear_gradient(gradient, start, end, spread) {
            Source::LinearGradient(gradient, spread, transform) => {
                Self::LinearGradient(gradient, spread, transform)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub fn new_radial_gradient(
        gradient: Gradient,
        center: Point,
        radius: f32,
        spread: Spread,
    ) -> Self {
        match Source::new_radial_gradient(gradient, center, radius, spread) {
            Source::RadialGradient(gradient, spread, transform) => {
                Self::RadialGradient(gradient, spread, transform)
            }
            _ => unreachable!(),
        }
    }

    pub fn new_two_circle_radial_gradient(
        gradient: Gradient,
        center1: Point,
        radius1: f32,
        center2: Point,
        radius2: f32,
        spread: Spread,
    ) -> Self {
        match Source::new_two_circle_radial_gradient(
            gradient, center1, radius1, center2, radius2, spread,
        ) {
            Source::TwoCircleRadialGradient(gradient, spread, c1, r1, c2, r2, transform) => {
                Self::TwoCircleRadialGradient(gradient, spread, c1, r1, c2, r2, transform)
            }
            _ => unreachable!(),
        }
    }

    pub fn new_sweep_gradient(
        gradient: Gradient,
        center: Point,
        start_angle: f32,
        end_angle: f32,
        spread: Spread,
    ) -> Self {
        match Source::new_sweep_gradient(gradient, center, start_angle, end_angle, spread) {
            Source::SweepGradient(gradient, spread, start_angle, end_angle, transform) => {
                Self::SweepGradient(gradient, spread, start_angle, end_angle, transform)
            }
            _ => unreachable!(),
        }
    }

    pub fn borrow(&self) -> Source {
        match *self {
            Self::Solid(color) => Source::Solid(color),
            Self::Image(ref image, extend, filter, transform, extend_x, extend_y) => Source::Image(
                image.borrow(),
                extend,
                filter,
                transform,
                extend_x,
                extend_y,
            ),
            Self::RadialGradient(ref gradient, spread, transform) => {
                Source::RadialGradient(gradient.clone(), spread, transform)
            }
            Self::TwoCircleRadialGradient(ref gradient, spread, c1, r1, c2, r2, transform) => {
                Source::TwoCircleRadialGradient(gradient.clone(), spread, c1, r1, c2, r2, transform)
            }
            Self::LinearGradient(ref gradient, spread, transform) => {
                Source::LinearGradient(gradient.clone(), spread, transform)
            }
            Self::SweepGradient(ref gradient, spread, start_angle, end_angle, transform) => {
                Source::SweepGradient(gradient.clone(), spread, start_angle, end_angle, transform)
            }
        }
    }
}
