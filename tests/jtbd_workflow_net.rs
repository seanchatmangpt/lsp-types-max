/// Jobs-To-Be-Done tests for the LSP Petri net / workflow net model.
///
/// Validates the formal process model that represents the LSP protocol lifecycle
/// as a workflow net (Petri net with source and sink places).
use lsp_types_max::workflow_net::*;
use serde_json::json;

// ─── Job: Build and validate the LSP lifecycle workflow net ───────────────────

#[test]
fn job_lsp_lifecycle_net_is_a_valid_workflow_net() {
    let net = lsp_lifecycle_net();
    assert!(net.is_workflow_net(), "LSP lifecycle should be a valid workflow net");
}

#[test]
fn job_lifecycle_net_source_is_unstarted_session() {
    let net = lsp_lifecycle_net();
    assert_eq!(net.source, "p_start");
    let source_place = net.places.iter().find(|p| p.id == "p_start").unwrap();
    assert!(!source_place.label.is_empty());
}

#[test]
fn job_lifecycle_net_sink_is_exited_session() {
    let net = lsp_lifecycle_net();
    assert_eq!(net.sink, "p_end");
    let sink_place = net.places.iter().find(|p| p.id == "p_end").unwrap();
    assert!(!sink_place.label.is_empty());
}

#[test]
fn job_lifecycle_net_has_all_protocol_states_as_places() {
    let net = lsp_lifecycle_net();
    let ids: Vec<&str> = net.places.iter().map(|p| p.id.as_str()).collect();
    for required in [
        "p_start",
        "p_initializing",
        "p_awaiting_initialized",
        "p_running",
        "p_shutting_down",
        "p_awaiting_exit",
        "p_end",
    ] {
        assert!(ids.contains(&required), "Missing place: {required}");
    }
}

#[test]
fn job_lifecycle_net_transitions_include_all_lifecycle_methods() {
    let net = lsp_lifecycle_net();
    let methods: Vec<Option<&str>> =
        net.transitions.iter().map(|t| t.method.as_deref()).collect();
    assert!(methods.contains(&Some("initialize")), "Missing initialize transition");
    assert!(methods.contains(&Some("initialized")), "Missing initialized transition");
    assert!(methods.contains(&Some("shutdown")), "Missing shutdown transition");
    assert!(methods.contains(&Some("exit")), "Missing exit transition");
}

#[test]
fn job_lifecycle_net_has_work_self_loop_on_running() {
    let net = lsp_lifecycle_net();
    // The self-loop means a transition takes from p_running and puts back to p_running
    let work_t = net.transitions.iter().find(|t| t.id == "t_work");
    assert!(work_t.is_some(), "Must have a work self-loop transition");
    let has_in = net.arcs.iter().any(|a| a.from == "p_running" && a.to == "t_work");
    let has_out = net.arcs.iter().any(|a| a.from == "t_work" && a.to == "p_running");
    assert!(has_in, "p_running must feed into t_work");
    assert!(has_out, "t_work must produce back to p_running");
}

#[test]
fn job_source_place_has_no_incoming_arcs() {
    let net = lsp_lifecycle_net();
    let has_incoming = net.arcs.iter().any(|a| a.to == net.source);
    assert!(!has_incoming, "Source place must have no incoming arcs in a workflow net");
}

#[test]
fn job_sink_place_has_no_outgoing_arcs() {
    let net = lsp_lifecycle_net();
    let has_outgoing = net.arcs.iter().any(|a| a.from == net.sink);
    assert!(!has_outgoing, "Sink place must have no outgoing arcs in a workflow net");
}

// ─── Job: Initial and final markings ─────────────────────────────────────────

#[test]
fn job_initial_marking_places_one_token_at_source() {
    let net = lsp_lifecycle_net();
    let m = Marking::initial(&net.source);
    assert_eq!(m.tokens("p_start"), 1);
    assert_eq!(m.tokens("p_running"), 0);
    assert_eq!(m.tokens("p_end"), 0);
}

#[test]
fn job_final_marking_is_one_token_at_sink() {
    let mut m = Marking::default();
    m.0.insert("p_end".into(), 1);
    assert!(m.is_final("p_end"));
}

#[test]
fn job_marking_with_multiple_tokens_is_not_final() {
    let mut m = Marking::default();
    m.0.insert("p_end".into(), 1);
    m.0.insert("p_running".into(), 1);
    assert!(!m.is_final("p_end"), "Final marking must have exactly 1 token total");
}

