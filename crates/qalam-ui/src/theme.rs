//! سمات الألوان - Color themes

use iced::Color;

/// السمة - Theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    /// اسم السمة
    pub name: String,
    /// لون الخلفية
    pub background: Color,
    /// لون خلفية اللوحة
    pub panel_background: Color,
    /// لون النص الأساسي
    pub text: Color,
    /// لون النص الثانوي
    pub text_secondary: Color,
    /// لون التحديد
    pub selection: Color,
    /// لون المؤشر
    pub cursor: Color,
    /// لون الحدود
    pub border: Color,
    /// لون الكلمات المفتاحية
    pub keyword: Color,
    /// لون الأنواع
    pub type_color: Color,
    /// لون الدوال
    pub function: Color,
    /// لون النصوص
    pub string: Color,
    /// لون الأرقام
    pub number: Color,
    /// لون التعليقات
    pub comment: Color,
    /// لون الأخطاء
    pub error: Color,
    /// لون التحذيرات
    pub warning: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// السمة الداكنة - Dark theme
    pub fn dark() -> Self {
        Self {
            name: "داكن".to_string(),
            background: Color::from_rgb8(30, 30, 30),
            panel_background: Color::from_rgb8(37, 37, 38),
            text: Color::from_rgb8(212, 212, 212),
            text_secondary: Color::from_rgb8(128, 128, 128),
            selection: Color::from_rgba8(38, 79, 120, 0.5),
            cursor: Color::from_rgb8(255, 255, 255),
            border: Color::from_rgb8(60, 60, 60),
            keyword: Color::from_rgb8(86, 156, 214),    // أزرق
            type_color: Color::from_rgb8(78, 201, 176), // أخضر فاتح
            function: Color::from_rgb8(220, 220, 170),  // أصفر فاتح
            string: Color::from_rgb8(206, 145, 120),    // برتقالي
            number: Color::from_rgb8(181, 206, 168),    // أخضر فاتح
            comment: Color::from_rgb8(106, 153, 85),    // أخضر
            error: Color::from_rgb8(244, 71, 71),       // أحمر
            warning: Color::from_rgb8(255, 204, 0),     // أصفر
        }
    }

    /// السمة الفاتحة - Light theme
    pub fn light() -> Self {
        Self {
            name: "فاتح".to_string(),
            background: Color::from_rgb8(255, 255, 255),
            panel_background: Color::from_rgb8(243, 243, 243),
            text: Color::from_rgb8(0, 0, 0),
            text_secondary: Color::from_rgb8(100, 100, 100),
            selection: Color::from_rgba8(173, 214, 255, 0.5),
            cursor: Color::from_rgb8(0, 0, 0),
            border: Color::from_rgb8(200, 200, 200),
            keyword: Color::from_rgb8(0, 0, 255),       // أزرق
            type_color: Color::from_rgb8(38, 127, 153), // أخضر مائل للأزرق
            function: Color::from_rgb8(121, 94, 38),    // بني
            string: Color::from_rgb8(163, 21, 21),      // أحمر
            number: Color::from_rgb8(9, 134, 88),       // أخضر
            comment: Color::from_rgb8(0, 128, 0),       // أخضر
            error: Color::from_rgb8(255, 0, 0),         // أحمر
            warning: Color::from_rgb8(200, 150, 0),     // أصفر غامق
        }
    }
}
