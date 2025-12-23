//! # qalam-ui
//! واجهة المستخدم لمحرر قلم
//! User interface for Qalam editor

mod app;
mod editor;
mod file_panel;
mod theme;

pub use app::{Qalam, Message, run};
pub use theme::Theme;
