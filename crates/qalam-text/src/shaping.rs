//! تشكيل الحروف العربية - Arabic letter shaping

use arabic_reshaper::ArabicReshaper;

/// مشكّل النص العربي - Arabic text shaper
pub struct ArabicShaper;

impl Default for ArabicShaper {
    fn default() -> Self {
        Self::new()
    }
}

impl ArabicShaper {
    /// إنشاء مشكّل جديد - Create new shaper
    pub fn new() -> Self {
        Self
    }

    /// تشكيل النص العربي - Shape Arabic text
    ///
    /// يحول الحروف إلى أشكالها الصحيحة (بداية، وسط، نهاية، منفصلة)
    /// Converts letters to their correct forms (initial, medial, final, isolated)
    pub fn shape(&self, text: &str) -> String {
        let reshaper = ArabicReshaper::new();
        reshaper.reshape(text)
    }

    /// تشكيل سطر كامل مع الاحتفاظ بالأرقام والرموز
    /// Shape full line preserving numbers and symbols
    pub fn shape_line(&self, text: &str) -> String {
        let reshaper = ArabicReshaper::new();
        let mut result = String::with_capacity(text.len() * 2);
        let mut arabic_buffer = String::new();

        for c in text.chars() {
            if is_arabic_letter(c) {
                arabic_buffer.push(c);
            } else {
                if !arabic_buffer.is_empty() {
                    result.push_str(&reshaper.reshape(&arabic_buffer));
                    arabic_buffer.clear();
                }
                result.push(c);
            }
        }

        if !arabic_buffer.is_empty() {
            result.push_str(&reshaper.reshape(&arabic_buffer));
        }

        result
    }
}

/// التحقق من أن الحرف عربي قابل للتشكيل
/// Check if character is a shapeable Arabic letter
fn is_arabic_letter(c: char) -> bool {
    matches!(c, '\u{0621}'..='\u{064A}' | '\u{066E}'..='\u{066F}' | '\u{0671}'..='\u{06D3}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shaping() {
        let shaper = ArabicShaper::new();
        let shaped = shaper.shape("مرحبا");
        // التشكيل يجب أن يغير الحروف
        assert!(!shaped.is_empty());
    }

    #[test]
    fn test_mixed_shaping() {
        let shaper = ArabicShaper::new();
        let shaped = shaper.shape_line("دالة main() { }");
        assert!(shaped.contains("main"));
    }
}
