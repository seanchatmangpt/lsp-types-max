use serde::{Deserialize, Serialize};

pub use crate::NumberOrString;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct CancelParams {
    /// The request id to cancel.
    pub id: NumberOrString,
}

/// The LSP any type
///
/// @since 3.17.0
pub type LSPAny = serde_json::Value;

/// LSP object definition.
///
/// @since 3.17.0
pub type LSPObject = serde_json::Map<String, serde_json::Value>;

/// LSP arrays.
///
/// @since 3.17.0
pub type LSPArray = Vec<serde_json::Value>;
