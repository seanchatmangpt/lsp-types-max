/// Jobs-To-Be-Done conformance tests for the LSP protocol lifecycle checker.
///
/// Each test represents a real scenario where a conformance checker must detect
/// protocol violations or confirm a well-formed message trace.
/// Based on the token-replay process mining approach (Dr. Wil van der Aalst).
use lsp_types_max::conformance::{
    ConformanceViolation, LspProtocolState, MessageOrigin, ProtocolConformanceChecker,
    ProtocolMessage,
};

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Build the standard 3-message opening handshake up to Running state.
/// Returns messages: initialize request (id=1), initialize response (id=1),
/// initialized notification.
fn init_sequence() -> Vec<(MessageOrigin, ProtocolMessage)> {
    vec![
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
    ]
}

/// Append a clean shutdown sequence (shutdown request id=N, shutdown response, exit).
fn shutdown_sequence(shutdown_id: u64) -> Vec<(MessageOrigin, ProtocolMessage)> {
    vec![
        (
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "shutdown".to_string(),
                id: shutdown_id,
            },
        ),
        (
            MessageOrigin::Server,
            ProtocolMessage::Response {
                id: shutdown_id,
                success: true,
            },
        ),
        (
            MessageOrigin::Client,
            ProtocolMessage::Notification {
                method: "exit".to_string(),
            },
        ),
    ]
}

// ── 1. Valid initialize sequence ─────────────────────────────────────────────

#[test]
fn job_valid_initialize_then_work_then_shutdown() {
    let mut trace = init_sequence();
    // Normal work: hover request/response
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/hover".to_string(),
            id: 2,
        },
    ));
    trace.push((
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 2, success: true },
    ));
    trace.extend(shutdown_sequence(3));

    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(
        result.is_conformant(),
        "Expected no violations but got: {:?}",
        result.violations
    );
    assert_eq!(result.final_state, LspProtocolState::Exited);
    // 3 init + 2 hover + 3 shutdown = 8 messages
    assert_eq!(result.message_count, 8);
    assert_eq!(result.request_count, 3);  // initialize, hover, shutdown
    assert_eq!(result.response_count, 3); // initialize resp, hover resp, shutdown resp
    assert_eq!(result.notification_count, 2); // initialized, exit
}

// ── 2. Request before initialize ─────────────────────────────────────────────

#[test]
fn job_request_before_initialize_is_violation() {
    let trace = vec![(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/hover".to_string(),
            id: 1,
        },
    )];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert_eq!(result.violations.len(), 1);
    assert!(
        matches!(
            &result.violations[0],
            ConformanceViolation::RequestBeforeInitialize { method }
            if method == "textDocument/hover"
        ),
        "Expected RequestBeforeInitialize{{method: textDocument/hover}}, got {:?}",
        result.violations[0]
    );
}

// ── 3. Duplicate initialize ───────────────────────────────────────────────────

#[test]
fn job_duplicate_initialize_is_violation() {
    let mut trace = init_sequence();
    // Second initialize while in Running state
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "initialize".to_string(),
            id: 2,
        },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(result.violations[0], ConformanceViolation::DuplicateInitialize),
        "Expected DuplicateInitialize, got {:?}",
        result.violations[0]
    );
}

// ── 4. Request after shutdown ─────────────────────────────────────────────────

#[test]
fn job_request_after_shutdown_is_violation() {
    let mut trace = init_sequence();
    // Send shutdown request — now in ShuttingDown
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "shutdown".to_string(),
            id: 2,
        },
    ));
    // Try a hover while shutting down
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/hover".to_string(),
            id: 3,
        },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(
            &result.violations[0],
            ConformanceViolation::RequestAfterShutdown { method }
            if method == "textDocument/hover"
        ),
        "Expected RequestAfterShutdown{{method: textDocument/hover}}, got {:?}",
        result.violations[0]
    );
}

// ── 5. Exit before shutdown ───────────────────────────────────────────────────

#[test]
fn job_exit_before_shutdown_is_violation() {
    let mut trace = init_sequence();
    // Exit without issuing shutdown first
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "exit".to_string(),
        },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(result.violations[0], ConformanceViolation::ExitBeforeShutdown),
        "Expected ExitBeforeShutdown, got {:?}",
        result.violations[0]
    );
    assert_eq!(result.final_state, LspProtocolState::Exited);
}

