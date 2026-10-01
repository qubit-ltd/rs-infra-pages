// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Verifies public GitHub Pages artifact creation.

use std::fs;
use std::sync::Mutex;

use flate2::read::GzDecoder;
use qubit_infra_pages::create_artifact;
use qubit_infra_pages::deploy_github_pages;
use qubit_infra_pages::publish_github_pages;
use tar::Archive;
use tempfile::tempdir;

static CURRENT_DIR_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_creates_github_pages_artifact_without_external_tools() {
    let project = tempdir().expect("temporary project directory should be created");
    fs::write(project.path().join("index.html"), "hello").expect("index page should be written");
    let artifact = project.path().join("pages.tar.gz");
    create_artifact(project.path(), &artifact).expect("pages artifact should be created");

    let file = fs::File::open(artifact).expect("pages artifact should be readable");
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    let names: Vec<_> = archive
        .entries()
        .expect("archive entries should be readable")
        .map(|entry| {
            entry
                .expect("archive entry should be readable")
                .path()
                .expect("archive entry path should be valid")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert!(names.iter().any(|name| name.ends_with("index.html")));
}

#[test]
fn deploy_creates_artifact_at_the_requested_path() {
    let project = tempdir().expect("temporary project directory should be created");
    let output = project.path().join("public");
    fs::create_dir(&output).expect("pages output directory should be created");
    fs::write(output.join("index.html"), "hello").expect("index page should be written");
    let artifact = project.path().join("artifacts/pages.tar.gz");

    deploy_github_pages(&output, &artifact).expect("deploy artifact should be created");

    assert!(artifact.is_file());
}

#[test]
fn publish_creates_default_artifact_in_the_current_directory() {
    let _guard = CURRENT_DIR_LOCK
        .lock()
        .expect("current directory lock should not be poisoned");
    let project = tempdir().expect("temporary project directory should be created");
    let output = project.path().join("public");
    let working_directory = project.path().join("working");
    fs::create_dir(&output).expect("pages output directory should be created");
    fs::create_dir(&working_directory).expect("working directory should be created");
    fs::write(output.join("index.html"), "hello").expect("index page should be written");
    let original_directory = std::env::current_dir().expect("current directory should be readable");
    std::env::set_current_dir(&working_directory).expect("working directory should be selected");

    let result = publish_github_pages(&output);

    std::env::set_current_dir(original_directory).expect("original directory should be restored");
    result.expect("default pages artifact should be created");
    assert!(working_directory.join("pages-artifact.tar.gz").is_file());
}
