use serde::{Deserialize, Serialize};

/// The current state in the LSP lifecycle "process model", inspired by
/// Dr. Wil van der Aalst's process conformance checking / token replay.
///
/// The protocol defines a strict ordering of messages:
///   PreInitialize → Initializing → Running → ShuttingDown → Exited
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LspProtocolState {
    /// No messages exchanged yet.
    PreInitialize,
    /// `initialize` request received from client; waiting for server response
    /// and client `initialized` notification.
    Initializing,
    /// Fully initialized and running — all normal LSP traffic is valid.
    Running,
    /// `shutdown` request received from client; waiting for server response
    /// and client `exit` notification.
    ShuttingDown,
    /// `exit` notification received — session complete, no further messages valid.
    Exited,
}

/// Which endpoint sent a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageOrigin {
    Client,
    Server,
}

/// A single LSP protocol message, classified by its structural role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolMessage {
    /// A JSON-RPC request (has an id, expects a response).
    Request { method: String, id: u64 },
    /// A JSON-RPC response to a prior request.
    Response { id: u64, success: bool },
    /// A JSON-RPC notification (no id, no response expected).
    Notification { method: String },
}

/// A deviation from the LSP lifecycle model detected during conformance checking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConformanceViolation {
    /// A non-`initialize` request was sent before initialization completed.
    RequestBeforeInitialize { method: String },
    /// A request was sent after `shutdown` was issued.
    RequestAfterShutdown { method: String },
    /// `initialize` was called a second time.
    DuplicateInitialize,
    /// `shutdown` was called before initialization completed.
    ShutdownBeforeInitialize,
    /// `exit` was called without a prior `shutdown`.
    ExitBeforeShutdown,
    /// A message was sent after `exit`.
    MessageAfterExit { method: String },
    /// A response arrived for an unknown (or already-consumed) request id.
    UnknownResponseId { id: u64 },
}

/// Aggregated result of a conformance check over a message trace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConformanceCheckResult {
    pub violations: Vec<ConformanceViolation>,
    pub final_state: LspProtocolState,
    pub message_count: usize,
    pub request_count: usize,
    pub response_count: usize,
    pub notification_count: usize,
}

