use thiserror::Error;

use super::css;

#[derive(Debug, Error)]
pub enum Canvas2DError {
    #[error("Invalid x: {x}")]
    InvalidX { x: i64 },
    #[error("Invalid y: {y}")]
    InvalidY { y: i64 },
    #[error("Invalid width: {width}")]
    InvalidWidth { width: u64 },
    #[error("Invalid height: {height}")]
    InvalidHeight { height: u64 },
    #[error("Invalid size: {size}")]
    InvalidSize { size: u64 },
    #[error("Invalid bitmap width: {width}")]
    InvalidBitmapWidth { width: u128 },
    #[error("Invalid bitmap height: {height}")]
    InvalidBitmapHeight { height: u128 },
    #[error("Invalid {kind} '{css}': {details}")]
    ParseCss {
        css: String,
        kind: css::ValueKind,
        details: css::SyntaxError,
    },
    #[error("Local font fallback is not supported")]
    UnsupportedLocalFontFallback,
    #[error("Invalid font data")]
    DecodeFont { from_url: bool },
    #[error("Unsupported image format '{mime_type}'")]
    UnsupportedImageFormat { mime_type: String },
    #[error(transparent)]
    DecodeImage(image::ImageError),
    #[error(transparent)]
    EncodeImage(image::ImageError),
}

pub fn get_canvas_2d_error_class(e: &Canvas2DError) -> &'static str {
    use Canvas2DError::*;

    match *e {
        InvalidX { .. }
        | InvalidY { .. }
        | InvalidWidth { .. }
        | InvalidHeight { .. }
        | InvalidSize { .. }
        | InvalidBitmapWidth { .. }
        | InvalidBitmapHeight { .. } => "RangeError",
        ParseCss { .. } | UnsupportedLocalFontFallback => "DOMExceptionSyntaxError",
        DecodeFont { from_url } => {
            if from_url {
                "DOMExceptionNetworkError"
            } else {
                "DOMExceptionSyntaxError"
            }
        }
        UnsupportedImageFormat { .. } | DecodeImage(_) => "DOMExceptionInvalidStateError",
        EncodeImage(_) => "DOMExceptionEncodingError",
    }
}
