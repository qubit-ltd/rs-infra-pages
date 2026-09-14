// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Verifies public GitHub Pages artifact creation.

use std::fs;

use flate2::read::GzDecoder;
use qubit_infra_pages::create_artifact;
use tar::Archive;
use tempfile::tempdir;

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
