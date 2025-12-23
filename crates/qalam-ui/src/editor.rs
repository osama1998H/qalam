//! لوحة المحرر - Editor panel component

use crate::theme::Theme;
use iced::keyboard::{self, Key};
use iced::widget::{column, container, row, text, text_input};
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
    /// قيمة الإدخال الحالية - Current input value
    input_value: String,
}

/// رسائل المحرر - Editor messages
#[derive(Debug, Clone)]
pub enum EditorMessage {
    /// تغيير النص - Text changed
    TextChanged(String),
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
            input_value: String::new(),
        }
    }

    /// فتح ملف - Open file
    pub fn open(&mut self, path: std::path::PathBuf) -> std::io::Result<()> {
        self.document = Document::open(path)?;
        self.input_value = self.document.buffer().text();
        self.rehighlight();
        Ok(())
    }

    /// حفظ الملف - Save file
    pub fn save(&mut self) -> std::io::Result<()> {
        self.document.save()
    }

    /// الحصول على المستند - Get document
    pub fn document(&self) -> &Document {
        &self.document
    }

    /// إعادة التلوين - Rehighlight
    fn rehighlight(&mut self) {
        self.tokens = self.highlighter.highlight(&self.input_value);
    }

    /// معالجة الرسالة - Handle message
    pub fn update(&mut self, message: EditorMessage) {
        match message {
            EditorMessage::TextChanged(new_value) => {
                let buffer = self.document.buffer_mut();
                let _ = buffer.delete(0, buffer.len_chars());
                let _ = buffer.insert(0, &new_value);
                self.input_value = new_value;
                self.rehighlight();
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
    pub fn view(&self, theme: &Theme) -> Element<EditorMessage> {
        // Clone colors needed
        let panel_bg = theme.panel_background;
        let main_bg = theme.background;

        // عرض أرقام الأسطر
        let line_count = self.input_value.lines().count().max(1);
        let line_numbers: String = (1..=line_count)
            .map(|n| format!("{:>4}", n))
            .collect::<Vec<_>>()
            .join("\n");

        let line_numbers_text = text(line_numbers).size(14);

        let line_numbers_container: Element<EditorMessage> = container(line_numbers_text)
            .padding(8)
            .style(move |_: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(panel_bg)),
                ..Default::default()
            })
            .into();

        // محرر النص
        let input = text_input("اكتب الكود هنا...", &self.input_value)
            .on_input(EditorMessage::TextChanged)
            .size(16)
            .padding(12)
            .width(Length::Fill);

        // التخطيط
        let editor_row = row![
            line_numbers_container,
            input,
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
