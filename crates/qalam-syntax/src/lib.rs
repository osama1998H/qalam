//! # qalam-syntax
//! تلوين الأكواد للغة ترقيم
//! Syntax highlighting for Tarqeem language

mod tarqeem;

pub use tarqeem::{TarqeemHighlighter, HighlightToken, TokenKind};

/// خطأ التلوين - Highlighting error
#[derive(Debug, thiserror::Error)]
pub enum SyntaxError {
    #[error("خطأ في التحليل - Parse error: {0}")]
    Parse(String),
}
