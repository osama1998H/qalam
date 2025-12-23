//! محرر النص RTL المخصص - Custom RTL Text Editor Widget
//!
//! This module implements a custom text editor widget with proper RTL support
//! for Arabic text editing, including correct cursor positioning and selection.

use crate::theme::Theme;
use cosmic_text::FontSystem;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::Quad;
use iced::advanced::text::Renderer as TextRenderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Renderer as AdvancedRenderer, Shell};
use iced::event::Status;
use iced::keyboard::{self, Key};
use iced::mouse::{self, Cursor};
use iced::{Color, Element, Event, Length, Point, Rectangle, Size, Theme as IcedTheme};
use qalam_text::ArabicShaper;
use std::cell::RefCell;

/// حالة المحرر - Editor state
pub struct EditorState {
    /// المحتوى النصي - Text content
    content: String,
    /// موقع المؤشر - Cursor position (character index)
    cursor: usize,
    /// التحديد - Selection (anchor, head)
    selection: Option<(usize, usize)>,
    /// إزاحة التمرير - Scroll offset
    scroll_offset: (f32, f32),
    /// هل المحرر مركز - Is editor focused
    is_focused: bool,
    /// نظام الخطوط - Font system
    font_system: RefCell<FontSystem>,
    /// مشكّل الحروف العربية - Arabic shaper
    shaper: ArabicShaper,
    /// ارتفاع السطر - Line height
    line_height: f32,
    /// حجم الخط - Font size
    font_size: f32,
}

/// رسائل المحرر - Editor messages
#[derive(Debug, Clone)]
pub enum RtlEditorMessage {
    /// إدخال نص - Text input
    TextInput(char),
    /// حذف للخلف - Backspace
    Backspace,
    /// حذف للأمام - Delete
    Delete,
    /// تحريك المؤشر - Cursor movement
    CursorMove(CursorDirection),
    /// نقر الفأرة - Mouse click
    Click(Point),
    /// سحب الفأرة - Mouse drag
    Drag(Point),
    /// تمرير - Scroll
    Scroll(f32, f32),
    /// تركيز - Focus
    Focus,
    /// إلغاء التركيز - Blur
    Blur,
    /// تحديد الكل - Select all
    SelectAll,
    /// إدخال سطر جديد - New line
    NewLine,
}

/// اتجاه تحريك المؤشر - Cursor movement direction
#[derive(Debug, Clone, Copy)]
pub enum CursorDirection {
    /// يمين (بصريًا يسار في RTL)
    Right,
    /// يسار (بصريًا يمين في RTL)
    Left,
    /// أعلى
    Up,
    /// أسفل
    Down,
    /// بداية السطر
    Home,
    /// نهاية السطر
    End,
}

impl EditorState {
    /// إنشاء حالة جديدة - Create new state
    pub fn new() -> Self {
        let font_system = FontSystem::new();
        Self {
            content: String::new(),
            cursor: 0,
            selection: None,
            scroll_offset: (0.0, 0.0),
            is_focused: false,
            font_system: RefCell::new(font_system),
            shaper: ArabicShaper::new(),
            line_height: 24.0,
            font_size: 16.0,
        }
    }