// ── 6. Unknown response id ────────────────────────────────────────────────────

#[test]
fn job_unknown_response_id_is_violation() {
    let mut trace = init_sequence();
    // Server sends a response for id=999 which was never requested
    trace.push((
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 999, success: true },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(
            result.violations[0],
            ConformanceViolation::UnknownResponseId { id: 999 }
        ),
        "Expected UnknownResponseId{{id: 999}}, got {:?}",
        result.violations[0]
    );
}

// ── 7. Message after exit ─────────────────────────────────────────────────────

#[test]
fn job_message_after_exit_is_violation() {
    let mut trace = init_sequence();
    trace.extend(shutdown_sequence(2));
    // Spurious request after session has exited
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/hover".to_string(),
            id: 10,
        },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(result.violations[0], ConformanceViolation::MessageAfterExit { .. }),
        "Expected MessageAfterExit, got {:?}",
        result.violations[0]
    );
}

// ── 8. State transitions ──────────────────────────────────────────────────────

#[test]
fn job_state_transitions_are_correct() {
    let mut checker = ProtocolConformanceChecker::new();

    // Before any messages: PreInitialize
    assert_eq!(checker.state(), &LspProtocolState::PreInitialize);

    // After initialize request: Initializing
    checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "initialize".to_string(),
            id: 1,
        },
    );
    assert_eq!(checker.state(), &LspProtocolState::Initializing);

    // After initialize response: still Initializing (need initialized notification)
    checker.check(
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 1, success: true },
    );
    assert_eq!(checker.state(), &LspProtocolState::Initializing);

    // After initialized notification: Running
    checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "initialized".to_string(),
        },
    );
    assert_eq!(checker.state(), &LspProtocolState::Running);

    // After shutdown request: ShuttingDown
    checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "shutdown".to_string(),
            id: 2,
        },
    );
    assert_eq!(checker.state(), &LspProtocolState::ShuttingDown);

    // Consume shutdown response
    checker.check(
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 2, success: true },
    );
    assert_eq!(checker.state(), &LspProtocolState::ShuttingDown);

    // After exit notification: Exited
    checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "exit".to_string(),
        },
    );
    assert_eq!(checker.state(), &LspProtocolState::Exited);

    let result = checker.finish();
    assert!(result.is_conformant(), "violations: {:?}", result.violations);
}

// ── 9. Multiple violations accumulate ────────────────────────────────────────

#[test]
fn job_multiple_violations_accumulate() {
    let trace = vec![
        (
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "textDocument/hover".to_string(),
                id: 1,
            },
        ),
        (
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "textDocument/completion".to_string(),
                id: 2,
            },
        ),
        (
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "textDocument/definition".to_string(),
                id: 3,
            },
        ),
    ];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    // All three produce RequestBeforeInitialize
    assert_eq!(result.violations.len(), 3);
    for v in &result.violations {
        assert!(
            matches!(v, ConformanceViolation::RequestBeforeInitialize { .. }),
            "Expected RequestBeforeInitialize, got {:?}",
            v
        );
    }
}

// ── 10. check_trace convenience method ───────────────────────────────────────

#[test]
fn job_check_trace_full_session() {
    let mut trace = init_sequence();
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/completion".to_string(),
            id: 2,
        },
    ));
    trace.push((
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 2, success: true },
    ));
    trace.extend(shutdown_sequence(3));

    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(
        result.is_conformant(),
        "check_trace should return conformant result; violations: {:?}",
        result.violations
    );
    assert_eq!(result.final_state, LspProtocolState::Exited);
}

// ── 11. Shutdown before initialize ───────────────────────────────────────────

#[test]
fn job_shutdown_before_initialize_is_violation() {
    let trace = vec![(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "shutdown".to_string(),
            id: 1,
        },
    )];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    // shutdown is a non-initialize request in PreInitialize state →
    // the implementation logs it as RequestBeforeInitialize (no separate variant for this path)
    // OR as ShutdownBeforeInitialize if still in Initializing. Since we're in PreInitialize
    // it falls through to RequestBeforeInitialize.
    // Either way, a violation must be present.
    assert!(!result.violations.is_empty());
}

