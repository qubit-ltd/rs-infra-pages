//! Verifies that every CLI invocation ends with an explicit status message.

use std::fs;
use std::process::Command;

use tempfile::tempdir;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rs-infra-pages"))
        .args(args)
        .output()
        .expect("pages CLI should start")
}

#[test]
fn successful_build_prints_final_status() {
    let project = tempdir().expect("temporary project directory should be created");
    fs::write(project.path().join("README.md"), "# Hello").expect("README should be written");

    let output = run(&[
        "--project",
        project
            .path()
            .to_str()
            .expect("project path should be UTF-8"),
        "build",
    ]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rs-infra-pages: SUCCESS: built site at"));
}

#[test]
fn failed_artifact_prints_final_status() {
    let project = tempdir().expect("temporary project directory should be created");

    let output = run(&[
        "--project",
        project
            .path()
            .to_str()
            .expect("project path should be UTF-8"),
        "artifact",
    ]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("rs-infra-pages: FAILURE (artifact):"));
}
