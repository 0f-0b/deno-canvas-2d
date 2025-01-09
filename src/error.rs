use deno_error::JsError;
use thiserror::Error;

use super::css;

#[derive(Debug, Error, JsError)]
pub enum Canvas2DError {
    #[error("Invalid x: {x}")]
    #[class(range)]
    InvalidX { x: i64 },
    #[error("Invalid y: {y}")]
    #[class(range)]
    InvalidY { y: i64 },
    #[error("Invalid width: {width}")]
    #[class(range)]
    InvalidWidth { width: u64 },
    #[error("Invalid height: {height}")]
    #[class(range)]
    InvalidHeight { height: u64 },
    #[error("Invalid size: {size}")]
    #[class(range)]
    InvalidSize { size: u64 },
    #[error("Invalid bitmap width: {width}")]
    #[class(range)]
    InvalidBitmapWidth { width: u128 },
    #[error("Invalid bitmap height: {height}")]
    #[class(range)]
    InvalidBitmapHeight { height: u128 },
    #[error("Invalid {kind} '{css}': {details}")]
    #[class("DOMExceptionSyntaxError")]
    ParseCss {
        css: String,
        kind: css::ValueKind,
        details: css::SyntaxError,
    },
    #[error("Local font fallback is not supported")]
    #[class("DOMExceptionSyntaxError")]
    UnsupportedLocalFontFallback,
    #[error("Invalid font data")]
    #[class("DOMExceptionSyntaxError")]
    DecodeFont,
    #[error("Invalid font data")]
    #[class("DOMExceptionNetworkError")]
    DecodeFontFromUrl,
    #[error("Unsupported image format '{mime_type}'")]
    #[class("DOMExceptionInvalidStateError")]
    UnsupportedImageFormat { mime_type: String },
    #[error(transparent)]
    #[class("DOMExceptionInvalidStateError")]
    DecodeImage(image::ImageError),
    #[error(transparent)]
    #[class("DOMExceptionEncodingError")]
    EncodeImage(image::ImageError),
}
