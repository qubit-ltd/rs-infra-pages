// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Defines a localized README page configuration.

use serde::Deserialize;

/// Describes one localized README page.
#[derive(Debug, Deserialize)]
pub(crate) struct Language {
    /// Identifies the project-relative README source path.
    pub(crate) readme: String,
    /// Identifies the output path relative to the generated site.
    pub(crate) output: String,
    /// Provides the human-readable language label shown in navigation.
    pub(crate) label: String,
}
