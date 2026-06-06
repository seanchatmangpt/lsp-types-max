use crate::*;
use serde::{Deserialize, Serialize};

/// Client capabilities for a text document content provider.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentClientCapabilities {
    /// Whether implementation supports dynamic registration. If this is set to
    /// `true` the client supports the new `(TextDocumentContentRegistrationOptions & StaticRegistrationOptions)`
    /// for the corresponding server capability as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

/// Text document content options.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentOptions {
    /// The schemes for which the server provides content.
    pub schemes: Vec<String>,
}

/// Text document content registration options.
///
/// @since 3.18.0
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

/// Parameters for the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,
}

/// Result of the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentResult {
    /// The content of the text document.
    pub text: String,
}

/// Parameters for the `workspace/textDocumentContent/refresh` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentContentRefreshParams {
    /// The uis of the text documents to refresh.
    pub text_documents: Vec<TextDocumentIdentifier>,
}

/// The `workspace/textDocumentContent` request is sent from the client to the server
/// to request the content of a text document.
///
/// @since 3.18.0
#[derive(Debug)]
pub enum TextDocumentContentRequest {}

impl crate::request::Request for TextDocumentContentRequest {
    type Params = TextDocumentContentParams;
    type Result = TextDocumentContentResult;
    const METHOD: &'static str = "workspace/textDocumentContent";
}

/// The `workspace/textDocumentContent/refresh` request is sent from the server to the client.
/// Servers can use it to ask clients to refresh the content for a given text document.
///
/// @since 3.18.0
#[derive(Debug)]
pub enum TextDocumentContentRefreshRequest {}

impl crate::request::Request for TextDocumentContentRefreshRequest {
    type Params = TextDocumentContentRefreshParams;
    type Result = ();
    const METHOD: &'static str = "workspace/textDocumentContent/refresh";
}
