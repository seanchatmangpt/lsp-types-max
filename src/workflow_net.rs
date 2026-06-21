/// Workflow net (Petri net) representation of the LSP protocol lifecycle.
///
/// Inspired by Dr. Wil van der Aalst's Workflow Net formalism:
/// a Petri net where every transition has a path from source to sink.
///
/// This models the LSP protocol as a formal process model:
/// - Places = protocol states (conditions that can hold a token)
/// - Transitions = LSP messages that fire when their input places are marked
/// - Tokens = current state markers
///
/// The canonical LSP lifecycle net has:
///   p_start → [t_initialize] → p_initializing → [t_response] → p_awaiting_initialized
///           → [t_initialized] → p_running → [t_any_work]* → p_running
///           → [t_shutdown] → p_shutting_down → [t_shutdown_response] → p_awaiting_exit
///           → [t_exit] → p_end
use serde::{Deserialize, Serialize};

/// A place in a Petri net — represents a condition that can hold tokens.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Place {
    pub id: String,
    pub label: String,
}

impl Place {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into() }
    }
}

/// A transition in a Petri net — fires when all input places are marked.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Transition {
    pub id: String,
    pub label: String,
    /// The LSP method this transition corresponds to.
    pub method: Option<String>,
    pub transition_kind: TransitionKind,
}

impl Transition {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        method: impl Into<String>,
        kind: TransitionKind,
    ) -> Self {
        let m: String = method.into();
        let method = if m.is_empty() { None } else { Some(m) };
        Self { id: id.into(), label: label.into(), method, transition_kind: kind }
    }

    pub fn silent(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into(), method: None, transition_kind: TransitionKind::Silent }
    }
}

/// The kind of transition — visible (corresponds to an LSP message) or silent (τ).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionKind {
    /// Corresponds to a client→server request
    ClientRequest,
    /// Corresponds to a server→client request (server-initiated)
    ServerRequest,
    /// Corresponds to a client→server notification
    ClientNotification,
    /// Corresponds to a server→client notification
    ServerNotification,
    /// A response to a client request
    Response,
    /// Silent transition (τ) — internal, not observable
    Silent,
}

/// A directed arc connecting a Place to a Transition (or vice versa).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arc {
    /// Source node ID (place or transition)
    pub from: String,
    /// Target node ID (place or transition)
    pub to: String,
    /// Arc weight (number of tokens consumed/produced), usually 1
    pub weight: u32,
}

impl Arc {
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self { from: from.into(), to: to.into(), weight: 1 }
    }
}

/// A Petri net — the formal process model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PetriNet {
    pub name: String,
    pub places: Vec<Place>,
    pub transitions: Vec<Transition>,
    pub arcs: Vec<Arc>,
    /// ID of the source place (initial marking has 1 token here)
    pub source: String,
    /// ID of the sink place (final marking has 1 token here)
    pub sink: String,
}

impl PetriNet {
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        sink: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            places: vec![],
            transitions: vec![],
            arcs: vec![],
            source: source.into(),
            sink: sink.into(),
        }
    }

    pub fn add_place(&mut self, place: Place) {
        self.places.push(place);
    }

    pub fn add_transition(&mut self, t: Transition) {
        self.transitions.push(t);
    }

    /// Add arc from place → transition
    pub fn p_to_t(&mut self, place: impl Into<String>, transition: impl Into<String>) {
        self.arcs.push(Arc::new(place, transition));
    }

    /// Add arc from transition → place
    pub fn t_to_p(&mut self, transition: impl Into<String>, place: impl Into<String>) {
        self.arcs.push(Arc::new(transition, place));
    }

    /// Shorthand: add a transition with its single input place and single output place.
    pub fn add_sequence(
        &mut self,
        from_place: impl Into<String>,
        transition: Transition,
        to_place: impl Into<String>,
    ) {
        let from_place = from_place.into();
        let to_place = to_place.into();
        let t_id = transition.id.clone();
        self.transitions.push(transition);
        self.arcs.push(Arc::new(from_place, t_id.clone()));
        self.arcs.push(Arc::new(t_id, to_place));
    }

    pub fn place_count(&self) -> usize {
        self.places.len()
    }

    pub fn transition_count(&self) -> usize {
        self.transitions.len()
    }

    pub fn arc_count(&self) -> usize {
        self.arcs.len()
    }

    /// Returns true if this is a valid workflow net:
    /// - Exactly one source place (no incoming arcs)
    /// - Exactly one sink place (no outgoing arcs)
    /// - Every node on a path from source to sink
    pub fn is_workflow_net(&self) -> bool {
        // Simplified check: source has no incoming arcs, sink has no outgoing arcs
        let source_has_incoming = self.arcs.iter().any(|a| a.to == self.source);
        let sink_has_outgoing = self.arcs.iter().any(|a| a.from == self.sink);
        !source_has_incoming && !sink_has_outgoing
    }
}

