// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Creates compressed artifacts for GitHub Pages deployment.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;

/// Archives the generated pages directory as a gzip-compressed tar artifact.
///
/// The output directory must already exist and be a directory. Parent directories
/// for the artifact are created as needed.
///
/// # Parameters
///
/// * `output` - Directory containing the generated pages to archive.
/// * `artifact` - Destination path for the compressed archive.
///
/// # Returns
///
/// Returns the destination path after the archive has been flushed and synchronized.
///
/// # Errors
///
/// Returns an error when the pages directory is absent or not a directory, or when
/// the archive cannot be created, populated, flushed, or synchronized.
pub fn create_artifact(output: &Path, artifact: &Path) -> Result<PathBuf> {
    let output = output
        .canonicalize()
        .context("pages output directory was not found")?;
    if !output.is_dir() {
        bail!("pages output is not a directory: {}", output.display());
    }
    if let Some(parent) = artifact.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = fs::File::create(artifact)?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut archive = Builder::new(encoder);
    archive.append_dir_all(".", &output)?;
    let encoder = archive.into_inner()?;
    encoder.finish()?.sync_all()?;
    println!("Prepared GitHub Pages artifact at {}", artifact.display());
    Ok(artifact.to_owned())
}

/// Creates the default pages artifact in the current working directory.
///
/// # Parameters
///
/// * `output` - Directory containing the generated pages to archive.
///
/// # Returns
///
/// Returns `()` after `pages-artifact.tar.gz` has been created and synchronized.
///
/// # Errors
///
/// Returns the archive creation errors from [`create_artifact`].
pub fn publish_github_pages(output: &Path) -> Result<()> {
    create_artifact(output, Path::new("pages-artifact.tar.gz")).map(|_| ())
}

/// Creates a pages artifact at the requested path for GitHub Pages Actions.
///
/// # Parameters
///
/// * `output` - Directory containing the generated pages to archive.
/// * `artifact` - Destination path for the compressed archive.
///
/// # Returns
///
/// Returns `()` after the archive has been created and synchronized.
///
/// # Errors
///
/// Returns the archive creation errors from [`create_artifact`].
pub fn deploy_github_pages(output: &Path, artifact: &Path) -> Result<()> {
    create_artifact(output, artifact)?;
    println!("GitHub Pages deploy artifact is ready for actions/deploy-pages");
    Ok(())
}
