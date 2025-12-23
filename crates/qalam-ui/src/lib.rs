//! # qalam-ui
//! واجهة المستخدم لمحرر قلم
//! User interface for Qalam editor

mod app;
mod editor;
mod file_panel;
mod rtl_editor;
mod theme;

pub use app::{Qalam, Message, run};
pub use rtl_editor::{CursorDirection, EditorState, RtlEditorMessage, RtlTextEditor, rtl_text_editor};
pub use theme::Theme;
