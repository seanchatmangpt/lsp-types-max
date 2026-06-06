use crate::*;
use serde::{Deserialize, Serialize};

/// A text edit inserting a snippet.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextEdit {
    pub range: Range,
    pub snippet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_apply: Option<bool>,
}