    /// تعيين المحتوى - Set content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.cursor = self.cursor.min(self.content.chars().count());
    }

    /// الحصول على المحتوى - Get content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// الحصول على موقع المؤشر - Get cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// تعيين موقع المؤشر - Set cursor position
    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos.min(self.content.chars().count());
    }

    /// عدد الأسطر - Line count
    pub fn line_count(&self) -> usize {
        self.content.lines().count().max(1)
    }

    /// تحويل موقع الحرف إلى سطر وعمود - Character to line/column
    fn char_to_line_col(&self, char_idx: usize) -> (usize, usize) {
        let mut line = 0;
        let mut col = 0;
        let mut current_idx = 0;

        for ch in self.content.chars() {
            if current_idx >= char_idx {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
            current_idx += 1;
        }
        (line, col)
    }

    /// تحويل سطر وعمود إلى موقع الحرف - Line/column to character
    fn line_col_to_char(&self, line: usize, col: usize) -> usize {
        let mut current_line = 0;
        let mut current_col = 0;
        let mut char_idx = 0;

        for ch in self.content.chars() {
            if current_line == line && current_col == col {
                return char_idx;
            }
            if ch == '\n' {
                if current_line == line {
                    return char_idx;
                }
                current_line += 1;
                current_col = 0;
            } else {
                current_col += 1;
            }
            char_idx += 1;
        }
        char_idx
    }

    /// الحصول على طول السطر - Get line length
    fn line_length(&self, line: usize) -> usize {
        self.content
            .lines()
            .nth(line)
            .map(|l| l.chars().count())
            .unwrap_or(0)
    }

    /// معالجة الرسالة - Handle message
    pub fn update(&mut self, message: RtlEditorMessage) {
        match message {
            RtlEditorMessage::TextInput(ch) => {
                // إدراج الحرف في موقع المؤشر
                let byte_idx = self.char_to_byte_idx(self.cursor);
                self.content.insert(byte_idx, ch);
                self.cursor += 1;
                self.selection = None;
            }
            RtlEditorMessage::Backspace => {
                if self.cursor > 0 {
                    let byte_idx = self.char_to_byte_idx(self.cursor - 1);
                    let next_byte_idx = self.char_to_byte_idx(self.cursor);
                    self.content.drain(byte_idx..next_byte_idx);
                    self.cursor -= 1;
                }
                self.selection = None;
            }
            RtlEditorMessage::Delete => {
                let char_count = self.content.chars().count();
                if self.cursor < char_count {
                    let byte_idx = self.char_to_byte_idx(self.cursor);
                    let next_byte_idx = self.char_to_byte_idx(self.cursor + 1);
                    self.content.drain(byte_idx..next_byte_idx);
                }
                self.selection = None;
            }
            RtlEditorMessage::CursorMove(direction) => {
                self.move_cursor(direction);
            }
            RtlEditorMessage::Click(point) => {
                self.cursor = self.point_to_char(point);
                self.selection = None;
                self.is_focused = true;
            }
            RtlEditorMessage::Drag(point) => {
                let new_pos = self.point_to_char(point);
                if self.selection.is_none() {
                    self.selection = Some((self.cursor, new_pos));
                } else if let Some((anchor, _)) = self.selection {
                    self.selection = Some((anchor, new_pos));
                }
                self.cursor = new_pos;
            }
            RtlEditorMessage::Scroll(dx, dy) => {
                self.scroll_offset.0 += dx;
                self.scroll_offset.1 += dy;
                self.scroll_offset.0 = self.scroll_offset.0.max(0.0);
                self.scroll_offset.1 = self.scroll_offset.1.max(0.0);
            }
            RtlEditorMessage::Focus => {
                self.is_focused = true;
            }
            RtlEditorMessage::Blur => {
                self.is_focused = false;
            }
            RtlEditorMessage::SelectAll => {
                self.selection = Some((0, self.content.chars().count()));
                self.cursor = self.content.chars().count();
            }
            RtlEditorMessage::NewLine => {
                let byte_idx = self.char_to_byte_idx(self.cursor);
                self.content.insert(byte_idx, '\n');
                self.cursor += 1;
                self.selection = None;
            }
        }
    }

    /// تحريك المؤشر - Move cursor
    fn move_cursor(&mut self, direction: CursorDirection) {
        let char_count = self.content.chars().count();
        let (line, col) = self.char_to_line_col(self.cursor);

        match direction {
            // في RTL: السهم الأيمن يحرك المؤشر للخلف (يسار بصريًا)
            CursorDirection::Right => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            // في RTL: السهم الأيسر يحرك المؤشر للأمام (يمين بصريًا)
            CursorDirection::Left => {
                if self.cursor < char_count {
                    self.cursor += 1;
                }
            }
            CursorDirection::Up => {
                if line > 0 {
                    let new_col = col.min(self.line_length(line - 1));
                    self.cursor = self.line_col_to_char(line - 1, new_col);
                }
            }
            CursorDirection::Down => {
                if line < self.line_count().saturating_sub(1) {
                    let new_col = col.min(self.line_length(line + 1));
                    self.cursor = self.line_col_to_char(line + 1, new_col);
                }
            }
            // في RTL: Home يذهب لبداية السطر (الحافة اليمنى)
            CursorDirection::Home => {
                self.cursor = self.line_col_to_char(line, 0);
            }
            // في RTL: End يذهب لنهاية السطر (الحافة اليسرى)
            CursorDirection::End => {
                self.cursor = self.line_col_to_char(line, self.line_length(line));
            }
        }
        self.selection = None;
    }

    /// تحويل موقع الحرف إلى موقع البايت - Character index to byte index
    fn char_to_byte_idx(&self, char_idx: usize) -> usize {
        self.content
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.content.len())
    }

    /// تحويل النقطة إلى موقع الحرف - Point to character index
    fn point_to_char(&self, point: Point) -> usize {
        // حساب السطر من الإحداثي Y
        let line = ((point.y + self.scroll_offset.1) / self.line_height) as usize;
        let line = line.min(self.line_count().saturating_sub(1));

        // الحصول على نص السطر
        let line_text = self.content.lines().nth(line).unwrap_or("");
        let line_len = line_text.chars().count();

        if line_len == 0 {
            return self.line_col_to_char(line, 0);
        }

        // في RTL: X يبدأ من اليمين
        // نحسب العمود من اليمين
        let char_width = self.font_size * 0.6; // تقريب عرض الحرف
        let x_from_right = point.x;
        let col = (x_from_right / char_width) as usize;
        let col = col.min(line_len);

        self.line_col_to_char(line, col)
    }

    /// حساب موقع المؤشر على الشاشة - Calculate cursor screen position
    pub fn cursor_screen_position(&self, width: f32) -> Point {
        let (line, col) = self.char_to_line_col(self.cursor);
        let char_width = self.font_size * 0.6;

        // في RTL: المؤشر في الموقع 0 يكون على الحافة اليمنى
        // كلما زاد الموقع، يتحرك المؤشر يسارًا
        let x = width - (col as f32 * char_width) - 20.0; // 20px padding
        let y = line as f32 * self.line_height;

        Point::new(x.max(0.0), y)
    }

    /// حساب مستطيلات التحديد - Calculate selection rectangles
    pub fn selection_rectangles(&self, width: f32) -> Vec<Rectangle> {
        let Some((anchor, head)) = self.selection else {
            return Vec::new();
        };

        let start = anchor.min(head);
        let end = anchor.max(head);

        let (start_line, start_col) = self.char_to_line_col(start);
        let (end_line, end_col) = self.char_to_line_col(end);

        let char_width = self.font_size * 0.6;
        let mut rects = Vec::new();

        for line in start_line..=end_line {
            let line_len = self.line_length(line);
            let line_y = line as f32 * self.line_height;

            let (sel_start_col, sel_end_col) = if line == start_line && line == end_line {
                (start_col, end_col)
            } else if line == start_line {
                (start_col, line_len)
            } else if line == end_line {
                (0, end_col)
            } else {
                (0, line_len)
            };

            // في RTL: التحديد يرسم من اليمين لليسار
            let x_start = width - (sel_start_col as f32 * char_width) - 20.0;
            let x_end = width - (sel_end_col as f32 * char_width) - 20.0;

            rects.push(Rectangle::new(
                Point::new(x_end, line_y),
                Size::new((x_start - x_end).abs(), self.line_height),
            ));
        }

        rects
    }

    /// تشكيل النص العربي - Shape Arabic text
    pub fn shape_text(&self, text: &str) -> String {
        self.shaper.shape_line(text)
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

/// عنصر محرر RTL - RTL Editor widget
pub struct RtlTextEditor<'a, Message> {
    state: &'a EditorState,
    theme: &'a Theme,
    on_edit: Box<dyn Fn(RtlEditorMessage) -> Message + 'a>,
}

