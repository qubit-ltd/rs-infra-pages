// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Defines the JSON configuration model used by the site builder.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use crate::config::copy_config::CopyConfig;
use crate::config::language::Language;

mod copy_config;
mod language;

/// Stores optional site-wide settings and language-specific page definitions.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    /// Overrides the language name used as the generated site title.
    pub(crate) site_title: Option<String>,
    /// Identifies the language whose README is required.
    pub(crate) default_language: String,
    /// Maps language identifiers to their README and output definitions.
    pub(crate) languages: BTreeMap<String, Language>,
    /// Lists additional project-relative assets to copy.
    pub(crate) assets: Vec<String>,
    /// Defines an optional coverage report copy operation.
    pub(crate) coverage: Option<CopyConfig>,
    /// Defines an optional coverage badge copy operation.
    pub(crate) coverage_badge: Option<CopyConfig>,
    /// Defines an optional metadata copy operation.
    pub(crate) metadata: Option<CopyConfig>,
}

/// Loads the first available supported configuration file or the default
/// configuration.
///
/// The primary `.infra/ci/pages.json` file takes precedence over the legacy
/// `.rs-ci-page.json` file. When neither file exists, English and optional
/// Chinese README definitions are supplied as defaults.
///
/// # Errors
///
/// Returns an error when a discovered configuration file cannot be read or
/// contains invalid JSON.
///
/// # Parameters
///
/// * `project` - Project root containing the supported configuration files.
///
/// # Returns
///
/// Returns the parsed configuration, or the default bilingual configuration
/// when no language definitions are provided.
pub(crate) fn load(project: &Path) -> Result<Config> {
    let config_path = [project.join(".infra/ci/pages.json"), project.join(".rs-ci-page.json")]
        .into_iter()
        .find(|path| path.is_file());
    let mut config: Config = config_path
        .map(fs::read_to_string)
        .transpose()?
        .map(|text| serde_json::from_str(&text))
        .transpose()?
        .unwrap_or_default();
    if config.languages.is_empty() {
        config.languages.insert(
            "en".into(),
            Language {
                readme: "README.md".into(),
                output: "index.html".into(),
                label: "English".into(),
            },
        );
        config.languages.insert(
            "zh_CN".into(),
            Language {
                readme: "README.zh_CN.md".into(),
                output: "zh_CN/index.html".into(),
                label: "中文".into(),
            },
        );
    }
    if config.default_language.is_empty() {
        config.default_language = "en".into();
    }
    Ok(config)
}
