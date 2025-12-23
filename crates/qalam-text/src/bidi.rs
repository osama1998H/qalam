//! معالجة الاتجاه ثنائي اللغة - Bidirectional text processing

use unicode_bidi::BidiInfo;

/// اتجاه النص - Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    /// من اليمين إلى اليسار
    #[default]
    RightToLeft,
    /// من اليسار إلى اليمين
    LeftToRight,
}

/// معالج الاتجاه ثنائي اللغة - Bidirectional text processor
pub struct BidiProcessor;

impl BidiProcessor {
    /// تحليل النص وإعادة ترتيبه للعرض
    /// Analyze and reorder text for display
    pub fn process(text: &str, base_direction: TextDirection) -> Vec<BidiRun> {
        let level = match base_direction {
            TextDirection::RightToLeft => Some(unicode_bidi::Level::rtl()),
            TextDirection::LeftToRight => Some(unicode_bidi::Level::ltr()),
        };

        let bidi_info = BidiInfo::new(text, level);
        let mut runs = Vec::new();

        for para in &bidi_info.paragraphs {
            let line = para.range.clone();
            let reordered = bidi_info.reorder_line(para, line.clone());

            // Simplified: treat whole paragraph as one run
            let direction = if para.level.is_rtl() {
                TextDirection::RightToLeft
            } else {
                TextDirection::LeftToRight
            };

            runs.push(BidiRun {
                text: reordered.to_string(),
                direction,
                start: para.range.start,
                end: para.range.end,
            });
        }

        runs
    }

    /// إعادة ترتيب السطر للعرض المرئي
    /// Reorder line for visual display
    pub fn reorder_line(text: &str, base_direction: TextDirection) -> String {
        let level = match base_direction {
            TextDirection::RightToLeft => Some(unicode_bidi::Level::rtl()),
            TextDirection::LeftToRight => Some(unicode_bidi::Level::ltr()),
        };

        let bidi_info = BidiInfo::new(text, level);
        if bidi_info.paragraphs.is_empty() {
            return text.to_string();
        }

        let paragraph = &bidi_info.paragraphs[0];
        let line = paragraph.range.clone();

        bidi_info.reorder_line(paragraph, line).to_string()
    }

    /// التحقق من أن الحرف عربي
    /// Check if character is Arabic
    pub fn is_arabic(c: char) -> bool {
        matches!(c,
            '\u{0600}'..='\u{06FF}' |  // Arabic
            '\u{0750}'..='\u{077F}' |  // Arabic Supplement
            '\u{08A0}'..='\u{08FF}' |  // Arabic Extended-A
            '\u{FB50}'..='\u{FDFF}' |  // Arabic Presentation Forms-A
            '\u{FE70}'..='\u{FEFF}'    // Arabic Presentation Forms-B
        )
    }

    /// التحقق من أن النص يحتوي على عربية
    /// Check if text contains Arabic
    pub fn contains_arabic(text: &str) -> bool {
        text.chars().any(Self::is_arabic)
    }

    /// تحديد الاتجاه الأساسي للنص
    /// Detect base direction of text
    pub fn detect_direction(text: &str) -> TextDirection {
        for c in text.chars() {
            if Self::is_arabic(c) {
                return TextDirection::RightToLeft;
            }
            if c.is_alphabetic() && !Self::is_arabic(c) {
                return TextDirection::LeftToRight;
            }
        }
        TextDirection::RightToLeft // الافتراضي للمحرر العربي
    }
}

/// تشغيل ثنائي الاتجاه - Bidirectional run
#[derive(Debug, Clone)]
pub struct BidiRun {
    /// النص - Text content
    pub text: String,
    /// الاتجاه - Direction
    pub direction: TextDirection,
    /// بداية النطاق - Start position in original text
    pub start: usize,
    /// نهاية النطاق - End position in original text
    pub end: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_detection() {
        assert!(BidiProcessor::is_arabic('م'));
        assert!(BidiProcessor::is_arabic('ا'));
        assert!(!BidiProcessor::is_arabic('a'));
        assert!(!BidiProcessor::is_arabic('1'));
    }

    #[test]
    fn test_direction_detection() {
        assert_eq!(
            BidiProcessor::detect_direction("مرحبا"),
            TextDirection::RightToLeft
        );
        assert_eq!(
            BidiProcessor::detect_direction("hello"),
            TextDirection::LeftToRight
        );
        assert_eq!(
            BidiProcessor::detect_direction("مرحبا hello"),
            TextDirection::RightToLeft
        );
    }

    #[test]
    fn test_mixed_text() {
        let runs = BidiProcessor::process("مرحبا hello عالم", TextDirection::RightToLeft);
        assert!(!runs.is_empty());
    }
}
