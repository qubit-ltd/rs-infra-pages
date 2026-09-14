// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Renders configured project README files into a static site.

use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;

use crate::config::Config;
use crate::config::load;
use crate::html::html_attr;
use crate::html::html_text;
use crate::html::markdown_to_html;
use crate::html::page;

/// Builds localized HTML pages and copies configured project assets.
///
/// The output path is resolved relative to `project` unless it is absolute. Existing
/// output is removed before rendering. The default language README must exist; other
/// missing language files are skipped. Filesystem and configuration errors are returned.
///
/// # Errors
///
/// Returns an error when configuration cannot be read or parsed, the required README
/// is missing, or a page or asset cannot be read or written.
pub fn build(project: &Path, output: &Path) -> Result<()> {
    let config = load(project)?;
    if config.languages.is_empty() {
        bail!("pages configuration must define at least one language");
    }
    let output = if output.is_absolute() {
        output.to_owned()
    } else {
        project.join(output)
    };
    if output.exists() {
        fs::remove_dir_all(&output)
            .with_context(|| format!("failed to clean pages output {}", output.display()))?;
    }
    fs::create_dir_all(&output)?;
    let mut links = String::new();
    for (language, definition) in &config.languages {
        let source = project.join(&definition.readme);
        if !source.is_file() {
            if language == &config.default_language {
                bail!(
                    "README for language {language} was not found: {}",
                    source.display()
                );
            }
            continue;
        }
        let destination = output.join(&definition.output);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let title = config.site_title.as_deref().unwrap_or(language);
        let body = markdown_to_html(&fs::read_to_string(source)?, title);
        fs::write(&destination, page(title, &body, &links))?;
        links.push_str(&format!(
            "<a href=\"{}\">{}</a> ",
            html_attr(&definition.output),
            html_text(&definition.label)
        ));
    }
    copy_assets(project, &output, &config)?;
    println!("Built site at {}", output.display());
    Ok(())
}

/// Copies explicitly configured and standard report assets into the generated site.
///
/// Missing optional assets are ignored. Filesystem errors from discovered assets are
/// returned to the caller.
///
/// # Errors
///
/// Returns an error when an asset exists but cannot be read, created, or copied.
fn copy_assets(project: &Path, output: &Path, config: &Config) -> Result<()> {
    for asset in &config.assets {
        let source = project.join(asset);
        if source.exists() {
            copy_path(
                &source,
                &output.join(Path::new(asset).file_name().unwrap_or_default()),
            )?;
        }
    }
    let defaults = [
        ("target/llvm-cov/html", "coverage"),
        ("coverage-badge.json", "coverage-badge.json"),
        ("ci-summary.json", "ci-summary.json"),
    ];
    for (source, destination) in defaults {
        if !config.assets.iter().any(|asset| asset == source) {
            let source_path = project.join(source);
            if source_path.exists() {
                copy_path(&source_path, &output.join(destination))?;
            }
        }
    }
    for item in [&config.coverage, &config.coverage_badge, &config.metadata]
        .into_iter()
        .flatten()
    {
        let source = project.join(&item.source);
        if source.exists() {
            copy_path(&source, &output.join(&item.output))?;
        }
    }
    Ok(())
}

/// Recursively copies a file or directory to the destination path.
///
/// # Errors
///
/// Returns an error when the source cannot be read or the destination cannot be created
/// or written.
fn copy_path(source: &Path, destination: &Path) -> Result<()> {
    if source.is_dir() {
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            copy_path(&entry.path(), &destination.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, destination)?;
    }
    Ok(())
}
