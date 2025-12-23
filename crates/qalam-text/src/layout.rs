//! تخطيط النص - Text layout using cosmic-text

use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap};

use crate::bidi::TextDirection;

/// تخطيط النص - Text layout manager
pub struct TextLayout {
    font_system: FontSystem,
    metrics: Metrics,
}

impl TextLayout {
    /// إنشاء تخطيط جديد - Create new layout
    pub fn new() -> Self {
        let mut font_system = FontSystem::new();

        // تحميل الخطوط العربية المتاحة
        // Load available Arabic fonts
        font_system.db_mut().load_system_fonts();

        Self {
            font_system,
            metrics: Metrics::new(16.0, 20.0), // حجم الخط وارتفاع السطر
        }
    }

    /// تعيين حجم الخط - Set font size
    pub fn set_font_size(&mut self, size: f32, line_height: f32) {
        self.metrics = Metrics::new(size, line_height);
    }

    /// تخطيط النص - Layout text
    pub fn layout(&mut self, text: &str, width: f32) -> Vec<LayoutLine> {
        let mut buffer = Buffer::new(&mut self.font_system, self.metrics);

        let attrs = Attrs::new()
            .family(Family::SansSerif);

        buffer.set_size(&mut self.font_system, Some(width), None);
        buffer.set_wrap(&mut self.font_system, Wrap::Word);
        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);

        let mut lines: Vec<LayoutLine> = Vec::new();
        let mut current_y: f32 = 0.0;

        for run in buffer.layout_runs() {
            let direction = if run.rtl {
                TextDirection::RightToLeft
            } else {
                TextDirection::LeftToRight
            };

            let layout_run = LayoutRun {
                text: run.text.to_string(),
                direction,
                x: 0.0, // Will be calculated based on RTL
                y: current_y,
                width: run.line_w,
            };

            // تجميع الـ runs في أسطر
            if lines.is_empty() || (current_y - lines.last().unwrap().y).abs() > 0.1 {
                lines.push(LayoutLine {
                    runs: vec![layout_run.clone()],
                    y: current_y,
                    height: self.metrics.line_height,
                });
            } else {
                lines.last_mut().unwrap().runs.push(layout_run);
            }

            current_y += self.metrics.line_height;
        }

        lines
    }

    /// الحصول على موضع المؤشر - Get cursor position
    pub fn cursor_position(&mut self, text: &str, char_idx: usize, width: f32) -> (f32, f32) {
        let lines = self.layout(text, width);

        let mut current_idx = 0;
        for line in &lines {
            for run in &line.runs {
                let run_len = run.text.chars().count();
                if current_idx + run_len >= char_idx {
                    // المؤشر في هذا الـ run
                    let offset_in_run = char_idx - current_idx;
                    let char_width = if run_len > 0 { run.width / run_len as f32 } else { 0.0 };

                    let x = if run.direction == TextDirection::RightToLeft {
                        run.x + run.width - (offset_in_run as f32 * char_width)
                    } else {
                        run.x + (offset_in_run as f32 * char_width)
                    };

                    return (x, run.y);
                }
                current_idx += run_len;
            }
        }

        (0.0, 0.0)
    }
}

impl Default for TextLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// سطر التخطيط - Layout line
#[derive(Debug, Clone)]
pub struct LayoutLine {
    /// الأجزاء - Runs in this line
    pub runs: Vec<LayoutRun>,
    /// الموضع الرأسي - Y position
    pub y: f32,
    /// الارتفاع - Height
    pub height: f32,
}

/// جزء التخطيط - Layout run
#[derive(Debug, Clone)]
pub struct LayoutRun {
    /// النص - Text
    pub text: String,
    /// الاتجاه - Direction
    pub direction: TextDirection,
    /// الموضع الأفقي - X position
    pub x: f32,
    /// الموضع الرأسي - Y position
    pub y: f32,
    /// العرض - Width
    pub width: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_arabic() {
        let mut layout = TextLayout::new();
        let lines = layout.layout("مرحبا بالعالم", 500.0);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_layout_mixed() {
        let mut layout = TextLayout::new();
        let lines = layout.layout("دالة main() { }", 500.0);
        assert!(!lines.is_empty());
    }
}
