use std::cell::{Cell, RefCell};
use std::rc::Rc;

use deno_core::{GarbageCollected, op2};
use euclid::default::Transform2D;
use strum_macros::FromRepr;

use super::image_bitmap::ImageBitmap;
use super::wrap::Wrap;
use super::{CanvasColorSpace, raqote_ext};

#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum RepetitionBehavior {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
}

#[derive(Debug)]
pub struct CanvasPattern {
    image: ImageBitmap,
    repetition_behavior: RepetitionBehavior,
    transformation_matrix: Cell<Transform2D<f64>>,
}

impl CanvasPattern {
    pub fn new(image: ImageBitmap, repetition: RepetitionBehavior) -> Self {
        Self {
            image,
            repetition_behavior: repetition,
            transformation_matrix: Cell::new(Transform2D::identity()),
        }
    }

    pub fn set_transform(&self, mat: Transform2D<f64>) {
        self.transformation_matrix.set(mat);
    }

    pub fn to_raqote(
        &self,
        destination_color_space: CanvasColorSpace,
        image_smoothing_enabled: bool,
    ) -> Option<raqote_ext::OwnedSource> {
        let image = self.image.clone().into_color_space(destination_color_space);
        let data = image.data?;
        let width = image.width.try_into().ok()?;
        let height = image.height.try_into().ok()?;
        let inverse_transform = self.transformation_matrix.get().inverse()?;
        Some(raqote_ext::OwnedSource::Image(
            raqote_ext::OwnedImage {
                width,
                height,
                data,
            },
            raqote::ExtendMode::Repeat,
            if image_smoothing_enabled {
                raqote::FilterMode::Bilinear
            } else {
                raqote::FilterMode::Nearest
            },
            inverse_transform.cast(),
            matches!(
                self.repetition_behavior,
                RepetitionBehavior::Repeat | RepetitionBehavior::RepeatX
            ),
            matches!(
                self.repetition_behavior,
                RepetitionBehavior::Repeat | RepetitionBehavior::RepeatY
            ),
        ))
    }
}

impl GarbageCollected for Wrap<Rc<CanvasPattern>> {}

#[op2]
#[cppgc]
pub fn op_canvas_2d_pattern_new(
    #[cppgc] image: &Wrap<RefCell<ImageBitmap>>,
    repetition: i32,
) -> Wrap<Rc<CanvasPattern>> {
    let image = image.take();
    let repetition = RepetitionBehavior::from_repr(repetition).unwrap();
    Wrap::new(Rc::new(CanvasPattern::new(image, repetition)))
}

#[op2(fast)]
pub fn op_canvas_2d_pattern_set_transform(
    #[cppgc] this: &Wrap<Rc<CanvasPattern>>,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    if [a, b, c, d, e, f].into_iter().all(f64::is_finite) {
        this.set_transform(Transform2D::new(a, b, c, d, e, f));
    }
}
