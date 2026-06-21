/// Jobs-To-Be-Done tests for the LSP process mining module.
///
/// Each test represents a real observation or analysis task performed on LSP
/// event logs using XES-inspired process mining types (event log, trace, event).
/// Inspired by Dr. Wil van der Aalst's XES standard and process mining work.
use lsp_types_max::process_mining::{
    LspActivity, LspEvent, LspEventLog, LspTrace, MessageDirection, MessageKind,
};

// ── 1. Build an event log ─────────────────────────────────────────────────────

#[test]
fn job_build_lsp_event_log() {
    let mut log = LspEventLog::new("rust-analyzer");
    let mut trace = LspTrace::new("session-001");

    // t=0: client sends initialize request (id=1)
    trace.push(LspEvent::request(
        0,
        LspActivity::Initialize,
        MessageDirection::ClientToServer,
        1,
    ));
    // t=10: server responds to initialize (id=1, took 10ms)
    trace.push(LspEvent::response(
        10,
        MessageDirection::ServerToClient,
        1,
        Some(10),
    ));
    // t=11: client sends initialized notification
    trace.push(LspEvent::notification(
        11,
        LspActivity::Initialized,
        MessageDirection::ClientToServer,
    ));

    log.push(trace);

    assert_eq!(log.case_count(), 1);
    assert_eq!(log.event_count(), 3);
    assert_eq!(log.name, "rust-analyzer");
}

// ── 2. Activity method names ──────────────────────────────────────────────────

#[test]
fn job_activity_method_names_are_correct() {
    assert_eq!(LspActivity::Initialize.method_name(), "initialize");
    assert_eq!(
        LspActivity::TextDocumentCompletion.method_name(),
        "textDocument/completion"
    );
    assert_eq!(
        LspActivity::TextDocumentHover.method_name(),
        "textDocument/hover"
    );
    assert_eq!(LspActivity::Shutdown.method_name(), "shutdown");
    assert_eq!(
        LspActivity::WorkspaceDiagnostic.method_name(),
        "workspace/diagnostic"
    );
    assert_eq!(
        LspActivity::Custom("my/method".into()).method_name(),
        "my/method"
    );
}

#[test]
fn job_activity_method_names_lifecycle() {
    assert_eq!(LspActivity::Initialized.method_name(), "initialized");
    assert_eq!(LspActivity::Exit.method_name(), "exit");
    assert_eq!(LspActivity::CancelRequest.method_name(), "$/cancelRequest");
    assert_eq!(LspActivity::Progress.method_name(), "$/progress");
}

#[test]
fn job_activity_method_names_text_document_sync() {
    assert_eq!(
        LspActivity::TextDocumentDidOpen.method_name(),
        "textDocument/didOpen"
    );
    assert_eq!(
        LspActivity::TextDocumentDidChange.method_name(),
        "textDocument/didChange"
    );
    assert_eq!(
        LspActivity::TextDocumentDidClose.method_name(),
        "textDocument/didClose"
    );
    assert_eq!(
        LspActivity::TextDocumentDidSave.method_name(),
        "textDocument/didSave"
    );
}

#[test]
fn job_activity_method_names_workspace() {
    assert_eq!(LspActivity::WorkspaceSymbol.method_name(), "workspace/symbol");
    assert_eq!(
        LspActivity::WorkspaceExecuteCommand.method_name(),
        "workspace/executeCommand"
    );
    assert_eq!(
        LspActivity::WorkspaceApplyEdit.method_name(),
        "workspace/applyEdit"
    );
}

// ── 3. From method string ─────────────────────────────────────────────────────

#[test]
fn job_activity_from_method_string() {
    assert_eq!(
        LspActivity::from_method("initialize"),
        LspActivity::Initialize
    );
    assert_eq!(
        LspActivity::from_method("textDocument/hover"),
        LspActivity::TextDocumentHover
    );
    assert_eq!(
        LspActivity::from_method("unknown/method"),
        LspActivity::Custom("unknown/method".into())
    );
}

