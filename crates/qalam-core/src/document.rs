//! المستند - Document management

use crate::buffer::Buffer;
use crate::selection::Selection;
use std::path::PathBuf;

/// المستند - Document containing buffer and metadata
#[derive(Debug)]
pub struct Document {
    /// مسار الملف - File path (None for untitled)
    path: Option<PathBuf>,
    /// المخزن - Text buffer
    buffer: Buffer,
    /// التحديد - Current selection
    selection: Selection,
    /// تم التعديل - Has unsaved changes
    dirty: bool,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    /// إنشاء مستند جديد فارغ - Create new empty document
    pub fn new() -> Self {
        Self {
            path: None,
            buffer: Buffer::new(),
            selection: Selection::default(),
            dirty: false,
        }
    }

    /// فتح ملف - Open file
    pub fn open(path: PathBuf) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        Ok(Self {
            path: Some(path),
            buffer: Buffer::from_str(&content),
            selection: Selection::default(),
            dirty: false,
        })
    }

    /// حفظ الملف - Save file
    pub fn save(&mut self) -> std::io::Result<()> {
        if let Some(ref path) = self.path {
            std::fs::write(path, self.buffer.text())?;
            self.dirty = false;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "لا يوجد مسار للملف - No file path set",
            ))
        }
    }

    /// حفظ باسم - Save as
    pub fn save_as(&mut self, path: PathBuf) -> std::io::Result<()> {
        std::fs::write(&path, self.buffer.text())?;
        self.path = Some(path);
        self.dirty = false;
        Ok(())
    }

    /// الحصول على المسار - Get path
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// الحصول على اسم الملف - Get file name
    pub fn name(&self) -> String {
        self.path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "بدون عنوان".to_string())
    }

    /// التحقق من وجود تعديلات - Check if dirty
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// الحصول على المخزن - Get buffer reference
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// الحصول على المخزن للتعديل - Get mutable buffer reference
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        self.dirty = true;
        &mut self.buffer
    }

    /// الحصول على التحديد - Get selection
    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    /// الحصول على التحديد للتعديل - Get mutable selection
    pub fn selection_mut(&mut self) -> &mut Selection {
        &mut self.selection
    }

    /// إدراج نص في موضع المؤشر - Insert text at cursor
    pub fn insert(&mut self, text: &str) {
        let pos = self.selection.cursor().position();
        if self.buffer.insert(pos, text).is_ok() {
            self.selection.move_by(text.chars().count() as isize);
            self.dirty = true;
        }
    }

    /// حذف الحرف قبل المؤشر - Delete character before cursor (backspace)
    pub fn backspace(&mut self) {
        let pos = self.selection.cursor().position();
        if pos > 0 {
            if self.buffer.delete(pos - 1, pos).is_ok() {
                self.selection.move_by(-1);
                self.dirty = true;
            }
        }
    }

    /// حذف الحرف بعد المؤشر - Delete character after cursor
    pub fn delete(&mut self) {
        let pos = self.selection.cursor().position();
        if pos < self.buffer.len_chars() {
            if self.buffer.delete(pos, pos + 1).is_ok() {
                self.dirty = true;
            }
        }
    }
}