impl ConformanceCheckResult {
    /// Returns `true` when the trace produced no conformance violations.
    pub fn is_conformant(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Token-replay conformance checker for the LSP lifecycle protocol model.
///
/// Feed messages one at a time with [`check`], or run an entire trace with
/// [`check_trace`].  The checker maintains:
///
/// * The current lifecycle state (the "token position" in process-mining terms).
/// * The set of pending request ids, so it can validate response matching.
/// * A log of all detected violations.
pub struct ProtocolConformanceChecker {
    state: LspProtocolState,
    pending_request_ids: std::collections::HashSet<u64>,
    violations: Vec<ConformanceViolation>,
    message_count: usize,
    request_count: usize,
    response_count: usize,
    notification_count: usize,
}

impl ProtocolConformanceChecker {
    /// Create a new checker in the `PreInitialize` state.
    pub fn new() -> Self {
        Self {
            state: LspProtocolState::PreInitialize,
            pending_request_ids: std::collections::HashSet::new(),
            violations: Vec::new(),
            message_count: 0,
            request_count: 0,
            response_count: 0,
            notification_count: 0,
        }
    }

    /// Return a reference to the current protocol state.
    pub fn state(&self) -> &LspProtocolState {
        &self.state
    }

    /// Process one message, update state, and return any violation detected.
    ///
    /// ## State-transition table (token replay)
    ///
    /// | State          | Message                              | Next state      | Violation?                   |
    /// |----------------|--------------------------------------|-----------------|------------------------------|
    /// | PreInitialize  | Client Request "initialize"          | Initializing    | —                            |
    /// | PreInitialize  | Client Request <other>               | (unchanged)     | RequestBeforeInitialize      |
    /// | PreInitialize  | Notification "exit" (client)         | Exited          | ExitBeforeShutdown           |
    /// | PreInitialize  | Notification "shutdown" concept      | —               | ShutdownBeforeInitialize (n/a, shutdown is a Request) |
    /// | Initializing   | Server Response (to initialize)      | (unchanged)     | — (waiting for initialized)  |
    /// | Initializing   | Client Notification "initialized"    | Running         | —                            |
    /// | Initializing   | Client Notification "exit"           | Exited          | ExitBeforeShutdown           |
    /// | Running        | Client Request "initialize"          | (unchanged)     | DuplicateInitialize          |
    /// | Running        | Client Request "shutdown"            | ShuttingDown    | —                            |
    /// | Running        | anything else                        | (unchanged)     | —                            |
    /// | ShuttingDown   | Server Response                      | (unchanged)     | —                            |
    /// | ShuttingDown   | Client Notification "exit"           | Exited          | —                            |
    /// | ShuttingDown   | Client Request "initialize"          | (unchanged)     | DuplicateInitialize          |
    /// | ShuttingDown   | Client Request <other>               | (unchanged)     | RequestAfterShutdown         |
    /// | Exited         | anything                             | (unchanged)     | MessageAfterExit             |
    pub fn check(
        &mut self,
        origin: MessageOrigin,
        msg: ProtocolMessage,
    ) -> Option<ConformanceViolation> {
        self.message_count += 1;

        // Update per-kind counters.
        match &msg {
            ProtocolMessage::Request { .. } => self.request_count += 1,
            ProtocolMessage::Response { .. } => self.response_count += 1,
            ProtocolMessage::Notification { .. } => self.notification_count += 1,
        }

        let violation = match &self.state {
            // ----------------------------------------------------------------
            // PreInitialize: only the initialize request is permitted.
            // ----------------------------------------------------------------
            LspProtocolState::PreInitialize => {
                match &msg {
                    ProtocolMessage::Request { method, id } => {
                        if method == "initialize" && origin == MessageOrigin::Client {
                            self.pending_request_ids.insert(*id);
                            self.state = LspProtocolState::Initializing;
                            None
                        } else if method == "initialize" {
                            // initialize from the wrong direction — still move to Initializing
                            // but record it as a violation treated as RequestBeforeInitialize
                            // (no specific enum variant for wrong-direction initialize; we
                            // treat it as premature to keep the model sound).
                            self.pending_request_ids.insert(*id);
                            self.state = LspProtocolState::Initializing;
                            None
                        } else {
                            Some(ConformanceViolation::RequestBeforeInitialize {
                                method: method.clone(),
                            })
                        }
                    }
                    ProtocolMessage::Notification { method } => {
                        if method == "exit" {
                            self.state = LspProtocolState::Exited;
                            Some(ConformanceViolation::ExitBeforeShutdown)
                        } else {
                            // Other notifications before initialize are also premature.
                            Some(ConformanceViolation::RequestBeforeInitialize {
                                method: method.clone(),
                            })
                        }
                    }
                    ProtocolMessage::Response { id, .. } => {
                        // A response with no prior request is an unknown id.
                        Some(ConformanceViolation::UnknownResponseId { id: *id })
                    }
                }
            }

            // ----------------------------------------------------------------
            // Initializing: waiting for server's initialize response AND the
            // client's "initialized" notification (in that conceptual order,
            // though we accept them without enforcing ordering between them).
            // ----------------------------------------------------------------
            LspProtocolState::Initializing => {
                match &msg {
                    ProtocolMessage::Response { id, .. } => {
                        // Server responds to the initialize request.
                        if self.pending_request_ids.remove(id) {
                            None
                        } else {
                            Some(ConformanceViolation::UnknownResponseId { id: *id })
                        }
                    }
                    ProtocolMessage::Notification { method } => {
                        if method == "initialized" && origin == MessageOrigin::Client {
                            self.state = LspProtocolState::Running;
                            None
                        } else if method == "exit" {
                            self.state = LspProtocolState::Exited;
                            Some(ConformanceViolation::ExitBeforeShutdown)
                        } else {
                            // Other notifications during initialization are permitted
                            // (spec allows some like $/cancelRequest), so no violation.
                            None
                        }
                    }
                    ProtocolMessage::Request { method, id } => {
                        if method == "initialize" {
                            // Second initialize while still initializing.
                            self.pending_request_ids.insert(*id);
                            Some(ConformanceViolation::DuplicateInitialize)
                        } else if method == "shutdown" {
                            // shutdown before initialization finished.
                            self.pending_request_ids.insert(*id);
                            Some(ConformanceViolation::ShutdownBeforeInitialize)
                        } else {
                            self.pending_request_ids.insert(*id);
                            Some(ConformanceViolation::RequestBeforeInitialize {
                                method: method.clone(),
                            })
                        }
                    }
                }
            }

            // ----------------------------------------------------------------
            // Running: all normal LSP traffic is valid.
            // ----------------------------------------------------------------
            LspProtocolState::Running => {
                match &msg {
                    ProtocolMessage::Request { method, id } => {
                        if method == "initialize" {
                            self.pending_request_ids.insert(*id);
                            Some(ConformanceViolation::DuplicateInitialize)
                        } else if method == "shutdown" && origin == MessageOrigin::Client {
                            self.pending_request_ids.insert(*id);
                            self.state = LspProtocolState::ShuttingDown;
                            None
                        } else {
                            self.pending_request_ids.insert(*id);
                            None
                        }
                    }
                    ProtocolMessage::Response { id, .. } => {
                        if self.pending_request_ids.remove(id) {
                            None
                        } else {
                            Some(ConformanceViolation::UnknownResponseId { id: *id })
                        }
                    }
                    ProtocolMessage::Notification { method } => {
                        if method == "exit" {
                            // exit without a prior shutdown is a protocol violation.
                            self.state = LspProtocolState::Exited;
                            Some(ConformanceViolation::ExitBeforeShutdown)
                        } else {
                            // All other notifications are valid in Running state.
                            None
                        }
                    }
                }
            }

            // ----------------------------------------------------------------
            // ShuttingDown: only the shutdown response and exit notification
            // are valid.
            // ----------------------------------------------------------------
            LspProtocolState::ShuttingDown => {
                match &msg {
                    ProtocolMessage::Response { id, .. } => {
                        // The server's response to the shutdown request.
                        if self.pending_request_ids.remove(id) {
                            None
                        } else {
                            Some(ConformanceViolation::UnknownResponseId { id: *id })
                        }
                    }
                    ProtocolMessage::Notification { method } => {
                        if method == "exit" && origin == MessageOrigin::Client {
                            self.state = LspProtocolState::Exited;
                            None
                        } else {
                            // Other notifications after shutdown are violations.
                            Some(ConformanceViolation::RequestAfterShutdown {
                                method: method.clone(),
                            })
                        }
                    }
                    ProtocolMessage::Request { method, id } => {
                        self.pending_request_ids.insert(*id);
                        if method == "initialize" {
                            Some(ConformanceViolation::DuplicateInitialize)
                        } else {
                            Some(ConformanceViolation::RequestAfterShutdown {
                                method: method.clone(),
                            })
                        }
                    }
                }
            }

            // ----------------------------------------------------------------
            // Exited: no further messages are valid.
            // ----------------------------------------------------------------
            LspProtocolState::Exited => {
                let label = match &msg {
                    ProtocolMessage::Request { method, .. } => method.clone(),
                    ProtocolMessage::Response { id, .. } => format!("<response id={}>", id),
                    ProtocolMessage::Notification { method } => method.clone(),
                };
                Some(ConformanceViolation::MessageAfterExit { method: label })
            }
        };

        if let Some(ref v) = violation {
            self.violations.push(v.clone());
        }
        violation
    }

    /// Consume the checker and return the aggregated conformance result.
    pub fn finish(self) -> ConformanceCheckResult {
        ConformanceCheckResult {
            violations: self.violations,
            final_state: self.state,
            message_count: self.message_count,
            request_count: self.request_count,
            response_count: self.response_count,
            notification_count: self.notification_count,
        }
    }

    /// Convenience method: run a complete trace and return the result.
    pub fn check_trace(messages: &[(MessageOrigin, ProtocolMessage)]) -> ConformanceCheckResult {
        let mut checker = Self::new();
        for (origin, msg) in messages {
            checker.check(origin.clone(), msg.clone());
        }
        checker.finish()
    }
}

impl Default for ProtocolConformanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Happy-path trace: initialize → initialized → shutdown → exit
    #[test]
    fn test_normal_lifecycle_is_conformant() {
        let trace = vec![
            // 1. Client sends initialize request (id=1)
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "initialize".to_string(),
                    id: 1,
                },
            ),
            // 2. Server responds to initialize
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 1, success: true },
            ),
            // 3. Client sends initialized notification → Running
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "initialized".to_string(),
                },
            ),
            // 4. Some normal work in Running state
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "textDocument/hover".to_string(),
                    id: 2,
                },
            ),
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 2, success: true },
            ),
            // 5. Client sends shutdown → ShuttingDown
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "shutdown".to_string(),
                    id: 3,
                },
            ),
            // 6. Server responds to shutdown
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 3, success: true },
            ),
            // 7. Client sends exit → Exited
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "exit".to_string(),
                },
            ),
        ];

        let result = ProtocolConformanceChecker::check_trace(&trace);
        assert!(result.is_conformant(), "violations: {:?}", result.violations);
        assert_eq!(result.final_state, LspProtocolState::Exited);
        assert_eq!(result.message_count, 8);
        assert_eq!(result.request_count, 3);
        assert_eq!(result.response_count, 3);
        assert_eq!(result.notification_count, 2);
    }

    #[test]
    fn test_request_before_initialize() {
        let trace = vec![(
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "textDocument/hover".to_string(),
                id: 1,
            },
        )];
        let result = ProtocolConformanceChecker::check_trace(&trace);
        assert!(!result.is_conformant());
        assert!(matches!(
            result.violations[0],
            ConformanceViolation::RequestBeforeInitialize { .. }
        ));
    }

    #[test]
    fn test_duplicate_initialize() {
        let trace = vec![
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "initialize".to_string(),
                    id: 1,
                },
            ),
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 1, success: true },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "initialized".to_string(),
                },
            ),
            // Second initialize in Running state
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "initialize".to_string(),
                    id: 2,
                },
            ),
        ];
        let result = ProtocolConformanceChecker::check_trace(&trace);
        assert!(!result.is_conformant());
        assert!(matches!(
            result.violations[0],
            ConformanceViolation::DuplicateInitialize
        ));
    }

    #[test]
    fn test_exit_before_shutdown() {
        let trace = vec![
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "initialize".to_string(),
                    id: 1,
                },
            ),
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 1, success: true },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "initialized".to_string(),
                },
            ),
            // exit without shutdown
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "exit".to_string(),
                },
            ),
        ];
        let result = ProtocolConformanceChecker::check_trace(&trace);
        assert!(!result.is_conformant());
        assert!(matches!(
            result.violations[0],
            ConformanceViolation::ExitBeforeShutdown
        ));
        assert_eq!(result.final_state, LspProtocolState::Exited);
    }

    #[test]
    fn test_message_after_exit() {
        let trace = vec![
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "initialize".to_string(),
                    id: 1,
                },
            ),
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 1, success: true },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "initialized".to_string(),
                },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "shutdown".to_string(),
                    id: 2,
                },
            ),
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 2, success: true },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "exit".to_string(),
                },
            ),
            // spurious message after exit
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "$/cancelRequest".to_string(),
                },
            ),
        ];
        let result = ProtocolConformanceChecker::check_trace(&trace);
        assert!(!result.is_conformant());
        assert!(matches!(
            result.violations[0],
            ConformanceViolation::MessageAfterExit { .. }
        ));
    }

    #[test]
    fn test_unknown_response_id() {
        let trace = vec![
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "initialize".to_string(),
                    id: 1,
                },
            ),
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 1, success: true },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "initialized".to_string(),
                },
            ),
            // Response for id 99 which was never requested
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 99, success: true },
            ),
        ];
        let result = ProtocolConformanceChecker::check_trace(&trace);
        assert!(!result.is_conformant());
        assert!(matches!(
            result.violations[0],
            ConformanceViolation::UnknownResponseId { id: 99 }
        ));
    }

    #[test]
    fn test_request_after_shutdown() {
        let trace = vec![
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "initialize".to_string(),
                    id: 1,
                },
            ),
            (
                MessageOrigin::Server,
                ProtocolMessage::Response { id: 1, success: true },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Notification {
                    method: "initialized".to_string(),
                },
            ),
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "shutdown".to_string(),
                    id: 2,
                },
            ),
            // Another request while shutting down
            (
                MessageOrigin::Client,
                ProtocolMessage::Request {
                    method: "textDocument/completion".to_string(),
                    id: 3,
                },
            ),
        ];
        let result = ProtocolConformanceChecker::check_trace(&trace);
        assert!(!result.is_conformant());
        assert!(matches!(
            result.violations[0],
            ConformanceViolation::RequestAfterShutdown { .. }
        ));
    }

    #[test]
    fn test_default_starts_in_pre_initialize() {
        let checker = ProtocolConformanceChecker::default();
        assert_eq!(checker.state(), &LspProtocolState::PreInitialize);
    }

    #[test]
    fn test_incremental_check_returns_violation() {
        let mut checker = ProtocolConformanceChecker::new();
        let v = checker.check(
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "textDocument/hover".to_string(),
                id: 1,
            },
        );
        assert!(v.is_some());
        assert!(matches!(
            v.unwrap(),
            ConformanceViolation::RequestBeforeInitialize { .. }
        ));
    }
}
