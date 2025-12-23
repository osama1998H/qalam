//! # qalam-text
//! معالجة النصوص العربية والعرض من اليمين إلى اليسار
//! Arabic text handling and RTL rendering

mod bidi;
mod layout;
mod shaping;

pub use bidi::{BidiProcessor, TextDirection};
pub use layout::{TextLayout, LayoutLine, LayoutRun};
pub use shaping::ArabicShaper;

/// خطأ معالجة النص - Text processing error
#[derive(Debug, thiserror::Error)]
pub enum TextError {
    #[error("خطأ في تهيئة الخطوط - Font initialization error: {0}")]
    FontInit(String),
    #[error("خطأ في تشكيل النص - Text shaping error: {0}")]
    Shaping(String),
}
