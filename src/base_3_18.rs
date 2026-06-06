use serde::{Deserialize, Serialize};
use crate::{
    Uri, TextDocumentRegistrationOptions, StaticRegistrationOptions, GlobPattern
};

/// A language kind.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LanguageKind {
    Known(LanguageKindEnum),
    Custom(std::borrow::Cow<'static, str>),
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LanguageKindEnum {
    Rust,
    TypeScript,
    JavaScript,
}

impl LanguageKind {
    pub const RUST: LanguageKind = LanguageKind::Known(LanguageKindEnum::Rust);
    pub const TYPESCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::TypeScript);
    pub const JAVASCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::JavaScript);

    pub const fn new(kind: &'static str) -> Self {
        LanguageKind::Custom(std::borrow::Cow::Borrowed(kind))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Known(k) => match k {
                LanguageKindEnum::Rust => "rust",
                LanguageKindEnum::TypeScript => "typescript",
                LanguageKindEnum::JavaScript => "javascript",
            },
            Self::Custom(c) => c.as_ref(),
        }
    }
}

impl From<String> for LanguageKind {
    fn from(from: String) -> Self {
        LanguageKind::Custom(std::borrow::Cow::from(from))
    }
}

impl From<&'static str> for LanguageKind {
    fn from(from: &'static str) -> Self {
        LanguageKind::new(from)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditMetadata {
    pub label: Option<String>,
    pub description: Option<String>,
    pub is_auto_apply: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentParams {
    pub text_document: crate::TextDocumentIdentifier,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentResult {
    pub text: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentRefreshParams {
    pub text_documents: Option<Vec<crate::TextDocumentIdentifier>>,
}


#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentClientCapabilities {
    pub dynamic_registration: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentOptions {
    pub schemes: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentRegistrationOptions {
    #[serde(flatten)]
    pub text_document_content_options: TextDocumentContentOptions,
    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,
    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

pub type DocumentUri = crate::Uri;
