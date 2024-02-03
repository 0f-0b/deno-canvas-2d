use palette::bool_mask::LazySelect;
use palette::encoding::gamma::{GammaFn, Number};
use palette::encoding::{FromLinear, IntoLinear, Srgb};
use palette::luma::LumaStandard;
use palette::num::{Arithmetics, MulAdd, MulSub, PartialCmp, Powf, Real};
use palette::rgb::{Primaries, RgbSpace, RgbStandard};
use palette::white_point::{Any, D50, D65};
use palette::{Mat3, Yxy};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayP3;

impl<T: Real> Primaries<T> for DisplayP3 {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.680),
            T::from_f64(0.320),
            T::from_f64(0.2289746),
        )
    }

    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.265),
            T::from_f64(0.690),
            T::from_f64(0.6917385),
        )
    }

    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.150),
            T::from_f64(0.060),
            T::from_f64(0.0792869),
        )
    }
}

impl RgbSpace for DisplayP3 {
    type Primaries = DisplayP3;
    type WhitePoint = D65;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        Some([
            0.4865709, 0.2656677, 0.1982173,
            0.2289746, 0.6917385, 0.0792869,
            0.0000000, 0.0451134, 1.0439444,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        Some([
             2.4934969, -0.9313836, -0.4027108,
            -0.8294890,  1.7626641,  0.0236247,
             0.0358458, -0.0761724,  0.9568845,
        ])
    }
}

impl RgbStandard for DisplayP3 {
    type Space = DisplayP3;
    type TransferFn = Srgb;
}

impl LumaStandard for DisplayP3 {
    type WhitePoint = D65;
    type TransferFn = Srgb;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct A98Rgb;

impl<T: Real> Primaries<T> for A98Rgb {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.6400),
            T::from_f64(0.3300),
            T::from_f64(0.2973450),
        )
    }

    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.2100),
            T::from_f64(0.7100),
            T::from_f64(0.6273636),
        )
    }

    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.1500),
            T::from_f64(0.0600),
            T::from_f64(0.0752915),
        )
    }
}

impl RgbSpace for A98Rgb {
    type Primaries = A98Rgb;
    type WhitePoint = D65;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        Some([
            0.5766690, 0.1855582, 0.1882286,
            0.2973450, 0.6273636, 0.0752915,
            0.0270314, 0.0706889, 0.9913375,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        Some([
             2.0415879, -0.5650070, -0.3447314,
            -0.9692436,  1.8759675,  0.0415551,
             0.0134443, -0.1183624,  1.0151750,
        ])
    }
}

impl RgbStandard for A98Rgb {
    type Space = A98Rgb;
    type TransferFn = GammaFn<F256_563>;
}

impl LumaStandard for A98Rgb {
    type WhitePoint = D65;
    type TransferFn = GammaFn<F256_563>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProphotoRgb;

impl<T: Real> Primaries<T> for ProphotoRgb {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.734699),
            T::from_f64(0.265301),
            T::from_f64(0.2880748),
        )
    }

    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.159597),
            T::from_f64(0.840403),
            T::from_f64(0.7118352),
        )
    }

    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.036598),
            T::from_f64(0.000105),
            T::from_f64(0.0000899),
        )
    }
}

impl RgbSpace for ProphotoRgb {
    type Primaries = ProphotoRgb;
    type WhitePoint = D50;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        Some([
            0.7977666, 0.1351813, 0.0313477,
            0.2880748, 0.7118352, 0.0000899,
            0.0000000, 0.0000000, 0.8251046,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        Some([
            1.3457869, -0.2555721, -0.0511019,
           -0.5446307,  1.5082477,  0.0205274,
            0.0000000,  0.0000000,  1.2119675,
        ])
    }
}

impl RgbStandard for ProphotoRgb {
    type Space = ProphotoRgb;
    type TransferFn = ProphotoRgb;
}

impl LumaStandard for ProphotoRgb {
    type WhitePoint = D50;
    type TransferFn = ProphotoRgb;
}

impl<T> IntoLinear<T, T> for ProphotoRgb
where
    T: Real + Powf + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn into_linear(x: T) -> T {
        x.lt_eq(&T::from_f64(0.03125)).lazy_select(
            || T::from_f64(1.0 / 16.0) * &x,
            || x.clone().powf(T::from_f64(1.8)),
        )
    }
}

impl<T> FromLinear<T, T> for ProphotoRgb
where
    T: Real + Powf + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn from_linear(x: T) -> T {
        x.lt_eq(&T::from_f64(0.001953125)).lazy_select(
            || T::from_f64(16.0) * &x,
            || x.clone().powf(T::from_f64(1.0 / 1.8)),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rec2020;

impl<T: Real> Primaries<T> for Rec2020 {
    fn red() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.708),
            T::from_f64(0.292),
            T::from_f64(0.2627002),
        )
    }

    fn green() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.170),
            T::from_f64(0.797),
            T::from_f64(0.6779981),
        )
    }

    fn blue() -> Yxy<Any, T> {
        Yxy::new(
            T::from_f64(0.131),
            T::from_f64(0.046),
            T::from_f64(0.0593017),
        )
    }
}

impl RgbSpace for Rec2020 {
    type Primaries = Rec2020;
    type WhitePoint = D65;

    #[rustfmt::skip]
    #[inline(always)]
    fn rgb_to_xyz_matrix() -> Option<Mat3<f64>> {
        Some([
            0.6369580, 0.1446169, 0.1688810,
            0.2627002, 0.6779981, 0.0593017,
            0.0000000, 0.0280727, 1.0609851,
        ])
    }

    #[rustfmt::skip]
    #[inline(always)]
    fn xyz_to_rgb_matrix() -> Option<Mat3<f64>> {
        Some([
            1.7166512, -0.3556708, -0.2533663,
           -0.6666844,  1.6164812,  0.0157685,
            0.0176399, -0.0427706,  0.9421031,
        ])
    }
}

impl RgbStandard for Rec2020 {
    type Space = Rec2020;
    type TransferFn = Rec2020;
}

impl LumaStandard for Rec2020 {
    type WhitePoint = D65;
    type TransferFn = Rec2020;
}

impl Rec2020 {
    const ALPHA_M1: f64 = 5.5 * Self::BETA;
    const BETA: f64 = 0.018053968510807806;
}

impl<T> IntoLinear<T, T> for Rec2020
where
    T: Real + Powf + MulAdd + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn into_linear(x: T) -> T {
        x.lt_eq(&T::from_f64(Self::BETA * 4.5)).lazy_select(
            || T::from_f64(1.0 / 4.5) * &x,
            || {
                x.clone()
                    .mul_add(
                        T::from_f64(1.0 / (1.0 + Self::ALPHA_M1)),
                        T::from_f64(Self::ALPHA_M1 / (1.0 + Self::ALPHA_M1)),
                    )
                    .powf(T::from_f64(1.0 / 0.45))
            },
        )
    }
}

impl<T> FromLinear<T, T> for Rec2020
where
    T: Real + Powf + MulSub + Arithmetics + PartialCmp + Clone,
    T::Mask: LazySelect<T>,
{
    #[inline]
    fn from_linear(x: T) -> T {
        x.lt_eq(&T::from_f64(Self::BETA)).lazy_select(
            || T::from_f64(4.5) * &x,
            || {
                x.clone().powf(T::from_f64(0.45)).mul_sub(
                    T::from_f64(1.0 + Self::ALPHA_M1),
                    T::from_f64(Self::ALPHA_M1),
                )
            },
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct F256_563;

impl Number for F256_563 {
    const VALUE: f64 = 256.0 / 563.0;
}
