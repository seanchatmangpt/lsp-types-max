use crate::Uri;

pub type BaseAny = serde_json::Value;
pub type BaseObject = serde_json::Map<String, serde_json::Value>;
pub type BaseArray = Vec<serde_json::Value>;
pub type URI = Uri;
pub type DocumentUri = Uri;
