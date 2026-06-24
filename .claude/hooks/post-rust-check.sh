#!/bin/bash
# PostToolUse hook: auto-run cargo check after editing a .rs file.
# Exits 2 to feed errors back to Claude immediately (continueOnBlock: true
# lets the turn continue rather than blocking it).
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | python3 -c "
import json, sys
try:
    d = json.load(sys.stdin)
    print(d.get('tool_input', {}).get('file_path', ''))
except Exception:
    print('')
" 2>/dev/null || echo "")

# Skip non-Rust files
[[ "$FILE_PATH" != *.rs ]] && exit 0

OUTPUT=$(cargo check 2>&1)
STATUS=$?

[ $STATUS -eq 0 ] && exit 0

echo "cargo check failed after editing $(basename "$FILE_PATH"):"
echo "$OUTPUT"
exit 2
