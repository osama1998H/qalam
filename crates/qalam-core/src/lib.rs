//! # qalam-core
//! المحرك الأساسي لمحرر قلم
//! Core engine for Qalam Arabic code editor

mod buffer;
mod document;
mod selection;

pub use buffer::Buffer;
pub use document::Document;
pub use selection::{Cursor, Selection};

/// اتجاه النص - Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    /// من اليمين إلى اليسار (العربية)
    #[default]
    RightToLeft,
    /// من اليسار إلى اليمين (الإنجليزية)
    LeftToRight,
}
