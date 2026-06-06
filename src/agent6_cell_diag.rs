use crate::*;
use serde::{Deserialize, Serialize};

/// Client capabilities specific to notebook diagnostic pull requests.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDiagnosticClientCapabilities {
    /// Whether implementation supports dynamic registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_registration: Option<bool>,
}

/// Parameters of the notebook diagnostic request.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDiagnosticParams {
    /// The notebook document for which diagnostics are requested.
    pub notebook_document: NotebookDocumentIdentifier,

    /// The ID of a previous response to allow for delta-based updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_result_id: Option<String>,

    #[serde(flatten)]
    pub work_done_progress_params: WorkDoneProgressParams,

    #[serde(flatten)]
    pub partial_result_params: PartialResultParams,
}

/// A diagnostic report for a specific cell in a notebook.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCellDiagnosticReport {
    /// The URI of the cell's text document.
    pub uri: Uri,

    /// The actual diagnostic report for the cell.
    #[serde(flatten)]
    pub report: DocumentDiagnosticReport,
}

/// A diagnostic report for the notebook document itself.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDocumentDiagnosticReport {
    /// The actual diagnostic report for the notebook.
    #[serde(flatten)]
    pub report: DocumentDiagnosticReport,
}

/// The result of a notebook diagnostic pull request.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDiagnosticReport {
    /// The actual diagnostics for the notebook document and its cells.
    pub items: Vec<OneOf<NotebookCellDiagnosticReport, NotebookDocumentDiagnosticReport>>,
}

/// A partial result for a notebook diagnostic report.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDiagnosticReportPartialResult {
    /// The actual diagnostics for the notebook document and its cells.
    pub items: Vec<OneOf<NotebookCellDiagnosticReport, NotebookDocumentDiagnosticReport>>,
}

/// The `notebookDocument/diagnostic` request is sent from the client to the server
/// to request diagnostics for a notebook document.
/// @since 3.18.0
pub enum NotebookDiagnosticRequest {}

impl crate::request::Request for NotebookDiagnosticRequest {
    type Params = NotebookDiagnosticParams;
    type Result = NotebookDiagnosticReport;
    const METHOD: &'static str = "notebookDocument/diagnostic";
}

/// Notebook diagnostic options.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDiagnosticOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    pub notebook_selector: Vec<NotebookSelector>,

    #[serde(flatten)]
    pub work_done_progress_options: WorkDoneProgressOptions,
}

/// Notebook diagnostic registration options.
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookDiagnosticRegistrationOptions {
    #[serde(flatten)]
    pub notebook_diagnostic_options: NotebookDiagnosticOptions,

    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}
