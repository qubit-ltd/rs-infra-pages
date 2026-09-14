// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Verifies the public site-building behavior.

use std::fs;

use qubit_infra_pages::build;
use tempfile::tempdir;

#[test]
fn test_builds_default_bilingual_site() {
    let project = tempdir().expect("temporary project directory should be created");
    fs::write(project.path().join("README.md"), "# Hello\n\nRust project.")
        .expect("English README should be written");
    fs::write(
        project.path().join("README.zh_CN.md"),
        "# 你好\n\nRust 项目。",
    )
    .expect("Chinese README should be written");

    let output = project.path().join("public");
    build(project.path(), &output).expect("default site should be built");

    assert!(output.join("index.html").is_file());
    assert!(output.join("zh_CN/index.html").is_file());
}

#[test]
fn test_builds_configured_assets_and_optional_reports() {
    let project = tempdir().expect("temporary project directory should be created");
    fs::create_dir_all(project.path().join(".infra/ci"))
        .expect("configuration directory should be created");
    fs::write(project.path().join("README.md"), "# Hello").expect("README should be written");
    fs::create_dir(project.path().join("assets")).expect("asset directory should be created");
    fs::write(project.path().join("assets/site.css"), "body {}").expect("asset should be written");
    fs::write(project.path().join("ci-summary.json"), "{}\n").expect("metadata should be written");
    fs::write(
        project.path().join(".infra/ci/pages.json"),
        r#"{"languages":{"en":{"readme":"README.md","output":"index.html","label":"English"}},"assets":["assets"],"metadata":{"source":"ci-summary.json","output":"ci-summary.json"}}"#,
    )
    .expect("configuration should be written");

    let output = project.path().join("public");
    build(project.path(), &output).expect("configured site should be built");

    assert!(output.join("assets/site.css").is_file());
    assert_eq!(
        fs::read_to_string(output.join("ci-summary.json"))
            .expect("copied metadata should be readable"),
        "{}\n"
    );
}
