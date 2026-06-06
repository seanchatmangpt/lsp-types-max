use crate::NumberOrString;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// A request message to describe a request between the client and the server.
/// Every request starts with an internal id.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestMessage {
    /// The protocol version. Always "2.0"
    pub jsonrpc: String,

    /// The request id.
    pub id: NumberOrString,

    /// The method to be invoked.
    pub method: String,

    /// The method's params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// A response message is sent as a result of a request.
#[derive(Debug, PartialEq, Clone)]
pub enum ResponseMessage {
    Success {
        /// The protocol version. Always "2.0"
        jsonrpc: String,
        /// The request id.
        id: Option<NumberOrString>,
        /// The result of a request.
        result: serde_json::Value,
    },
    Error {
        /// The protocol version. Always "2.0"
        jsonrpc: String,
        /// The request id.
        id: Option<NumberOrString>,
        /// The error object in case a request fails.
        error: ResponseError,
    },
}

impl Serialize for ResponseMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ResponseMessage", 3)?;
        match self {
            ResponseMessage::Success {
                jsonrpc,
                id,
                result,
            } => {
                state.serialize_field("jsonrpc", jsonrpc)?;
                state.serialize_field("id", id)?;
                state.serialize_field("result", result)?;
            }
            ResponseMessage::Error { jsonrpc, id, error } => {
                state.serialize_field("jsonrpc", jsonrpc)?;
                state.serialize_field("id", id)?;
                state.serialize_field("error", error)?;
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for ResponseMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let jsonrpc = value
            .get("jsonrpc")
            .and_then(|v| v.as_str())
            .ok_or_else(|| de::Error::missing_field("jsonrpc"))?
            .to_string();

        let id_val = value
            .get("id")
            .ok_or_else(|| de::Error::missing_field("id"))?;
        let id = if id_val.is_null() {
            None
        } else {
            Some(serde_json::from_value(id_val.clone()).map_err(de::Error::custom)?)
        };

        let has_result = value.get("result").is_some();
        let has_error = value.get("error").is_some();

        if has_result && has_error {
            return Err(de::Error::custom(
                "ResponseMessage cannot contain both 'result' and 'error'",
            ));
        } else if !has_result && !has_error {
            return Err(de::Error::custom(
                "ResponseMessage must contain either 'result' or 'error'",
            ));
        }

        if has_result {
            let result = value.get("result").unwrap().clone();
            Ok(ResponseMessage::Success {
                jsonrpc,
                id,
                result,
            })
        } else {
            let error = serde_json::from_value(value.get("error").unwrap().clone())
                .map_err(de::Error::custom)?;
            Ok(ResponseMessage::Error { jsonrpc, id, error })
        }
    }
}

/// A NotificationMessage works like an event and does not send a response back.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotificationMessage {
    /// The protocol version. Always "2.0"
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: String,

    /// The notification's params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// The error object in case a request fails.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    /// A number indicating the error type that occurred.
    pub code: i32,

    /// A string providing a short description of the error.
    pub message: String,

    /// A primitive or structured value that contains additional
    /// information about the error. Can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
