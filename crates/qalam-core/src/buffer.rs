//! مخزن النص - Text buffer using rope data structure

use ropey::Rope;
use thiserror::Error;

/// أخطاء المخزن - Buffer errors
#[derive(Error, Debug)]
pub enum BufferError {
    #[error("موضع غير صالح: {0} - Invalid position: {0}")]
    InvalidPosition(usize),
    #[error("نطاق غير صالح - Invalid range")]
    InvalidRange,
}

/// مخزن النص - Text buffer
///
/// يستخدم بنية rope للتعامل الفعال مع النصوص الكبيرة
/// Uses rope data structure for efficient handling of large texts
#[derive(Debug, Clone)]
pub struct Buffer {
    /// النص الداخلي - Internal rope
    rope: Rope,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    /// إنشاء مخزن جديد فارغ - Create new empty buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
        }
    }

    /// إنشاء مخزن من نص - Create buffer from text
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
        }
    }

    /// الحصول على طول النص بالأحرف - Get text length in chars
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// الحصول على طول النص بالبايتات - Get text length in bytes
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// التحقق من أن المخزن فارغ - Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// الحصول على عدد الأسطر - Get line count
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// إدراج نص في موضع معين - Insert text at position
    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<(), BufferError> {
        if char_idx > self.len_chars() {
            return Err(BufferError::InvalidPosition(char_idx));
        }
        self.rope.insert(char_idx, text);
        Ok(())
    }

    /// حذف نطاق من النص - Delete range of text
    pub fn delete(&mut self, start: usize, end: usize) -> Result<(), BufferError> {
        if start > end || end > self.len_chars() {
            return Err(BufferError::InvalidRange);
        }
        self.rope.remove(start..end);
        Ok(())
    }

    /// الحصول على سطر بالفهرس - Get line by index
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.line_count() {
            return None;
        }
        Some(self.rope.line(line_idx).to_string())
    }

    /// الحصول على كل النص - Get all text
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// الحصول على نطاق من النص - Get text range
    pub fn slice(&self, start: usize, end: usize) -> Result<String, BufferError> {
        if start > end || end > self.len_chars() {
            return Err(BufferError::InvalidRange);
        }
        Ok(self.rope.slice(start..end).to_string())
    }

    /// تحويل موضع الحرف إلى سطر وعمود - Convert char position to line and column
    pub fn char_to_line_col(&self, char_idx: usize) -> Option<(usize, usize)> {
        if char_idx > self.len_chars() {
            return None;
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        Some((line, col))
    }

    /// تحويل سطر وعمود إلى موضع الحرف - Convert line and column to char position
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Option<usize> {
        if line >= self.line_count() {
            return None;
        }
        let line_start = self.rope.line_to_char(line);
        let line_len = self.rope.line(line).len_chars();
        if col > line_len {
            return None;
        }
        Some(line_start + col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_text() {
        let mut buffer = Buffer::from_str("مرحبا بالعالم");
        assert_eq!(buffer.len_chars(), 13);

        buffer.insert(6, " ").unwrap();
        assert_eq!(buffer.text(), "مرحبا  بالعالم");
    }

    #[test]
    fn test_mixed_text() {
        let buffer = Buffer::from_str("دالة main() { }");
        assert!(buffer.len_chars() > 0);
    }
}
