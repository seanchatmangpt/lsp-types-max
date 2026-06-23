#!/bin/bash
set -euo pipefail

BRANCH=$(git branch --show-current 2>/dev/null || echo "detached")
COMMIT=$(git log --oneline -1 2>/dev/null || echo "no commits")
TEST_COUNT=$(cargo test -- --list 2>/dev/null | grep -c '^test ' || echo "unknown")

CHECK_OUTPUT=$(cargo check 2>&1)
if [ $? -eq 0 ]; then
    BUILD_STATUS="HEALTHY — cargo check passed"
else
    BUILD_STATUS="BROKEN — cargo check failed:
$CHECK_OUTPUT"
fi

python3 - "$BRANCH" "$COMMIT" "$BUILD_STATUS" "$TEST_COUNT" <<'PYEOF'
import json, sys

branch = sys.argv[1]
commit = sys.argv[2]
build  = sys.argv[3]
tests  = sys.argv[4]

context = f"""lsp-types-max session context
Branch : {branch}
Commit : {commit}
Build  : {build}
Tests  : {tests} registered (run: cargo test)

Architecture quick-ref
  src/lib.rs              flat re-export root (use lsp_types_max::*)
  src/types/              Position, Range, Location, auxiliary
  src/protocol/           JSON-RPC RequestMessage/ResponseMessage/NotificationMessage
  src/request.rs          lsp_request!("method") macro → type alias, not value
  src/notification.rs     lsp_notification!("method") macro → type alias
  src/lsif.rs             LSIF 0.6.0 vertices + edges (IDs are i32, not NumberOrString)
  src/open_set_types.rs   SemanticTokenTypes/Modifiers (newtype wrappers around String)
  agent1..agent10         LSP 3.18 feature modules (all #[allow(unused_imports)])

Serde gotchas
  • Every struct has #[serde(rename_all = "camelCase")]
  • Optional fields use #[serde(skip_serializing_if = "Option::is_none")]
  • Untagged enums (OneOf<A,B>, OneOf3<A,B,C>, NumberOrString) are order-sensitive
  • PartialResultParams needs is_partial_result_token_null: bool
  • DeclarationTag.deprecated always serialises (no skip_serializing_if) — include in JSON
  • WorkspaceSymbolResponse: use WorkspaceLocation (no range) for roundtrip safety
  • UniquenessLevel: no Workspace variant; use Global/Document/Project/Group/Scheme

Test files
  tests/e2e_spec_3_18.rs      174 tests  LSP 3.18 feature E2E
  tests/base_spec.rs            10 tests  Base Protocol 0.9 JSON-RPC
  tests/jtbd_all_requests.rs   124 tests  all request/notification param+result types
  tests/jtbd_lsp_3_18.rs        51 tests  LSP 3.18 JTBD scenarios
  tests/jtbd_lsif_0_6.rs        53 tests  LSIF 0.6.0 vertex/edge types

Hooks active in this session
  PostToolUse(Edit|Write on *.rs) → cargo check auto-runs, errors fed back immediately
  PreToolUse(Bash with git commit) → blocked if cargo check fails
"""

output = {
    "hookSpecificOutput": {
        "hookEventName": "SessionStart",
        "additionalContext": context,
        "sessionTitle": f"lsp-types-max | {branch}"
    }
}
print(json.dumps(output))
PYEOF
