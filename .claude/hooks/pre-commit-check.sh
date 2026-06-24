#!/bin/bash
# PreToolUse hook: block git commit if cargo check fails.
# Exits 2 to block the tool call entirely.
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | python3 -c "
import json, sys
try:
    d = json.load(sys.stdin)
    print(d.get('tool_input', {}).get('command', ''))
except Exception:
    print('')
" 2>/dev/null || echo "")

# Only intercept git commit invocations
[[ "$COMMAND" != *"git commit"* ]] && exit 0

OUTPUT=$(cargo check 2>&1)
STATUS=$?

[ $STATUS -eq 0 ] && exit 0

echo "Blocked: cargo check must pass before committing." >&2
echo "$OUTPUT" >&2
exit 2
