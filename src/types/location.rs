use serde::{Deserialize, Serialize};
use crate::Uri;
use crate::types::range::Range;

/// Represents a location inside a resource, such as a line inside a text file.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub uri: Uri,
    pub range: Range,
}

impl Location {
    pub fn new(uri: Uri, range: Range) -> Self {
        Location { uri, range }
    }
}