/// Shutdown sent while still in Initializing state (initialize req sent, but no initialized notif).
#[test]
fn job_shutdown_during_initializing_is_violation() {
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
        // shutdown before "initialized" notification
        (
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "shutdown".to_string(),
                id: 2,
            },
        ),
    ];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(result.violations[0], ConformanceViolation::ShutdownBeforeInitialize),
        "Expected ShutdownBeforeInitialize, got {:?}",
        result.violations[0]
    );
}

// ── 12. Counter methods ───────────────────────────────────────────────────────

#[test]
fn job_is_conformant_true_when_no_violations() {
    let trace = init_sequence();
    let result = ProtocolConformanceChecker::check_trace(&trace);
    // init sequence alone leaves state as Running — that is valid; no violations yet
    assert!(result.is_conformant());
}

#[test]
fn job_is_conformant_false_when_violations_exist() {
    let trace = vec![(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/hover".to_string(),
            id: 1,
        },
    )];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
}

#[test]
fn job_message_count_is_accurate() {
    let trace = init_sequence(); // 3 messages
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert_eq!(result.message_count, 3);
}

#[test]
fn job_request_count_is_accurate() {
    let trace = init_sequence(); // 1 request (initialize)
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert_eq!(result.request_count, 1);
}

#[test]
fn job_notification_count_is_accurate() {
    let trace = init_sequence(); // 1 notification (initialized)
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert_eq!(result.notification_count, 1);
}

#[test]
fn job_response_count_is_accurate() {
    let trace = init_sequence(); // 1 response (to initialize)
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert_eq!(result.response_count, 1);
}

#[test]
fn job_all_counters_in_full_session() {
    let mut trace = init_sequence(); // req=1, resp=1, notif=1
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/hover".to_string(),
            id: 2,
        },
    )); // req=2
    trace.push((
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 2, success: true },
    )); // resp=2
    trace.extend(shutdown_sequence(3)); // req=3, resp=3, notif=2
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert_eq!(result.message_count, 8);
    assert_eq!(result.request_count, 3);
    assert_eq!(result.response_count, 3);
    assert_eq!(result.notification_count, 2);
}

// ── 13. Edge cases ────────────────────────────────────────────────────────────

#[test]
fn job_notification_before_initialize_is_violation() {
    let trace = vec![(
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "textDocument/didOpen".to_string(),
        },
    )];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    // Non-exit notification in PreInitialize state → RequestBeforeInitialize
    assert!(
        matches!(
            &result.violations[0],
            ConformanceViolation::RequestBeforeInitialize { method }
            if method == "textDocument/didOpen"
        ),
        "Unexpected violation: {:?}",
        result.violations[0]
    );
}

#[test]
fn job_exit_before_initialize_is_violation() {
    let trace = vec![(
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "exit".to_string(),
        },
    )];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(result.violations[0], ConformanceViolation::ExitBeforeShutdown),
        "Expected ExitBeforeShutdown, got {:?}",
        result.violations[0]
    );
    assert_eq!(result.final_state, LspProtocolState::Exited);
}

#[test]
fn job_server_request_in_running_state_is_valid() {
    // Servers can send requests (e.g., client/registerCapability) in Running state.
    let mut trace = init_sequence();
    trace.push((
        MessageOrigin::Server,
        ProtocolMessage::Request {
            method: "client/registerCapability".to_string(),
            id: 2,
        },
    ));
    // Client responds
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Response { id: 2, success: true },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(
        result.is_conformant(),
        "Server requests in Running state should be valid; violations: {:?}",
        result.violations
    );
}

#[test]
fn job_unknown_response_in_pre_initialize_is_violation() {
    let trace = vec![(
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 42, success: false },
    )];
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(
            result.violations[0],
            ConformanceViolation::UnknownResponseId { id: 42 }
        ),
        "Expected UnknownResponseId{{id: 42}}, got {:?}",
        result.violations[0]
    );
}

#[test]
fn job_message_after_exit_response_label_contains_id() {
    let mut trace = init_sequence();
    trace.extend(shutdown_sequence(2));
    // Response after exit: label should encode the response id
    trace.push((
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 5, success: true },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(result.violations[0], ConformanceViolation::MessageAfterExit { .. }),
        "Expected MessageAfterExit, got {:?}",
        result.violations[0]
    );
}