#[test]
fn job_activity_from_method_lifecycle_roundtrip() {
    for activity in [
        LspActivity::Initialize,
        LspActivity::Initialized,
        LspActivity::Shutdown,
        LspActivity::Exit,
    ] {
        let method = activity.method_name();
        assert_eq!(
            LspActivity::from_method(method),
            activity,
            "from_method({method:?}) did not round-trip"
        );
    }
}

#[test]
fn job_activity_from_method_text_document_roundtrip() {
    for activity in [
        LspActivity::TextDocumentHover,
        LspActivity::TextDocumentCompletion,
        LspActivity::TextDocumentDefinition,
        LspActivity::TextDocumentReferences,
        LspActivity::TextDocumentCodeAction,
    ] {
        let method = activity.method_name();
        assert_eq!(
            LspActivity::from_method(method),
            activity,
            "from_method({method:?}) did not round-trip"
        );
    }
}

#[test]
fn job_activity_custom_method_preserved() {
    let method = "myExtension/doSomething";
    let activity = LspActivity::from_method(method);
    assert_eq!(activity, LspActivity::Custom(method.to_owned()));
    assert_eq!(activity.method_name(), method);
}

// ── 4. Trace duration ─────────────────────────────────────────────────────────

#[test]
fn job_trace_duration_calculation() {
    let mut trace = LspTrace::new("dur-test");
    trace.push(LspEvent::request(
        100,
        LspActivity::Initialize,
        MessageDirection::ClientToServer,
        1,
    ));
    trace.push(LspEvent::response(
        200,
        MessageDirection::ServerToClient,
        1,
        Some(100),
    ));
    trace.push(LspEvent::notification(
        500,
        LspActivity::Initialized,
        MessageDirection::ClientToServer,
    ));
    // duration = max(500) - min(100) = 400
    assert_eq!(trace.duration_ms(), Some(400));
}

// ── 5. Empty trace duration ───────────────────────────────────────────────────

#[test]
fn job_empty_trace_has_no_duration() {
    let trace = LspTrace::new("empty");
    assert_eq!(trace.duration_ms(), None);
}

#[test]
fn job_single_event_trace_has_no_duration() {
    let mut trace = LspTrace::new("single");
    trace.push(LspEvent::notification(
        42,
        LspActivity::Initialized,
        MessageDirection::ClientToServer,
    ));
    assert_eq!(trace.duration_ms(), None);
}

// ── 6. Activity classification ────────────────────────────────────────────────

#[test]
fn job_activity_is_request_or_notification() {
    // Requests
    assert!(LspActivity::Initialize.is_request(), "Initialize should be a request");
    assert!(
        LspActivity::TextDocumentCompletion.is_request(),
        "TextDocumentCompletion should be a request"
    );
    assert!(LspActivity::TextDocumentHover.is_request(), "TextDocumentHover should be a request");
    assert!(LspActivity::Shutdown.is_request(), "Shutdown should be a request");

    // Notifications
    assert!(
        LspActivity::Initialized.is_notification(),
        "Initialized should be a notification"
    );
    assert!(
        LspActivity::TextDocumentDidOpen.is_notification(),
        "TextDocumentDidOpen should be a notification"
    );
    assert!(
        LspActivity::TextDocumentDidChange.is_notification(),
        "TextDocumentDidChange should be a notification"
    );
    assert!(LspActivity::Exit.is_notification(), "Exit should be a notification");
}

#[test]
fn job_activity_request_and_notification_are_mutually_exclusive() {
    // An activity cannot be both a request and a notification
    let requests = [
        LspActivity::Initialize,
        LspActivity::TextDocumentHover,
        LspActivity::Shutdown,
        LspActivity::WorkspaceDiagnostic,
    ];
    let notifications = [
        LspActivity::Initialized,
        LspActivity::Exit,
        LspActivity::TextDocumentDidOpen,
        LspActivity::TextDocumentDidChange,
    ];
    for r in &requests {
        assert!(
            !r.is_notification(),
            "{:?} should not be a notification",
            r
        );
    }
    for n in &notifications {
        assert!(!n.is_request(), "{:?} should not be a request", n);
    }
}