impl<'a, Message> RtlTextEditor<'a, Message> {
    /// إنشاء محرر جديد - Create new editor
    pub fn new<F>(state: &'a EditorState, theme: &'a Theme, on_edit: F) -> Self
    where
        F: Fn(RtlEditorMessage) -> Message + 'a,
    {
        Self {
            state,
            theme,
            on_edit: Box::new(on_edit),
        }
    }
}

impl<'a, Message: Clone> Widget<Message, IcedTheme, iced::Renderer> for RtlTextEditor<'a, Message> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.max();
        layout::Node::new(size)
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut iced::Renderer,
        _theme: &IcedTheme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        // رسم الخلفية
        <iced::Renderer as AdvancedRenderer>::fill_quad(
            renderer,
            Quad {
                bounds,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            self.theme.background,
        );

        // رسم مستطيلات التحديد
        let selection_color = Color::from_rgba(0.3, 0.5, 0.8, 0.4);
        for rect in self.state.selection_rectangles(bounds.width) {
            let adjusted_rect = Rectangle::new(
                Point::new(bounds.x + rect.x, bounds.y + rect.y - self.state.scroll_offset.1),
                rect.size(),
            );
            if adjusted_rect.y + adjusted_rect.height > bounds.y
                && adjusted_rect.y < bounds.y + bounds.height
            {
                <iced::Renderer as AdvancedRenderer>::fill_quad(
                    renderer,
                    Quad {
                        bounds: adjusted_rect,
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    },
                    selection_color,
                );
            }
        }

        // رسم النص - سطر بسطر من اليمين
        let padding = 20.0;
        let mut y = bounds.y - self.state.scroll_offset.1;

        for line_text in self.state.content.lines() {
            if y + self.state.line_height > bounds.y && y < bounds.y + bounds.height {
                // شكل الحروف العربية
                let shaped_text = self.state.shape_text(line_text);

                // رسم النص من اليمين
                let text_x = bounds.x + bounds.width - padding;

                <iced::Renderer as TextRenderer>::fill_text(
                    renderer,
                    iced::advanced::text::Text {
                        content: shaped_text.into(),
                        bounds: Size::new(bounds.width - padding * 2.0, self.state.line_height),
                        size: iced::Pixels(self.state.font_size),
                        line_height: iced::advanced::text::LineHeight::Relative(1.5),
                        font: iced::Font::default(),
                        horizontal_alignment: iced::alignment::Horizontal::Right,
                        vertical_alignment: iced::alignment::Vertical::Top,
                        shaping: iced::advanced::text::Shaping::Advanced,
                        wrapping: iced::advanced::text::Wrapping::None,
                    },
                    Point::new(text_x, y),
                    self.theme.foreground,
                    bounds,
                );
            }
            y += self.state.line_height;
        }

        // Handle empty content - show cursor at start position
        if self.state.content.is_empty() && self.state.is_focused {
            // رسم المؤشر عندما يكون المحرر فارغًا
            let cursor_rect = Rectangle::new(
                Point::new(
                    bounds.x + bounds.width - padding,
                    bounds.y,
                ),
                Size::new(2.0, self.state.line_height),
            );

            <iced::Renderer as AdvancedRenderer>::fill_quad(
                renderer,
                Quad {
                    bounds: cursor_rect,
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                },
                self.theme.cursor,
            );
        } else if self.state.is_focused {
            // رسم المؤشر إذا كان المحرر مركزًا
            let cursor_pos = self.state.cursor_screen_position(bounds.width);
            let cursor_rect = Rectangle::new(
                Point::new(
                    bounds.x + cursor_pos.x,
                    bounds.y + cursor_pos.y - self.state.scroll_offset.1,
                ),
                Size::new(2.0, self.state.line_height),
            );

            if cursor_rect.y + cursor_rect.height > bounds.y
                && cursor_rect.y < bounds.y + bounds.height
            {
                <iced::Renderer as AdvancedRenderer>::fill_quad(
                    renderer,
                    Quad {
                        bounds: cursor_rect,
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    },
                    self.theme.cursor,
                );
            }
        }
    }

    fn on_event(
        &mut self,
        _tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> Status {
        let bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_in(bounds) {
                    let point = Point::new(
                        bounds.width - position.x,
                        position.y + self.state.scroll_offset.1,
                    );
                    shell.publish((self.on_edit)(RtlEditorMessage::Click(point)));
                    return Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if cursor.is_over(bounds) {
                    // يمكن إضافة منطق السحب هنا
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(bounds) {
                    let (dx, dy) = match delta {
                        mouse::ScrollDelta::Lines { x, y } => (x * 20.0, y * 20.0),
                        mouse::ScrollDelta::Pixels { x, y } => (x, y),
                    };
                    shell.publish((self.on_edit)(RtlEditorMessage::Scroll(dx, -dy)));
                    return Status::Captured;
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if self.state.is_focused {
                    match key {
                        Key::Named(keyboard::key::Named::ArrowRight) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::CursorMove(
                                CursorDirection::Right,
                            )));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::ArrowLeft) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::CursorMove(
                                CursorDirection::Left,
                            )));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::ArrowUp) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::CursorMove(
                                CursorDirection::Up,
                            )));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::ArrowDown) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::CursorMove(
                                CursorDirection::Down,
                            )));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::Home) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::CursorMove(
                                CursorDirection::Home,
                            )));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::End) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::CursorMove(
                                CursorDirection::End,
                            )));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::Backspace) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::Backspace));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::Delete) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::Delete));
                            return Status::Captured;
                        }
                        Key::Named(keyboard::key::Named::Enter) => {
                            shell.publish((self.on_edit)(RtlEditorMessage::NewLine));
                            return Status::Captured;
                        }
                        Key::Character(ref c) => {
                            if modifiers.command() && c.as_str() == "a" {
                                shell.publish((self.on_edit)(RtlEditorMessage::SelectAll));
                                return Status::Captured;
                            }
                            // إدخال الحرف
                            if !modifiers.command() && !modifiers.control() {
                                for ch in c.chars() {
                                    shell.publish((self.on_edit)(RtlEditorMessage::TextInput(ch)));
                                }
                                return Status::Captured;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        Status::Ignored
    }
}

impl<'a, Message: Clone + 'a> From<RtlTextEditor<'a, Message>>
    for Element<'a, Message, IcedTheme, iced::Renderer>
{
    fn from(editor: RtlTextEditor<'a, Message>) -> Self {
        Element::new(editor)
    }
}

/// دالة مساعدة لإنشاء المحرر - Helper function to create editor
pub fn rtl_text_editor<'a, Message: Clone + 'a>(
    state: &'a EditorState,
    theme: &'a Theme,
    on_edit: impl Fn(RtlEditorMessage) -> Message + 'a,
) -> RtlTextEditor<'a, Message> {
    RtlTextEditor::new(state, theme, on_edit)
}
