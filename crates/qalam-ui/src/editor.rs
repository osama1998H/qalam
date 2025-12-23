//! لوحة المحرر - Editor panel component

use crate::rtl_editor::{rtl_text_editor, EditorState, RtlEditorMessage};
use crate::theme::Theme;
use iced::keyboard::{self, Key};
use iced::widget::{container, row, text};
use iced::{Element, Length};
use qalam_core::Document;
use qalam_syntax::{HighlightToken, TarqeemHighlighter};

/// حالة المحرر - Editor state
pub struct Editor {
    /// المستند - Document
    document: Document,
    /// المظهر - Highlighter
    highlighter: TarqeemHighlighter,
    /// رموز ملونة مخزنة - Cached highlight tokens
    tokens: Vec<HighlightToken>,
    /// حالة المحرر RTL - RTL editor state
    editor_state: EditorState,
}

/// رسائل المحرر - Editor messages
#[derive(Debug, Clone)]
pub enum EditorMessage {
    /// رسالة المحرر RTL - RTL editor message
    RtlEditor(RtlEditorMessage),
    /// ضغط مفتاح - Key pressed
    KeyPressed(keyboard::Key, keyboard::Modifiers),
}

impl Editor {
    /// إنشاء محرر جديد - Create new editor
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            highlighter: TarqeemHighlighter::new(),
            tokens: Vec::new(),
            editor_state: EditorState::new(),
        }
    }

    /// فتح ملف - Open file
    pub fn open(&mut self, path: std::path::PathBuf) -> std::io::Result<()> {
        self.document = Document::open(path)?;
        let text = self.document.buffer().text();
        self.editor_state.set_content(text);
        self.rehighlight();
        Ok(())
    }

    /// حفظ الملف - Save file
    pub fn save(&mut self) -> std::io::Result<()> {
        // تحديث المستند من المحرر قبل الحفظ
        let text = self.editor_state.content().to_string();
        let buffer = self.document.buffer_mut();
        let _ = buffer.delete(0, buffer.len_chars());
        let _ = buffer.insert(0, &text);
        self.document.save()
    }

    /// الحصول على المستند - Get document
    pub fn document(&self) -> &Document {
        &self.document
    }

    /// إعادة التلوين - Rehighlight
    fn rehighlight(&mut self) {
        let text = self.editor_state.content();
        self.tokens = self.highlighter.highlight(text);
    }

    /// معالجة الرسالة - Handle message
    pub fn update(&mut self, message: EditorMessage) {
        match message {
            EditorMessage::RtlEditor(rtl_msg) => {
                self.editor_state.update(rtl_msg);
                self.rehighlight();

                // تحديث علامة التعديل
                let text = self.editor_state.content().to_string();
                let buffer = self.document.buffer_mut();
                let _ = buffer.delete(0, buffer.len_chars());
                let _ = buffer.insert(0, &text);
            }
            EditorMessage::KeyPressed(key, modifiers) => {
                if modifiers.command() {
                    if let Key::Character(ref c) = key {
                        if c == "s" {
                            let _ = self.save();
                        }
                    }
                }
            }
        }
    }

    /// عرض المحرر - Render editor
    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, EditorMessage> {
        // Clone colors needed
        let panel_bg = theme.panel_background;
        let main_bg = theme.background;

        // عرض أرقام الأسطر
        let line_count = self.editor_state.line_count();
        let line_numbers: String = (1..=line_count)
            .map(|n| format!("{:>4}", n))
            .collect::<Vec<_>>()
            .join("\n");

        let line_numbers_text = text(line_numbers)
            .size(14)
            .color(theme.line_number);

        let line_numbers_container: Element<'_, EditorMessage> = container(line_numbers_text)
            .padding(8)
            .height(Length::Fill)
            .style(move |_: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(panel_bg)),
                ..Default::default()
            })
            .into();

        // محرر النص RTL المخصص
        let editor = rtl_text_editor(&self.editor_state, theme, EditorMessage::RtlEditor);

        // التخطيط - RTL: editor on left, line numbers on right
        let editor_row = row![
            editor,
            line_numbers_container,
        ];

        container(editor_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(main_bg)),
                ..Default::default()
            })
            .into()
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}
