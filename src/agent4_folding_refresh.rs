use crate::request::Request;
use serde::{Deserialize, Serialize};

/// Capabilities specific to the `workspace/foldingRange` request.
///
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeWorkspaceClientCapabilities {
    /// Whether the client implementation supports refreshes for folding ranges
    /// sent from the server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// folding ranges currently shown. It should be used with absolute care
    /// and is useful for situation where a server for example detects a project
    /// wide change that requires a re-calculation of all folding ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_support: Option<bool>,
}

/// The `workspace/foldingRange/refresh` request is sent from the server to the client.
///
/// Servers can use it to ask clients to refresh the editors for which this server
/// provides folding ranges. As a result the client should ask the server to
/// recompute the folding ranges for these editors.
///
/// This is useful if a server detects a project wide configuration change which
/// requires a re-calculation of all folding ranges.
///
/// Note that the client still has the freedom to delay the re-calculation of the
/// folding ranges if for example an editor is currently not visible.
///
/// @since 3.18.0
pub enum FoldingRangeRefresh {}

impl Request for FoldingRangeRefresh {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "workspace/foldingRange/refresh";
}
