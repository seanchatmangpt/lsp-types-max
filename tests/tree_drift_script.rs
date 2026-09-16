//! Integration tests for `scripts/tree_drift.sh`.
//!
//! The drift reporter characterizes divergence between a dormant copy and its
//! actively-worked sibling tree. These tests pin its contract with
//! deterministic fixture pairs in temp dirs, so the suite never depends on the
//! mutable real sibling and can never be flaky because of concurrent work.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn script_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("scripts")
        .join("tree_drift.sh")
}

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new(tag: &str) -> Fixture {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!(
            "lsp-types-max-drift-test-{}-{}-{}",
            tag,
            std::process::id(),
            n
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create fixture root");
        Fixture { root }
    }

    fn write(&self, rel: &str, contents: &str) -> &Self {
        let path = self.root.join(rel);
        fs::create_dir_all(path.parent().expect("parent")).expect("create fixture dir");
        fs::write(path, contents).expect("write fixture file");
        self
    }

    fn mkdir(&self, rel: &str) -> &Self {
        fs::create_dir_all(self.root.join(rel)).expect("create fixture dir");
        self
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

struct Run {
    status: Option<i32>,
    stdout: String,
    stderr: String,
}

fn run_script(primary: &Path, secondary: &Path, strict: bool) -> Run {
    let mut cmd = Command::new("sh");
    cmd.arg(script_path()).arg(primary).arg(secondary);
    if strict {
        cmd.arg("--strict");
    }
    let out = cmd.output().expect("spawn tree_drift.sh");
    Run {
        status: out.status.code(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

fn class_count(output: &str, class: &str) -> usize {
    for line in output.lines() {
        if let Some(rest) = line.strip_prefix(&format!("CLASS {} ", class)) {
            return rest.trim().parse().expect("class count is an integer");
        }
    }
    panic!("CLASS {} not found in output:\n{}", class, output);
}

fn total_drift(output: &str) -> usize {
    for line in output.lines() {
        if let Some(rest) = line.strip_prefix("TOTAL_DRIFT ") {
            return rest.trim().parse().expect("TOTAL_DRIFT is an integer");
        }
    }
    panic!("TOTAL_DRIFT not found in output:\n{}", output);
}

fn identical_pair(tag: &str) -> (Fixture, Fixture) {
    let a = Fixture::new(&format!("{}-a", tag));
    let b = Fixture::new(&format!("{}-b", tag));
    for f in [&a, &b] {
        f.write("src/lib.rs", "pub fn answer() -> u32 { 42 }\n")
            .write("Cargo.toml", "[package]\nname = \"fixture\"\n")
            .write("docs/guide.md", "# Guide\n");
    }
    (a, b)
}

#[test]
fn identical_trees_report_zero_drift_and_exit_zero() {
    let (a, b) = identical_pair("identical");
    let run = run_script(&a.root, &b.root, false);
    assert_eq!(run.status, Some(0), "stderr: {}", run.stderr);
    assert!(run.stdout.starts_with("TREE DRIFT REPORT"), "{}", run.stdout);
    assert_eq!(class_count(&run.stdout, "ONLY_IN_PRIMARY"), 0);
    assert_eq!(class_count(&run.stdout, "ONLY_IN_SECONDARY"), 0);
    assert_eq!(class_count(&run.stdout, "CONTENT_DIFFERS"), 0);
    assert_eq!(class_count(&run.stdout, "PATH_TYPE_CHANGED"), 0);
    assert_eq!(total_drift(&run.stdout), 0);
    a.cleanup();
    b.cleanup();
}

#[test]
fn identical_trees_strict_mode_still_exits_zero() {
    let (a, b) = identical_pair("strict-ok");
    let run = run_script(&a.root, &b.root, true);
    assert_eq!(run.status, Some(0), "stderr: {}", run.stderr);
    assert_eq!(total_drift(&run.stdout), 0);
    a.cleanup();
    b.cleanup();
}

#[test]
fn classifies_every_divergence_class() {
    let a = Fixture::new("classes-a");
    let b = Fixture::new("classes-b");
    a.write("shared/same.rs", "identical\n")
        .write("shared/changed.rs", "original\n")
        .write("only_primary.txt", "p\n")
        .write("split_path", "i am a file\n");
    b.write("shared/same.rs", "identical\n")
        .write("shared/changed.rs", "mutated\n")
        .write("only_secondary.txt", "s\n")
        .mkdir("split_path/inner")
        .write("split_path/inner/mod.rs", "i am a dir\n");

    let run = run_script(&a.root, &b.root, false);
    assert_eq!(run.status, Some(0), "stderr: {}", run.stderr);

    assert_eq!(class_count(&run.stdout, "ONLY_IN_PRIMARY"), 1);
    assert!(run.stdout.contains("only_primary.txt"), "{}", run.stdout);
    // A file->dir replacement contributes the dir's inner files to
    // ONLY_IN_SECONDARY *and* counts once as PATH_TYPE_CHANGED: the classes
    // describe different facts (new contents vs. kind change at the path).
    assert_eq!(class_count(&run.stdout, "ONLY_IN_SECONDARY"), 2);
    assert!(
        run.stdout.contains("split_path/inner/mod.rs"),
        "dir replacement files are only-in-secondary: {}",
        run.stdout
    );
    assert_eq!(class_count(&run.stdout, "CONTENT_DIFFERS"), 1);
    assert!(run.stdout.contains("shared/changed.rs"), "{}", run.stdout);
    assert_eq!(class_count(&run.stdout, "PATH_TYPE_CHANGED"), 1);
    assert!(
        run.stdout.contains("split_path file->dir"),
        "{}",
        run.stdout
    );
    assert_eq!(total_drift(&run.stdout), 5);
    assert!(
        run.stdout.contains("nogit"),
        "fixture trees are not git repos; GIT_STATE must say so: {}",
        run.stdout
    );
    a.cleanup();
    b.cleanup();
}

#[test]
fn strict_mode_signals_drift_with_nonzero_exit() {
    let a = Fixture::new("strict-drift-a");
    let b = Fixture::new("strict-drift-b");
    a.write("file.txt", "one\n");
    b.write("file.txt", "two\n");

    let run = run_script(&a.root, &b.root, true);
    assert_ne!(run.status, Some(0), "strict mode must fail on drift");
    assert_eq!(total_drift(&run.stdout), 1, "{}", run.stdout);
    a.cleanup();
    b.cleanup();
}

#[test]
fn missing_tree_fails_operationally() {
    let a = Fixture::new("missing-a");
    a.write("file.txt", "x\n");
    let ghost = a.root.join("does-not-exist");
    let run = run_script(&a.root, &ghost, false);
    assert_ne!(run.status, Some(0));
    assert!(
        run.stderr.contains("tree_drift:"),
        "operational failures explain themselves: {}",
        run.stderr
    );
    assert!(!run.stdout.contains("TREE DRIFT REPORT"));
    a.cleanup();
}

#[test]
fn bad_flag_is_a_usage_error() {
    let (a, b) = identical_pair("usage");
    let run = run_script(&a.root, &b.root, false);
    assert_eq!(run.status, Some(0));
    let mut cmd = Command::new("sh");
    cmd.arg(script_path()).arg(&a.root).arg(&b.root).arg("--nonsense");
    let out = cmd.output().expect("spawn tree_drift.sh");
    assert_ne!(out.status.code(), Some(0));
    a.cleanup();
    b.cleanup();
}