/// A marking of a Petri net — maps place IDs to token counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Marking(pub std::collections::HashMap<String, u32>);

impl Marking {
    pub fn initial(source: impl Into<String>) -> Self {
        let mut m = std::collections::HashMap::new();
        m.insert(source.into(), 1);
        Marking(m)
    }

    pub fn tokens(&self, place_id: &str) -> u32 {
        self.0.get(place_id).copied().unwrap_or(0)
    }

    pub fn is_final(&self, sink_id: &str) -> bool {
        self.tokens(sink_id) == 1 && self.0.values().sum::<u32>() == 1
    }
}

/// Build the canonical LSP lifecycle workflow net.
///
/// This is the formal process model for the LSP protocol. Every valid LSP session
/// corresponds to a firing sequence from the initial marking to the final marking.
pub fn lsp_lifecycle_net() -> PetriNet {
    let mut net = PetriNet::new("LSP Lifecycle", "p_start", "p_end");

    // Places
    for (id, label) in [
        ("p_start", "Ready (no session)"),
        ("p_initializing", "Initialize request sent"),
        ("p_awaiting_initialized", "Initialize response received"),
        ("p_running", "Session running"),
        ("p_shutting_down", "Shutdown request sent"),
        ("p_awaiting_exit", "Shutdown response received"),
        ("p_end", "Session ended"),
    ] {
        net.add_place(Place::new(id, label));
    }

    // Lifecycle transitions
    net.add_sequence(
        "p_start",
        Transition::new("t_initialize", "initialize", "initialize", TransitionKind::ClientRequest),
        "p_initializing",
    );
    net.add_sequence(
        "p_initializing",
        Transition::new("t_init_response", "initialize (response)", "initialize", TransitionKind::Response),
        "p_awaiting_initialized",
    );
    net.add_sequence(
        "p_awaiting_initialized",
        Transition::new("t_initialized", "initialized", "initialized", TransitionKind::ClientNotification),
        "p_running",
    );

    // Self-loop on p_running for all normal work
    let work_id = "t_work";
    net.transitions.push(Transition::new(work_id, "any LSP operation", "", TransitionKind::ClientRequest));
    net.p_to_t("p_running", work_id);
    net.t_to_p(work_id, "p_running");

    net.add_sequence(
        "p_running",
        Transition::new("t_shutdown", "shutdown", "shutdown", TransitionKind::ClientRequest),
        "p_shutting_down",
    );
    net.add_sequence(
        "p_shutting_down",
        Transition::new("t_shutdown_response", "shutdown (response)", "shutdown", TransitionKind::Response),
        "p_awaiting_exit",
    );
    net.add_sequence(
        "p_awaiting_exit",
        Transition::new("t_exit", "exit", "exit", TransitionKind::ClientNotification),
        "p_end",
    );

    net
}