// ── 7. Activity frequency in event log ───────────────────────────────────────

#[test]
fn job_activity_frequency_counts() {
    let mut log = LspEventLog::new("freq-test");

    // Session 1: hover x2, completion x1
    let mut t1 = LspTrace::new("s1");
    for id in 1u64..=2 {
        t1.push(LspEvent::request(
            id as i64 * 10,
            LspActivity::TextDocumentHover,
            MessageDirection::ClientToServer,
            id,
        ));
    }
    t1.push(LspEvent::request(
        30,
        LspActivity::TextDocumentCompletion,
        MessageDirection::ClientToServer,
        3,
    ));
    log.push(t1);

    // Session 2: hover x1 more
    let mut t2 = LspTrace::new("s2");
    t2.push(LspEvent::request(
        5,
        LspActivity::TextDocumentHover,
        MessageDirection::ClientToServer,
        1,
    ));
    log.push(t2);

    // hover appears 3x, completion 1x
    let freq = log.activities_by_frequency();
    assert!(!freq.is_empty());

    // The first entry should be hover with count 3
    let (top_activity, top_count) = &freq[0];
    assert_eq!(
        top_activity,
        &LspActivity::TextDocumentHover,
        "Most frequent activity should be TextDocumentHover"
    );
    assert_eq!(*top_count, 3, "TextDocumentHover should appear 3 times");

    // completion should appear somewhere with count 1
    let completion_entry = freq
        .iter()
        .find(|(a, _)| a == &LspActivity::TextDocumentCompletion);
    assert!(completion_entry.is_some(), "TextDocumentCompletion should appear in frequencies");
    assert_eq!(completion_entry.unwrap().1, 1);
}

// ── 8. Multiple traces in log ─────────────────────────────────────────────────

#[test]
fn job_event_log_multiple_traces() {
    let mut log = LspEventLog::new("multi-session");

    for session_id in 1u64..=3 {
        let mut trace = LspTrace::new(format!("session-{}", session_id));
        // 4 events per trace
        trace.push(LspEvent::request(
            0,
            LspActivity::Initialize,
            MessageDirection::ClientToServer,
            1,
        ));
        trace.push(LspEvent::response(
            5,
            MessageDirection::ServerToClient,
            1,
            Some(5),
        ));
        trace.push(LspEvent::notification(
            6,
            LspActivity::Initialized,
            MessageDirection::ClientToServer,
        ));
        trace.push(LspEvent::notification(
            10,
            LspActivity::Exit,
            MessageDirection::ClientToServer,
        ));
        log.push(trace);
    }

    assert_eq!(log.case_count(), 3);
    assert_eq!(log.event_count(), 12); // 3 traces * 4 events
}

// ── 9. Request and notification counts on trace ───────────────────────────────

#[test]
fn job_trace_request_and_notification_counts() {
    let mut trace = LspTrace::new("count-test");

    // 2 requests
    trace.push(LspEvent::request(
        0,
        LspActivity::Initialize,
        MessageDirection::ClientToServer,
        1,
    ));
    trace.push(LspEvent::request(
        20,
        LspActivity::TextDocumentHover,
        MessageDirection::ClientToServer,
        2,
    ));

    // 3 notifications
    trace.push(LspEvent::notification(
        10,
        LspActivity::Initialized,
        MessageDirection::ClientToServer,
    ));
    trace.push(LspEvent::notification(
        15,
        LspActivity::TextDocumentDidOpen,
        MessageDirection::ClientToServer,
    ));
    trace.push(LspEvent::notification(
        30,
        LspActivity::Exit,
        MessageDirection::ClientToServer,
    ));

    // 2 responses
    trace.push(LspEvent::response(5, MessageDirection::ServerToClient, 1, Some(5)));
    trace.push(LspEvent::response(25, MessageDirection::ServerToClient, 2, Some(5)));

    assert_eq!(trace.request_count(), 2);
    assert_eq!(trace.notification_count(), 3);
}

// ── 10. Serde roundtrip ───────────────────────────────────────────────────────

