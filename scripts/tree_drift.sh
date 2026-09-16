#!/bin/sh
# tree_drift.sh — executable drift characterization between two working trees.
#
# Purpose: a dormant copy (PRIMARY) and its actively-worked sibling (SECONDARY)
# share history; drift between them must be observable and classified, never
# silently merged. This script makes that drift characterization EXECUTABLE.
#
# Read-only on BOTH trees: no file inside either tree is written or removed.
# GIT_OPTIONAL_LOCKS=0 keeps `git status` from refreshing the index; scratch
# files live in a mktemp dir under $TMPDIR and are removed on exit.
#
# Usage:
#   sh scripts/tree_drift.sh PRIMARY SECONDARY [--strict]
#
# Exit codes:
#   0  report produced (see TOTAL_DRIFT for the divergence verdict)
#   1  operational failure (unreadable tree, bad usage, missing tool), OR
#      drift detected in --strict (alarm) mode
#
# Divergence classes reported:
#   GIT_STATE         committed heads + dirty-file counts (informational)
#   ONLY_IN_PRIMARY   relative path present only in PRIMARY
#   ONLY_IN_SECONDARY relative path present only in SECONDARY
#   CONTENT_DIFFERS   same relative path, different bytes
#   PATH_TYPE_CHANGED same relative path, file <-> directory kind change
#
# Excluded from comparison: .git/ and any directory named target/ (build
# artifacts). Symlinks are not followed and not classified.

set -u

export GIT_OPTIONAL_LOCKS=0

STRICT=0
case $# in
  2) ;;
  3) [ "$3" = "--strict" ] || { echo "usage: $0 PRIMARY SECONDARY [--strict]" >&2; exit 1; }; STRICT=1 ;;
  *) echo "usage: $0 PRIMARY SECONDARY [--strict]" >&2; exit 1 ;;
esac

PRIMARY=$1
SECONDARY=$2

fail() { echo "tree_drift: $1" >&2; exit 1; }

[ -d "$PRIMARY" ] || fail "PRIMARY not a directory: $PRIMARY"
[ -d "$SECONDARY" ] || fail "SECONDARY not a directory: $SECONDARY"
command -v git >/dev/null 2>&1 || fail "git not available"

SCRATCH=$(mktemp -d "${TMPDIR:-/tmp}/tree_drift.XXXXXX") || fail "mktemp failed"
trap 'rm -rf "$SCRATCH"' EXIT INT TERM

P_ABS=$(cd "$PRIMARY" && pwd) || fail "cannot resolve PRIMARY"
S_ABS=$(cd "$SECONDARY" && pwd) || fail "cannot resolve SECONDARY"

git_head() { git -C "$1" rev-parse HEAD 2>/dev/null || echo nogit; }
git_dirty() { git -C "$1" status --porcelain 2>/dev/null | wc -l | tr -d ' ' || true; }

P_HEAD=$(git_head "$P_ABS"); P_DIRTY=$(git_dirty "$P_ABS")
S_HEAD=$(git_head "$S_ABS"); S_DIRTY=$(git_dirty "$S_ABS")

# Relative file lists, .git/ and target/ pruned, newline-safe via IFS loops.
list_files() {
  (cd "$1" && find . \( -name .git -o -name target \) -prune -o -type f -print) \
    | sed 's|^\./||' | LC_ALL=C sort
}

list_files "$P_ABS" > "$SCRATCH/primary.files" || fail "cannot list PRIMARY"
list_files "$S_ABS" > "$SCRATCH/secondary.files" || fail "cannot list SECONDARY"

LC_ALL=C comm -23 "$SCRATCH/primary.files" "$SCRATCH/secondary.files" > "$SCRATCH/only_p"
LC_ALL=C comm -13 "$SCRATCH/primary.files" "$SCRATCH/secondary.files" > "$SCRATCH/only_s"
LC_ALL=C comm -12 "$SCRATCH/primary.files" "$SCRATCH/secondary.files" > "$SCRATCH/common"

kind_of() {
  if [ -d "$1" ]; then echo dir
  elif [ -f "$1" ]; then echo file
  else echo other
  fi
}

ONLY_P=0; ONLY_S=0; CONTENT=0; TYPECHANGED=0
: > "$SCRATCH/type_changed"
: > "$SCRATCH/only_p_list"
: > "$SCRATCH/only_s_list"

while IFS= read -r p; do
  [ -n "$p" ] || continue
  if [ -e "$S_ABS/$p" ]; then
    printf '%s %s->%s\n' "$p" "$(kind_of "$P_ABS/$p")" "$(kind_of "$S_ABS/$p")" >> "$SCRATCH/type_changed"
    TYPECHANGED=$((TYPECHANGED + 1))
  else
    ONLY_P=$((ONLY_P + 1))
    printf '%s\n' "$p" >> "$SCRATCH/only_p_list"
  fi
done < "$SCRATCH/only_p"

while IFS= read -r p; do
  [ -n "$p" ] || continue
  if [ -e "$P_ABS/$p" ]; then
    # Already counted from the primary side as a type change.
    :
  else
    ONLY_S=$((ONLY_S + 1))
    printf '%s\n' "$p" >> "$SCRATCH/only_s_list"
  fi
done < "$SCRATCH/only_s"

: > "$SCRATCH/content_differs"
while IFS= read -r p; do
  [ -n "$p" ] || continue
  if ! cmp -s "$P_ABS/$p" "$S_ABS/$p"; then
    printf '  %s\n' "$p" >> "$SCRATCH/content_differs"
    CONTENT=$((CONTENT + 1))
  fi
done < "$SCRATCH/common"

if [ "$P_HEAD" = "$S_HEAD" ]; then GIT_STATE=same-heads; else GIT_STATE=diverged-heads; fi
[ "$P_HEAD" = nogit ] || [ "$S_HEAD" = nogit ] && GIT_STATE="$GIT_STATE (nogit tree present)"

TOTAL=$((ONLY_P + ONLY_S + CONTENT + TYPECHANGED))

echo "TREE DRIFT REPORT"
echo "primary:   $P_ABS @ $P_HEAD dirty=$P_DIRTY"
echo "secondary: $S_ABS @ $S_HEAD dirty=$S_DIRTY"
echo "CLASS GIT_STATE $GIT_STATE"
echo "CLASS ONLY_IN_PRIMARY $ONLY_P"
while IFS= read -r line; do echo "  $line"; done < "$SCRATCH/only_p_list"
echo "CLASS ONLY_IN_SECONDARY $ONLY_S"
while IFS= read -r line; do echo "  $line"; done < "$SCRATCH/only_s_list"
echo "CLASS CONTENT_DIFFERS $CONTENT"
while IFS= read -r line; do echo "  $line"; done < "$SCRATCH/content_differs"
echo "CLASS PATH_TYPE_CHANGED $TYPECHANGED"
while IFS= read -r line; do echo "  $line"; done < "$SCRATCH/type_changed"
echo "TOTAL_DRIFT $TOTAL"

if [ "$STRICT" -eq 1 ] && [ "$TOTAL" -gt 0 ]; then
  exit 1
fi
exit 0
