use crate::*;
use serde::{Deserialize, Serialize};

/// Client capabilities specific to the `textDocument/rangesFormatting` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangesFormattingClientCapabilities {
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// registration options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

/// The parameters of a `textDocument/rangesFormatting` request.
///
/// @since 3.18.0
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangesFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The ranges to format.
    pub ranges: Vec<Range>,

    /// The format options.
    pub options: FormattingOptions,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,
}

/// Provider options for `textDocument/rangesFormatting`.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangesFormattingOptions {
    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Registration options for `textDocument/rangesFormatting`.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangesFormattingRegistrationOptions {
    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,

    #[serde(flatten)]
    pub document_ranges_formatting_options: DocumentRangesFormattingOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

/// The `textDocument/rangesFormatting` request is sent from the client to the
/// server to format multiple ranges in a given document.
///
/// @since 3.18.0
pub enum DocumentRangesFormatting {}

impl crate::request::Request for DocumentRangesFormatting {
    type Params = DocumentRangesFormattingParams;
    type Result = Option<Vec<TextEdit>>;
    const METHOD: &'static str = "textDocument/rangesFormatting";
}
