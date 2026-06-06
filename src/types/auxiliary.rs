use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::Uri;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

/// The LSP any type
pub type LSPAny = Value;

/// LSP object definition.
pub type LSPObject = Map<String, Value>;

/// LSP arrays.
pub type LSPArray = Vec<Value>;

/* ----------------- Base Protocol 0.9 aliases ----------------- */

pub type BaseAny = LSPAny;
pub type BaseObject = LSPObject;
pub type BaseArray = LSPArray;
pub type DocumentUri = Uri;
pub type URI = Uri;

/* ----------------- Cancel support ----------------- */

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelParams {
    /// The request id to cancel.
    pub id: NumberOrString,
}
