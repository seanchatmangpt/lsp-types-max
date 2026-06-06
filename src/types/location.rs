use super::range::Range;
use crate::uri::Uri;
use serde::{Deserialize, Serialize};

/// Represents a location inside a resource, such as a line inside a text file.
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Hash)]
pub struct Location {
    pub uri: Uri,
    pub range: Range,
}

impl Location {
    pub fn new(uri: Uri, range: Range) -> Location {
        Location { uri, range }
    }
}
