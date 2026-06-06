use crate::GlobPattern;
use serde::{Deserialize, Serialize};

/// Client capabilities specific to the `MessageType.Debug` and `Relative Glob Patterns`.

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent10Capabilities {
    /// Window specific client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<Agent10WindowClientCapabilities>,
    /// General client capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<Agent10GeneralClientCapabilities>,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent10WindowClientCapabilities {
    /// Whether the client supports the new `MessageType.Debug`.
    /// @since 3.18.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type_debug_support: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent10GeneralClientCapabilities {
    /// Client capability that signals how the client handles relative glob patterns.
    /// @since 3.18.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_pattern_support: Option<bool>,
}

/// Extension of DocumentFilter to support RelativePattern in 3.18.0.
/// In LSP 3.18.0, the `pattern` field can now be a `GlobPattern` (String | RelativePattern).
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFilterExt {
    /// A language id, like `typescript`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// A Uri [scheme](#Uri.scheme), like `file` or `untitled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    /// A glob pattern, like `*.{ts,js}`.
    /// @since 3.18.0 support for relative patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<GlobPattern>,
}

// MessageType::DEBUG is not defined here.
