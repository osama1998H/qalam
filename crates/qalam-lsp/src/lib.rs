//! # qalam-lsp
//! عميل بروتوكول خادم اللغة للتكامل مع ترقيم
//! LSP client for Tarqeem language integration

mod client;
mod protocol;

pub use client::{LspClient, LspEvent};
pub use protocol::{Diagnostic, DiagnosticSeverity, Completion, Location};

/// خطأ LSP
#[derive(Debug, thiserror::Error)]
pub enum LspError {
    #[error("فشل بدء الخادم - Failed to start server: {0}")]
    StartFailed(String),
    #[error("فشل الاتصال - Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("خطأ في البروتوكول - Protocol error: {0}")]
    Protocol(String),
    #[error("انتهت مهلة الطلب - Request timeout")]
    Timeout,
}
