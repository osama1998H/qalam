//! المؤشر والتحديد - Cursor and selection management

/// المؤشر - Cursor position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cursor {
    /// موضع الحرف - Character position
    position: usize,
}

impl Cursor {
    /// إنشاء مؤشر جديد - Create new cursor
    pub fn new(position: usize) -> Self {
        Self { position }
    }

    /// الحصول على الموضع - Get position
    pub fn position(&self) -> usize {
        self.position
    }

    /// تعيين الموضع - Set position
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    /// تحريك المؤشر - Move cursor by offset
    pub fn move_by(&mut self, offset: isize) {
        if offset < 0 {
            self.position = self.position.saturating_sub((-offset) as usize);
        } else {
            self.position = self.position.saturating_add(offset as usize);
        }
    }
}

/// التحديد - Selection with anchor and head
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// نقطة البداية - Anchor point
    anchor: usize,
    /// نقطة النهاية (المؤشر) - Head point (cursor)
    head: usize,
}

impl Default for Selection {
    fn default() -> Self {
        Self { anchor: 0, head: 0 }
    }
}

impl Selection {
    /// إنشاء تحديد جديد - Create new selection
    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    /// إنشاء مؤشر بدون تحديد - Create cursor (no selection)
    pub fn cursor_at(position: usize) -> Self {
        Self {
            anchor: position,
            head: position,
        }
    }

    /// الحصول على نقطة البداية - Get anchor
    pub fn anchor(&self) -> usize {
        self.anchor
    }

    /// الحصول على نقطة النهاية - Get head
    pub fn head(&self) -> usize {
        self.head
    }

    /// الحصول على المؤشر - Get cursor
    pub fn cursor(&self) -> Cursor {
        Cursor::new(self.head)
    }

    /// التحقق من وجود تحديد - Check if has selection
    pub fn has_selection(&self) -> bool {
        self.anchor != self.head
    }

    /// الحصول على بداية التحديد - Get selection start
    pub fn start(&self) -> usize {
        self.anchor.min(self.head)
    }

    /// الحصول على نهاية التحديد - Get selection end
    pub fn end(&self) -> usize {
        self.anchor.max(self.head)
    }

    /// تحريك المؤشر مع الاحتفاظ بالتحديد - Move head with selection
    pub fn extend_by(&mut self, offset: isize) {
        if offset < 0 {
            self.head = self.head.saturating_sub((-offset) as usize);
        } else {
            self.head = self.head.saturating_add(offset as usize);
        }
    }

    /// تحريك المؤشر بدون تحديد - Move without selection
    pub fn move_by(&mut self, offset: isize) {
        self.extend_by(offset);
        self.anchor = self.head;
    }

    /// تعيين موضع المؤشر - Set cursor position
    pub fn set_cursor(&mut self, position: usize) {
        self.head = position;
        self.anchor = position;
    }

    /// تعيين التحديد - Set selection range
    pub fn set_range(&mut self, anchor: usize, head: usize) {
        self.anchor = anchor;
        self.head = head;
    }

    /// إلغاء التحديد - Collapse selection to cursor
    pub fn collapse(&mut self) {
        self.anchor = self.head;
    }

    /// تحديد الكل من الموضع الحالي - Select all from current
    pub fn select_all(&mut self, buffer_len: usize) {
        self.anchor = 0;
        self.head = buffer_len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_rtl() {
        let mut cursor = Cursor::new(10);
        // في RTL، السهم الأيسر يتحرك للأمام (يزيد الموضع)
        cursor.move_by(1);
        assert_eq!(cursor.position(), 11);
        // السهم الأيمن يتحرك للخلف (ينقص الموضع)
        cursor.move_by(-1);
        assert_eq!(cursor.position(), 10);
    }

    #[test]
    fn test_selection() {
        let mut sel = Selection::cursor_at(5);
        assert!(!sel.has_selection());

        sel.extend_by(3);
        assert!(sel.has_selection());
        assert_eq!(sel.start(), 5);
        assert_eq!(sel.end(), 8);
    }
}
