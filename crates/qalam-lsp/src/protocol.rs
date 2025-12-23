//! تعريفات بروتوكول LSP - LSP protocol definitions

use serde::{Deserialize, Serialize};

/// مستوى خطورة التشخيص - Diagnostic severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticSeverity {
    /// خطأ
    Error = 1,
    /// تحذير
    Warning = 2,
    /// معلومات
    Information = 3,
    /// تلميح
    Hint = 4,
}

/// نطاق في الملف - Range in file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// البداية - Start position
    pub start: Position,
    /// النهاية - End position
    pub end: Position,
}

/// موضع في الملف - Position in file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// رقم السطر (يبدأ من 0)
    pub line: u32,
    /// رقم العمود (يبدأ من 0)
    pub character: u32,
}

/// التشخيص - Diagnostic (error/warning)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// النطاق - Range
    pub range: Range,
    /// الخطورة - Severity
    pub severity: Option<DiagnosticSeverity>,
    /// الرسالة - Message
    pub message: String,
    /// المصدر - Source (e.g., "tarqeem")
    pub source: Option<String>,
}

/// الموقع - Location in file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// مسار الملف - File URI
    pub uri: String,
    /// النطاق - Range
    pub range: Range,
}

/// عنصر الإكمال - Completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Completion {
    /// التسمية - Label
    pub label: String,
    /// النوع - Kind
    pub kind: Option<CompletionKind>,
    /// التفاصيل - Detail
    pub detail: Option<String>,
    /// الوثائق - Documentation
    pub documentation: Option<String>,
    /// النص للإدراج - Insert text
    pub insert_text: Option<String>,
}

/// نوع الإكمال - Completion kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletionKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Keyword = 14,
    Snippet = 15,
}

/// رسالة تهيئة - Initialize params
#[derive(Debug, Serialize)]
pub struct InitializeParams {
    #[serde(rename = "processId")]
    pub process_id: Option<u32>,
    #[serde(rename = "rootUri")]
    pub root_uri: Option<String>,
    pub capabilities: ClientCapabilities,
}

/// قدرات العميل - Client capabilities
#[derive(Debug, Default, Serialize)]
pub struct ClientCapabilities {
    #[serde(rename = "textDocument")]
    pub text_document: Option<TextDocumentClientCapabilities>,
}

/// قدرات المستند - Text document capabilities
#[derive(Debug, Default, Serialize)]
pub struct TextDocumentClientCapabilities {
    pub completion: Option<CompletionClientCapabilities>,
    pub hover: Option<HoverClientCapabilities>,
}

/// قدرات الإكمال - Completion capabilities
#[derive(Debug, Default, Serialize)]
pub struct CompletionClientCapabilities {
    #[serde(rename = "completionItem")]
    pub completion_item: Option<CompletionItemCapabilities>,
}

/// قدرات عنصر الإكمال
#[derive(Debug, Default, Serialize)]
pub struct CompletionItemCapabilities {
    #[serde(rename = "snippetSupport")]
    pub snippet_support: Option<bool>,
}

/// قدرات التحويم
#[derive(Debug, Default, Serialize)]
pub struct HoverClientCapabilities {
    #[serde(rename = "contentFormat")]
    pub content_format: Option<Vec<String>>,
}

/// رسالة JSON-RPC
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// خطأ JSON-RPC
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}

impl JsonRpcMessage {
    /// إنشاء طلب جديد - Create new request
    pub fn request(id: u64, method: &str, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
        }
    }

    /// إنشاء إشعار - Create notification
    pub fn notification(method: &str, params: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
        }
    }
}