#[test]
fn job_serde_roundtrip_lsp_activity() {
    let activities = [
        LspActivity::Initialize,
        LspActivity::TextDocumentHover,
        LspActivity::Shutdown,
        LspActivity::Custom("ext/custom".into()),
    ];
    for activity in &activities {
        let value = serde_json::to_value(activity).expect("serialize LspActivity");
        let back: LspActivity = serde_json::from_value(value).expect("deserialize LspActivity");
        assert_eq!(activity, &back, "LspActivity serde roundtrip failed for {:?}", activity);
    }
}

#[test]
fn job_serde_roundtrip_message_direction() {
    for dir in [MessageDirection::ClientToServer, MessageDirection::ServerToClient] {
        let value = serde_json::to_value(&dir).expect("serialize MessageDirection");
        let back: MessageDirection =
            serde_json::from_value(value).expect("deserialize MessageDirection");
        assert_eq!(dir, back);
    }
}

#[test]
fn job_serde_roundtrip_message_kind() {
    for kind in [MessageKind::Request, MessageKind::Response, MessageKind::Notification] {
        let value = serde_json::to_value(&kind).expect("serialize MessageKind");
        let back: MessageKind = serde_json::from_value(value).expect("deserialize MessageKind");
        assert_eq!(kind, back);
    }
}

#[test]
fn job_serde_roundtrip_lsp_event_request() {
    let event = LspEvent::request(
        1234,
        LspActivity::TextDocumentHover,
        MessageDirection::ClientToServer,
        42,
    );
    let value = serde_json::to_value(&event).expect("serialize LspEvent request");
    let back: LspEvent = serde_json::from_value(value).expect("deserialize LspEvent request");
    assert_eq!(event, back);
    assert_eq!(back.kind, MessageKind::Request);
    assert_eq!(back.message_id, Some(42));
    assert_eq!(back.duration_ms, None);
}

#[test]
fn job_serde_roundtrip_lsp_event_response() {
    let event = LspEvent::response(2000, MessageDirection::ServerToClient, 7, Some(50));
    let value = serde_json::to_value(&event).expect("serialize LspEvent response");
    let back: LspEvent = serde_json::from_value(value).expect("deserialize LspEvent response");
    assert_eq!(event, back);
    assert_eq!(back.kind, MessageKind::Response);
    assert_eq!(back.message_id, Some(7));
    assert_eq!(back.duration_ms, Some(50));
}

#[test]
fn job_serde_roundtrip_lsp_event_notification() {
    let event = LspEvent::notification(
        500,
        LspActivity::TextDocumentDidOpen,
        MessageDirection::ClientToServer,
    );
    let value = serde_json::to_value(&event).expect("serialize LspEvent notification");
    let back: LspEvent =
        serde_json::from_value(value).expect("deserialize LspEvent notification");
    assert_eq!(event, back);
    assert_eq!(back.kind, MessageKind::Notification);
    assert_eq!(back.message_id, None);
}

#[test]
fn job_serde_roundtrip_lsp_trace() {
    let mut trace = LspTrace::new("roundtrip-session");
    trace.push(LspEvent::request(
        0,
        LspActivity::Initialize,
        MessageDirection::ClientToServer,
        1,
    ));
    trace.push(LspEvent::response(10, MessageDirection::ServerToClient, 1, Some(10)));

    let value = serde_json::to_value(&trace).expect("serialize LspTrace");
    let back: LspTrace = serde_json::from_value(value).expect("deserialize LspTrace");
    assert_eq!(trace, back);
    assert_eq!(back.case_id, "roundtrip-session");
    assert_eq!(back.events.len(), 2);
}

#[test]
fn job_serde_roundtrip_lsp_event_log() {
    let mut log = LspEventLog::new("serde-log");
    let mut trace = LspTrace::new("t1");
    trace.push(LspEvent::notification(
        0,
        LspActivity::Initialized,
        MessageDirection::ClientToServer,
    ));
    log.push(trace);

    let value = serde_json::to_value(&log).expect("serialize LspEventLog");
    let back: LspEventLog = serde_json::from_value(value).expect("deserialize LspEventLog");
    assert_eq!(log, back);
    assert_eq!(back.name, "serde-log");
    assert_eq!(back.case_count(), 1);
    assert_eq!(back.event_count(), 1);
}