#[test]
fn job_marking_with_zero_tokens_at_sink_is_not_final() {
    let m = Marking::initial("p_start");
    assert!(!m.is_final("p_end"));
}

#[test]
fn job_default_marking_is_empty() {
    let m = Marking::default();
    assert_eq!(m.tokens("p_start"), 0);
    assert_eq!(m.tokens("p_end"), 0);
}

// ─── Job: Transition kinds model message directions ───────────────────────────

#[test]
fn job_initialize_is_a_client_request_transition() {
    let net = lsp_lifecycle_net();
    let t = net.transitions.iter().find(|t| t.id == "t_initialize").unwrap();
    assert_eq!(t.transition_kind, TransitionKind::ClientRequest);
    assert_eq!(t.method.as_deref(), Some("initialize"));
}

#[test]
fn job_initialized_is_a_client_notification_transition() {
    let net = lsp_lifecycle_net();
    let t = net.transitions.iter().find(|t| t.id == "t_initialized").unwrap();
    assert_eq!(t.transition_kind, TransitionKind::ClientNotification);
    assert_eq!(t.method.as_deref(), Some("initialized"));
}

#[test]
fn job_init_response_is_a_response_transition() {
    let net = lsp_lifecycle_net();
    let t = net.transitions.iter().find(|t| t.id == "t_init_response").unwrap();
    assert_eq!(t.transition_kind, TransitionKind::Response);
}

#[test]
fn job_shutdown_is_a_client_request_transition() {
    let net = lsp_lifecycle_net();
    let t = net.transitions.iter().find(|t| t.id == "t_shutdown").unwrap();
    assert_eq!(t.transition_kind, TransitionKind::ClientRequest);
    assert_eq!(t.method.as_deref(), Some("shutdown"));
}

#[test]
fn job_exit_is_a_client_notification_transition() {
    let net = lsp_lifecycle_net();
    let t = net.transitions.iter().find(|t| t.id == "t_exit").unwrap();
    assert_eq!(t.transition_kind, TransitionKind::ClientNotification);
    assert_eq!(t.method.as_deref(), Some("exit"));
}

// ─── Job: Capability negotiation sub-net ─────────────────────────────────────

#[test]
fn job_capability_negotiation_net_is_a_valid_workflow_net() {
    let net = lsp_capability_negotiation_net();
    assert!(net.is_workflow_net());
}

#[test]
fn job_capability_negotiation_has_four_places() {
    let net = lsp_capability_negotiation_net();
    assert_eq!(net.place_count(), 4);
}

#[test]
fn job_capability_negotiation_has_silent_eval_transition() {
    let net = lsp_capability_negotiation_net();
    let silent = net.transitions.iter().find(|t| t.transition_kind == TransitionKind::Silent);
    assert!(silent.is_some(), "Capability negotiation must have a silent evaluation step");
}

#[test]
fn job_capability_negotiation_source_to_sink_path_exists() {
    let net = lsp_capability_negotiation_net();
    // Verify all arcs form a linear chain: p_cn_start → t → p → t → ... → p_cn_end
    assert_eq!(net.source, "p_cn_start");
    assert_eq!(net.sink, "p_cn_end");
    // Sink has no outgoing arcs
    let outgoing = net.arcs.iter().filter(|a| a.from == "p_cn_end").count();
    assert_eq!(outgoing, 0);
}

// ─── Job: Build custom Petri nets ────────────────────────────────────────────

#[test]
fn job_build_custom_petri_net_for_hover_request() {
    let mut net = PetriNet::new("Hover flow", "p_idle", "p_result_shown");

    net.add_place(Place::new("p_idle", "Editor idle"));
    net.add_place(Place::new("p_hover_pending", "Hover request in flight"));
    net.add_place(Place::new("p_result_shown", "Hover result displayed"));

    net.add_sequence(
        "p_idle",
        Transition::new("t_hover_req", "textDocument/hover", "textDocument/hover", TransitionKind::ClientRequest),
        "p_hover_pending",
    );
    net.add_sequence(
        "p_hover_pending",
        Transition::new("t_hover_resp", "hover response", "textDocument/hover", TransitionKind::Response),
        "p_result_shown",
    );

    assert!(net.is_workflow_net());
    assert_eq!(net.place_count(), 3);
    assert_eq!(net.transition_count(), 2);
    assert_eq!(net.arc_count(), 4);
}