/// Build a Petri net model of LSP capability negotiation.
///
/// Models the sub-process within the initialize handshake where capabilities
/// are negotiated between client and server.
pub fn lsp_capability_negotiation_net() -> PetriNet {
    let mut net = PetriNet::new("LSP Capability Negotiation", "p_cn_start", "p_cn_end");

    for (id, label) in [
        ("p_cn_start", "Client capabilities available"),
        ("p_cn_client_sent", "Client capabilities transmitted"),
        ("p_cn_server_evaluated", "Server evaluated client capabilities"),
        ("p_cn_end", "Negotiation complete — server capabilities returned"),
    ] {
        net.add_place(Place::new(id, label));
    }

    net.add_sequence(
        "p_cn_start",
        Transition::new("t_cn_send", "send ClientCapabilities", "initialize", TransitionKind::ClientRequest),
        "p_cn_client_sent",
    );
    net.add_sequence(
        "p_cn_client_sent",
        Transition::new("t_cn_eval", "evaluate capabilities", "", TransitionKind::Silent),
        "p_cn_server_evaluated",
    );
    net.add_sequence(
        "p_cn_server_evaluated",
        Transition::new("t_cn_respond", "return ServerCapabilities", "initialize", TransitionKind::Response),
        "p_cn_end",
    );

    net
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsp_lifecycle_net_is_valid_workflow_net() {
        let net = lsp_lifecycle_net();
        assert!(net.is_workflow_net(), "LSP lifecycle net must be a valid workflow net");
        assert_eq!(net.source, "p_start");
        assert_eq!(net.sink, "p_end");
        // 7 places, 8 transitions (6 lifecycle + 1 work self-loop + 1 already counted)
        assert_eq!(net.place_count(), 7);
        assert!(net.transition_count() >= 7);
    }

    #[test]
    fn lsp_lifecycle_net_has_correct_places() {
        let net = lsp_lifecycle_net();
        let place_ids: Vec<&str> = net.places.iter().map(|p| p.id.as_str()).collect();
        assert!(place_ids.contains(&"p_start"));
        assert!(place_ids.contains(&"p_running"));
        assert!(place_ids.contains(&"p_end"));
        assert!(place_ids.contains(&"p_initializing"));
        assert!(place_ids.contains(&"p_shutting_down"));
    }

    #[test]
    fn initial_marking_has_token_at_source() {
        let net = lsp_lifecycle_net();
        let marking = Marking::initial(&net.source);
        assert_eq!(marking.tokens("p_start"), 1);
        assert_eq!(marking.tokens("p_running"), 0);
        assert!(!marking.is_final("p_end"));
    }

    #[test]
    fn final_marking_at_sink() {
        let mut m = Marking::default();
        m.0.insert("p_end".into(), 1);
        assert!(m.is_final("p_end"));
    }

    #[test]
    fn workflow_net_source_has_no_incoming_arcs() {
        let net = lsp_lifecycle_net();
        let incoming_to_source = net.arcs.iter().any(|a| a.to == net.source);
        assert!(!incoming_to_source, "Source place must have no incoming arcs");
    }

    #[test]
    fn workflow_net_sink_has_no_outgoing_arcs() {
        let net = lsp_lifecycle_net();
        let outgoing_from_sink = net.arcs.iter().any(|a| a.from == net.sink);
        assert!(!outgoing_from_sink, "Sink place must have no outgoing arcs");
    }

    #[test]
    fn capability_negotiation_net_is_valid() {
        let net = lsp_capability_negotiation_net();
        assert!(net.is_workflow_net());
        assert_eq!(net.place_count(), 4);
    }

    #[test]
    fn petri_net_serializes_to_json() {
        let net = lsp_lifecycle_net();
        let json = serde_json::to_value(&net).expect("Failed to serialize PetriNet");
        assert!(json["places"].is_array());
        assert!(json["transitions"].is_array());
        assert!(json["arcs"].is_array());
        assert_eq!(json["source"], "p_start");
        assert_eq!(json["sink"], "p_end");
    }

    #[test]
    fn transition_kinds_cover_all_directions() {
        let net = lsp_lifecycle_net();
        let has_client_req = net.transitions.iter().any(|t| t.transition_kind == TransitionKind::ClientRequest);
        let has_notification = net.transitions.iter().any(|t| t.transition_kind == TransitionKind::ClientNotification);
        let has_response = net.transitions.iter().any(|t| t.transition_kind == TransitionKind::Response);
        assert!(has_client_req);
        assert!(has_notification);
        assert!(has_response);
    }

    #[test]
    fn place_new_sets_id_and_label() {
        let p = Place::new("p1", "my place");
        assert_eq!(p.id, "p1");
        assert_eq!(p.label, "my place");
    }

    #[test]
    fn arc_new_creates_weight_one_arc() {
        let a = Arc::new("p1", "t1");
        assert_eq!(a.from, "p1");
        assert_eq!(a.to, "t1");
        assert_eq!(a.weight, 1);
    }
}