// ── 11. Additional edge cases ─────────────────────────────────────────────────

#[test]
fn job_empty_log_has_zero_cases_and_events() {
    let log = LspEventLog::new("empty");
    assert_eq!(log.case_count(), 0);
    assert_eq!(log.event_count(), 0);
}

#[test]
fn job_empty_log_activity_frequency_is_empty() {
    let log = LspEventLog::new("empty");
    let freq = log.activities_by_frequency();
    assert!(freq.is_empty());
}

#[test]
fn job_trace_activities_iterator_visits_all_events() {
    let mut trace = LspTrace::new("iter-test");
    trace.push(LspEvent::request(
        0,
        LspActivity::Initialize,
        MessageDirection::ClientToServer,
        1,
    ));
    trace.push(LspEvent::notification(
        5,
        LspActivity::Initialized,
        MessageDirection::ClientToServer,
    ));
    trace.push(LspEvent::request(
        10,
        LspActivity::TextDocumentHover,
        MessageDirection::ClientToServer,
        2,
    ));

    let activities: Vec<&LspActivity> = trace.activities().collect();
    assert_eq!(activities.len(), 3);
    assert_eq!(activities[0], &LspActivity::Initialize);
    assert_eq!(activities[1], &LspActivity::Initialized);
    assert_eq!(activities[2], &LspActivity::TextDocumentHover);
}

#[test]
fn job_trace_with_server_name() {
    let mut trace = LspTrace::new("session-xyz");
    trace.server_name = Some("clangd".to_string());
    trace.push(LspEvent::notification(
        0,
        LspActivity::Initialized,
        MessageDirection::ClientToServer,
    ));
    assert_eq!(trace.server_name.as_deref(), Some("clangd"));
    assert_eq!(trace.case_id, "session-xyz");
}

#[test]
fn job_response_event_activity_is_custom_response() {
    // LspEvent::response sets activity to Custom("response") since responses
    // carry no method name in the LSP wire format.
    let event = LspEvent::response(100, MessageDirection::ServerToClient, 3, None);
    assert_eq!(event.activity, LspActivity::Custom("response".to_owned()));
    assert_eq!(event.kind, MessageKind::Response);
    assert_eq!(event.message_id, Some(3));
    assert_eq!(event.duration_ms, None);
}

#[test]
fn job_two_event_trace_duration_is_correct() {
    let mut trace = LspTrace::new("two-event");
    trace.push(LspEvent::request(
        1000,
        LspActivity::TextDocumentHover,
        MessageDirection::ClientToServer,
        1,
    ));
    trace.push(LspEvent::response(1250, MessageDirection::ServerToClient, 1, Some(250)));
    assert_eq!(trace.duration_ms(), Some(250));
}

#[test]
fn job_activity_frequency_ties_broken_by_method_name() {
    let mut log = LspEventLog::new("tie-break");
    let mut trace = LspTrace::new("t");
    // Add one of each — all have frequency 1, sorted by method name ascending
    trace.push(LspEvent::request(
        0,
        LspActivity::TextDocumentHover,
        MessageDirection::ClientToServer,
        1,
    ));
    trace.push(LspEvent::request(
        1,
        LspActivity::Initialize,
        MessageDirection::ClientToServer,
        2,
    ));
    log.push(trace);

    let freq = log.activities_by_frequency();
    // Both have count=1. Sorted descending by count, then ascending by method name.
    // "initialize" < "textDocument/hover" lexicographically → Initialize comes first.
    assert_eq!(freq.len(), 2);
    assert_eq!(freq[0].1, 1);
    assert_eq!(freq[1].1, 1);
    // method name of freq[0] must be <= method name of freq[1]
    assert!(
        freq[0].0.method_name() <= freq[1].0.method_name(),
        "Tie-breaking should be ascending by method name"
    );
}
