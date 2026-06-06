use crate::{CompletionItem, InsertTextFormat, InsertTextMode, OneOf, Range};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A default edit range.
///
/// @since 3.17.0
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionListItemDefaultsEditRange {
    pub insert: Range,
    pub replace: Range,
}

/// In many cases the items of an actual completion list share the same
/// value for properties like `commitCharacters` or the range of a text
/// edit. A server can therefore omit those properties from the actual
/// completion items and provide them as defaults for the whole list.
///
/// @since 3.17.0
#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionListItemDefaults {
    /// A default commit character set.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_characters: Option<Vec<String>>,

    /// A default edit range.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_range: Option<OneOf<Range, CompletionListItemDefaultsEditRange>>,

    /// A default insert text format.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<InsertTextFormat>,

    /// A default insert text mode.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_mode: Option<InsertTextMode>,

    /// A default data value.
    ///
    /// @since 3.18.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Represents a collection of [completion items](#CompletionItem) to be presented
/// in the editor.
#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionList {
    /// This list it not complete. Further typing should result in recomputing
    /// this list.
    pub is_incomplete: bool,

    /// In many cases the items of an actual completion list share the same
    /// value for properties like `commitCharacters` or the range of a text
    /// edit. A server can therefore omit those properties from the actual
    /// completion items and provide them as defaults for the whole list.
    ///
    /// @since 3.17.0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_defaults: Option<CompletionListItemDefaults>,

    /// The completion items.
    pub items: Vec<CompletionItem>,
}