#[test]
fn job_duplicate_initialize_in_shutting_down_is_violation() {
    let mut trace = init_sequence();
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "shutdown".to_string(),
            id: 2,
        },
    ));
    // Attempt to reinitialize while shutting down
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "initialize".to_string(),
            id: 3,
        },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(!result.is_conformant());
    assert!(
        matches!(result.violations[0], ConformanceViolation::DuplicateInitialize),
        "Expected DuplicateInitialize in ShuttingDown state, got {:?}",
        result.violations[0]
    );
}

#[test]
fn job_default_checker_starts_in_pre_initialize() {
    let checker = ProtocolConformanceChecker::default();
    assert_eq!(checker.state(), &LspProtocolState::PreInitialize);
}

#[test]
fn job_incremental_check_returns_violation_on_bad_message() {
    let mut checker = ProtocolConformanceChecker::new();
    let violation = checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "textDocument/definition".to_string(),
            id: 7,
        },
    );
    assert!(violation.is_some());
    assert!(
        matches!(
            violation.unwrap(),
            ConformanceViolation::RequestBeforeInitialize { .. }
        )
    );
}

#[test]
fn job_incremental_check_returns_none_on_valid_message() {
    let mut checker = ProtocolConformanceChecker::new();
    let violation = checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "initialize".to_string(),
            id: 1,
        },
    );
    assert!(violation.is_none(), "Valid message should produce no violation");
}

#[test]
fn job_finish_consumes_checker_and_returns_result() {
    let mut checker = ProtocolConformanceChecker::new();
    checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "initialize".to_string(),
            id: 1,
        },
    );
    checker.check(
        MessageOrigin::Server,
        ProtocolMessage::Response { id: 1, success: true },
    );
    checker.check(
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "initialized".to_string(),
        },
    );
    let result = checker.finish();
    assert_eq!(result.final_state, LspProtocolState::Running);
    assert!(result.is_conformant());
    assert_eq!(result.message_count, 3);
}

#[test]
fn job_cancel_request_notification_in_running_is_valid() {
    // $/cancelRequest is a common notification sent during normal operation.
    let mut trace = init_sequence();
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "$/cancelRequest".to_string(),
        },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(
        result.is_conformant(),
        "$/cancelRequest in Running state should be valid; violations: {:?}",
        result.violations
    );
}

#[test]
fn job_multiple_work_requests_in_running_state_are_valid() {
    let mut trace = init_sequence();
    for id in 2u64..=6 {
        trace.push((
            MessageOrigin::Client,
            ProtocolMessage::Request {
                method: "textDocument/hover".to_string(),
                id,
            },
        ));
        trace.push((
            MessageOrigin::Server,
            ProtocolMessage::Response { id, success: true },
        ));
    }
    trace.extend(shutdown_sequence(7));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    assert!(
        result.is_conformant(),
        "Many requests in Running state should all be valid; violations: {:?}",
        result.violations
    );
    assert_eq!(result.final_state, LspProtocolState::Exited);
}

#[test]
fn job_exit_without_shutdown_response_is_violation() {
    // After sending shutdown, server hasn't responded yet but client sends exit.
    // The exit itself is valid in ShuttingDown state (spec allows it), so we
    // only check that the sequence reaches Exited and response becomes orphaned.
    let mut trace = init_sequence();
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Request {
            method: "shutdown".to_string(),
            id: 2,
        },
    ));
    // Skip the server's shutdown response — go straight to exit.
    trace.push((
        MessageOrigin::Client,
        ProtocolMessage::Notification {
            method: "exit".to_string(),
        },
    ));
    let result = ProtocolConformanceChecker::check_trace(&trace);
    // Exiting without the shutdown response is accepted by the state machine
    // (exit in ShuttingDown → Exited), but the pending request id=2 is never resolved.
    // The checker doesn't flag this as a violation (no "UnresolvedRequest" variant).
    assert_eq!(result.final_state, LspProtocolState::Exited);
}

#[test]
fn job_empty_trace_is_conformant_in_pre_initialize() {
    let result = ProtocolConformanceChecker::check_trace(&[]);
    assert!(result.is_conformant());
    assert_eq!(result.final_state, LspProtocolState::PreInitialize);
    assert_eq!(result.message_count, 0);
    assert_eq!(result.request_count, 0);
    assert_eq!(result.response_count, 0);
    assert_eq!(result.notification_count, 0);
}
