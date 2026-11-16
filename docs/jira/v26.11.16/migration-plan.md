# lsp-types-max migration plan v26.11.16

Standing: UNVERIFIED

## Identity
- Path: /Users/sac/dev/lsp-types-max
- Remote: https://github.com/seanchatmangpt/lsp-types-max.git (origin)
- Branch: master@61a9d3f — last commit 2026-09-29 — 767 commits — ~0 MB (.git 2.5 MB)
- Languages/build: Rust crate (Cargo.toml, Justfile, release.toml, release.sh), scripts/
- Nested repos: none

## Observed state
- Dirty 0; untracked 6: docs/ (docs/jira/v26.9.19 etc.)
- ahead 13 / behind 0 origin/master — 13 unpushed commits on master
- stash 0
- Unpushed local branches 2: exp/completion-item-defaults-resolve, exp/drift-characterization
  (both appear merged into master)
- Build dirs: none

## What it is
Fork/extension of gluon-lang `lsp-types` (LSP 3.16/3.17 Rust types) with conformance
matrix, drift-characterization scripts and a type-authority report.

## Classification
ACTIVE (census agrees).

## Migration risk
- 13 unpushed commits on master (drift characterization, itemDefaults semantics, release bump)
- Untracked docs/ history
Rate: MEDIUM

## Recommended action
COMMIT-PUSH
1. `cd /Users/sac/dev/lsp-types-max && git add docs && git commit -m "docs(jira): track"`
2. `git push origin master`
3. `git push -u origin exp/completion-item-defaults-resolve exp/drift-characterization`
   (or delete after confirming merged: `git branch --merged master`)
4. Nothing to clean; clone fresh on the new machine

## First commands on the new machine
`cargo check && cargo test` (stated, not run).

## See Also
- ../v26.9.19/
- /Users/sac/docs/jira/v26.11.16/README.md