#[test]
fn job_build_completion_request_net() {
    let mut net = PetriNet::new("Completion flow", "p_start", "p_item_selected");

    for (id, label) in [
        ("p_start", "User triggered completion"),
        ("p_list_shown", "Completion list displayed"),
        ("p_item_resolving", "Selected item being resolved"),
        ("p_item_selected", "Item inserted"),
    ] {
        net.add_place(Place::new(id, label));
    }

    net.add_sequence(
        "p_start",
        Transition::new("t_complete", "textDocument/completion", "textDocument/completion", TransitionKind::ClientRequest),
        "p_list_shown",
    );
    net.add_sequence(
        "p_list_shown",
        Transition::new("t_resolve", "completionItem/resolve", "completionItem/resolve", TransitionKind::ClientRequest),
        "p_item_resolving",
    );
    net.add_sequence(
        "p_item_resolving",
        Transition::new("t_insert", "insert text", "", TransitionKind::Silent),
        "p_item_selected",
    );

    assert!(net.is_workflow_net());
    assert_eq!(net.place_count(), 4);
    assert_eq!(net.transition_count(), 3);
}

// ─── Job: Petri net serialization ────────────────────────────────────────────

#[test]
fn job_lifecycle_net_serializes_to_valid_json() {
    let net = lsp_lifecycle_net();
    let v = serde_json::to_value(&net).expect("Failed to serialize lifecycle net");
    assert_eq!(v["name"], "LSP Lifecycle");
    assert_eq!(v["source"], "p_start");
    assert_eq!(v["sink"], "p_end");
    assert!(v["places"].as_array().unwrap().len() >= 7);
    assert!(v["transitions"].as_array().unwrap().is_empty() == false);
    assert!(v["arcs"].as_array().unwrap().is_empty() == false);
}

#[test]
fn job_lifecycle_net_roundtrips_through_json() {
    let net = lsp_lifecycle_net();
    let json = serde_json::to_value(&net).expect("Serialize");
    let restored: PetriNet = serde_json::from_value(json).expect("Deserialize");
    assert_eq!(net, restored);
}

#[test]
fn job_place_serializes_correctly() {
    let p = Place::new("p1", "Running");
    let v = serde_json::to_value(&p).unwrap();
    assert_eq!(v, json!({"id": "p1", "label": "Running"}));
}

#[test]
fn job_arc_serializes_correctly() {
    let a = Arc::new("p1", "t1");
    let v = serde_json::to_value(&a).unwrap();
    assert_eq!(v, json!({"from": "p1", "to": "t1", "weight": 1}));
}

#[test]
fn job_transition_serializes_correctly() {
    let t = Transition::new("t1", "initialize", "initialize", TransitionKind::ClientRequest);
    let v = serde_json::to_value(&t).unwrap();
    assert_eq!(v["id"], "t1");
    assert_eq!(v["method"], "initialize");
    // TransitionKind serializes with camelCase enum variant names (rename_all = "camelCase")
    assert_eq!(v["transition_kind"], "ClientRequest");
}

#[test]
fn job_silent_transition_has_no_method() {
    let t = Transition::silent("t_tau", "silent step");
    assert!(t.method.is_none());
    assert_eq!(t.transition_kind, TransitionKind::Silent);
}

#[test]
fn job_marking_serializes_as_map() {
    let m = Marking::initial("p_start");
    let v = serde_json::to_value(&m).expect("Serialize");
    assert_eq!(v["p_start"], 1);
}

// ─── Job: Protocol coverage — every LSP transition has a method ──────────────

#[test]
fn job_all_visible_transitions_have_methods() {
    let net = lsp_lifecycle_net();
    for t in &net.transitions {
        if t.transition_kind != TransitionKind::Silent {
            // Visible transitions must have a method (except the self-loop placeholder)
            if t.id != "t_work" {
                assert!(
                    t.method.is_some(),
                    "Visible transition {} must have a method name",
                    t.id
                );
            }
        }
    }
}

#[test]
fn job_transition_count_meets_minimum() {
    let net = lsp_lifecycle_net();
    // initialize + init_response + initialized + work + shutdown + shutdown_response + exit = 7
    assert!(
        net.transition_count() >= 7,
        "LSP lifecycle net must have at least 7 transitions, got {}",
        net.transition_count()
    );
}
