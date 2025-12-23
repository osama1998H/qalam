//! ملون أكواد ترقيم - Tarqeem syntax highlighter

/// أنواع الرموز - Token kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// كلمة مفتاحية (دالة، إذا، طالما...)
    Keyword,
    /// نوع بيانات (صحيح، نص، عشري...)
    Type,
    /// دالة
    Function,
    /// متغير
    Variable,
    /// نص (سلسلة حروف)
    String,
    /// رقم
    Number,
    /// تعليق
    Comment,
    /// عامل (=، +، -...)
    Operator,
    /// علامة ترقيم (،؛:...)
    Punctuation,
    /// عادي
    Normal,
}

/// رمز ملون - Highlighted token
#[derive(Debug, Clone)]
pub struct HighlightToken {
    /// النوع - Kind
    pub kind: TokenKind,
    /// البداية - Start byte offset
    pub start: usize,
    /// النهاية - End byte offset
    pub end: usize,
}

/// ملون أكواد ترقيم - Tarqeem highlighter
pub struct TarqeemHighlighter {
    keywords: Vec<&'static str>,
    types: Vec<&'static str>,
}

impl Default for TarqeemHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl TarqeemHighlighter {
    /// الكلمات المفتاحية في ترقيم
    const KEYWORDS: &'static [&'static str] = &[
        // التحكم
        "إذا", "وإلا", "لكل", "طالما", "كرر", "توقف", "استمر", "أرجع",
        // التصريحات
        "دالة", "ثابت", "متغير", "صنف", "واجهة", "وحدة", "استخدم",
        // القيم
        "صحيح", "خطأ", "فارغ", "ذاتي", "هذا",
        // التزامن
        "انتظر", "متزامن",
        // أخرى
        "جديد", "نوع", "مطابقة", "حالة",
        // English aliases
        "if", "else", "for", "while", "loop", "break", "continue", "return",
        "fn", "const", "let", "class", "interface", "mod", "use",
        "true", "false", "null", "self", "this",
        "await", "async", "new", "type", "match", "case",
    ];

    /// أنواع البيانات
    const TYPES: &'static [&'static str] = &[
        "صحيح", "صحيح٨", "صحيح١٦", "صحيح٣٢", "صحيح٦٤",
        "طبيعي", "طبيعي٨", "طبيعي١٦", "طبيعي٣٢", "طبيعي٦٤",
        "عشري", "عشري٣٢", "عشري٦٤",
        "نص", "حرف", "منطقي", "فراغ",
        "مصفوفة", "قائمة", "قاموس",
        // English
        "int", "i8", "i16", "i32", "i64",
        "uint", "u8", "u16", "u32", "u64",
        "float", "f32", "f64",
        "str", "char", "bool", "void",
        "array", "list", "dict",
    ];

    /// إنشاء ملون جديد - Create new highlighter
    pub fn new() -> Self {
        Self {
            keywords: Self::KEYWORDS.to_vec(),
            types: Self::TYPES.to_vec(),
        }
    }

    /// تلوين النص - Highlight text
    pub fn highlight(&self, text: &str) -> Vec<HighlightToken> {
        let mut tokens = Vec::new();
        let mut chars = text.char_indices().peekable();

        while let Some((start, c)) = chars.next() {
            let token = match c {
                // تعليق سطر واحد
                '/' if chars.peek().map(|(_, c)| *c) == Some('/') => {
                    chars.next(); // consume second /
                    let end = self.consume_until(&mut chars, '\n');
                    HighlightToken {
                        kind: TokenKind::Comment,
                        start,
                        end: end.unwrap_or(text.len()),
                    }
                }

                // نص
                '"' | '\'' | '«' => {
                    let end_char = match c {
                        '«' => '»',
                        other => other,
                    };
                    let end = self.consume_string(&mut chars, end_char);
                    HighlightToken {
                        kind: TokenKind::String,
                        start,
                        end: end.unwrap_or(text.len()),
                    }
                }

                // رقم
                '0'..='9' | '٠'..='٩' => {
                    let end = self.consume_number(&mut chars);
                    HighlightToken {
                        kind: TokenKind::Number,
                        start,
                        end,
                    }
                }

                // عوامل
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '%' => {
                    // تحقق من العوامل المركبة
                    let end = if chars.peek().map(|(_, c)| matches!(*c, '=' | '&' | '|' | '<' | '>')).unwrap_or(false) {
                        let (e, _) = chars.next().unwrap();
                        e + 1
                    } else {
                        start + c.len_utf8()
                    };
                    HighlightToken {
                        kind: TokenKind::Operator,
                        start,
                        end,
                    }
                }

                // علامات ترقيم
                '(' | ')' | '{' | '}' | '[' | ']' | '،' | '؛' | ':' | '.' | ',' | ';' => {
                    HighlightToken {
                        kind: TokenKind::Punctuation,
                        start,
                        end: start + c.len_utf8(),
                    }
                }

                // معرّف أو كلمة مفتاحية
                _ if c.is_alphabetic() || c == '_' || is_arabic_letter(c) => {
                    let end = self.consume_identifier(&mut chars, text, start);
                    let word = &text[start..end];

                    let kind = if self.keywords.contains(&word) {
                        TokenKind::Keyword
                    } else if self.types.contains(&word) {
                        TokenKind::Type
                    } else if chars.peek().map(|(_, c)| *c == '(').unwrap_or(false) {
                        TokenKind::Function
                    } else {
                        TokenKind::Variable
                    };

                    HighlightToken { kind, start, end }
                }

                // مسافات - تجاهل
                _ if c.is_whitespace() => continue,

                // أي شيء آخر
                _ => HighlightToken {
                    kind: TokenKind::Normal,
                    start,
                    end: start + c.len_utf8(),
                },
            };

            tokens.push(token);
        }

        tokens
    }

    fn consume_until(
        &self,
        chars: &mut std::iter::Peekable<std::str::CharIndices>,
        target: char,
    ) -> Option<usize> {
        while let Some((i, c)) = chars.next() {
            if c == target {
                return Some(i + c.len_utf8());
            }
        }
        None
    }

    fn consume_string(
        &self,
        chars: &mut std::iter::Peekable<std::str::CharIndices>,
        end_char: char,
    ) -> Option<usize> {
        let mut escaped = false;
        while let Some((i, c)) = chars.next() {
            if escaped {
                escaped = false;
                continue;
            }
            if c == '\\' {
                escaped = true;
                continue;
            }
            if c == end_char {
                return Some(i + c.len_utf8());
            }
        }
        None
    }

    fn consume_number(
        &self,
        chars: &mut std::iter::Peekable<std::str::CharIndices>,
    ) -> usize {
        let mut end = 0;
        while let Some(&(i, c)) = chars.peek() {
            if c.is_ascii_digit() || ('٠'..='٩').contains(&c) || c == '.' || c == '_' {
                end = i + c.len_utf8();
                chars.next();
            } else {
                break;
            }
        }
        end
    }

    fn consume_identifier(
        &self,
        chars: &mut std::iter::Peekable<std::str::CharIndices>,
        text: &str,
        start: usize,
    ) -> usize {
        let mut end = start;
        while let Some(&(i, c)) = chars.peek() {
            if c.is_alphanumeric() || c == '_' || is_arabic_letter(c) {
                end = i + c.len_utf8();
                chars.next();
            } else {
                break;
            }
        }
        if end == start {
            // At least include the first character
            start + text[start..].chars().next().map(|c| c.len_utf8()).unwrap_or(0)
        } else {
            end
        }
    }
}

/// التحقق من أن الحرف عربي
fn is_arabic_letter(c: char) -> bool {
    matches!(c, '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_keywords() {
        let highlighter = TarqeemHighlighter::new();
        let tokens = highlighter.highlight("دالة رئيسية() { }");

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Keyword); // دالة
    }

    #[test]
    fn test_highlight_string() {
        let highlighter = TarqeemHighlighter::new();
        let tokens = highlighter.highlight("اطبع(\"مرحبا\")");

        let string_token = tokens.iter().find(|t| t.kind == TokenKind::String);
        assert!(string_token.is_some());
    }

    #[test]
    fn test_highlight_comment() {
        let highlighter = TarqeemHighlighter::new();
        let tokens = highlighter.highlight("// تعليق\nكود");

        assert_eq!(tokens[0].kind, TokenKind::Comment);
    }
}
