//! التطبيق الرئيسي - Main application

use crate::editor::{Editor, EditorMessage};
use crate::file_panel::{FilePanel, FilePanelMessage};
use crate::theme::Theme;
use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};
use std::path::PathBuf;

/// حالة التطبيق - Application state
pub struct Qalam {
    /// لوحة الملفات - File panel
    file_panel: FilePanel,
    /// المحرر - Editor
    editor: Editor,
    /// السمة - Theme
    theme: Theme,
    /// المجلد الحالي - Current directory
    current_dir: Option<PathBuf>,
    /// رسالة الحالة - Status message
    status: String,
}

/// رسائل التطبيق - Application messages
#[derive(Debug, Clone)]
pub enum Message {
    /// رسالة لوحة الملفات
    FilePanel(FilePanelMessage),
    /// رسالة المحرر
    Editor(EditorMessage),
    /// فتح مجلد
    OpenFolder(PathBuf),
    /// فتح ملف
    OpenFile(PathBuf),
    /// حفظ الملف
    Save,
    /// تغيير السمة
    ToggleTheme,
}

impl Qalam {
    /// إنشاء تطبيق جديد - Create new application
    pub fn new() -> (Self, Task<Message>) {
        let mut app = Self {
            file_panel: FilePanel::new(),
            editor: Editor::new(),
            theme: Theme::dark(),
            current_dir: None,
            status: "جاهز".to_string(),
        };

        // فتح المجلد الحالي
        if let Ok(cwd) = std::env::current_dir() {
            app.file_panel.set_root(cwd.clone());
            app.current_dir = Some(cwd);
        }

        (app, Task::none())
    }

    /// عنوان النافذة - Window title
    pub fn title(&self) -> String {
        let file_name = self.editor.document().name();
        let dirty = if self.editor.document().is_dirty() { " *" } else { "" };
        format!("قلم - {}{}", file_name, dirty)
    }

    /// معالجة الرسائل - Handle messages
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FilePanel(msg) => {
                if let Some(path) = self.file_panel.update(msg) {
                    return Task::done(Message::OpenFile(path));
                }
            }
            Message::Editor(msg) => {
                self.editor.update(msg);
            }
            Message::OpenFolder(path) => {
                self.file_panel.set_root(path.clone());
                self.current_dir = Some(path);
                self.status = "تم فتح المجلد".to_string();
            }
            Message::OpenFile(path) => {
                match self.editor.open(path.clone()) {
                    Ok(()) => {
                        self.status = format!("تم فتح: {}", path.display());
                    }
                    Err(e) => {
                        self.status = format!("خطأ: {}", e);
                    }
                }
            }
            Message::Save => {
                match self.editor.save() {
                    Ok(()) => {
                        self.status = "تم الحفظ".to_string();
                    }
                    Err(e) => {
                        self.status = format!("خطأ في الحفظ: {}", e);
                    }
                }
            }
            Message::ToggleTheme => {
                self.theme = if self.theme.name == "داكن" {
                    Theme::light()
                } else {
                    Theme::dark()
                };
            }
        }
        Task::none()
    }

    /// عرض الواجهة - Render UI
    pub fn view(&self) -> Element<Message> {
        // Clone colors needed
        let panel_bg = self.theme.panel_background;
        let border_color = self.theme.border;
        let main_bg = self.theme.background;
        let theme_name = self.theme.name.clone();

        // شريط العنوان
        let title_bar = container(
            row![
                text("قلم").size(18),
                iced::widget::horizontal_space(),
                text(&self.status).size(12),
            ]
            .spacing(16)
            .padding(8)
        )
        .width(Length::Fill)
        .style(move |_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(panel_bg)),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        // المحتوى الرئيسي
        let editor_view = self.editor.view(&self.theme).map(Message::Editor);
        let file_panel_view = self.file_panel.view(&self.theme).map(Message::FilePanel);

        let main_content = row![
            editor_view,
            file_panel_view,
        ];

        // شريط الحالة
        let status_bar = container(
            row![
                text(theme_name).size(12),
                iced::widget::horizontal_space(),
                text("UTF-8").size(12),
                text(" | ").size(12),
                text("ترقيم").size(12),
            ]
            .spacing(8)
            .padding(4)
        )
        .width(Length::Fill)
        .style(move |_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(panel_bg)),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

        // التخطيط الرئيسي
        container(
            column![
                title_bar,
                main_content,
                status_bar,
            ]
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(main_bg)),
            ..Default::default()
        })
        .into()
    }
}

impl Default for Qalam {
    fn default() -> Self {
        Self::new().0
    }
}

/// تشغيل التطبيق - Run application
pub fn run() -> iced::Result {
    iced::application(Qalam::title, Qalam::update, Qalam::view)
        .window_size((1200.0, 800.0))
        .run_with(Qalam::new)
}
