// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Defines an optional source-to-output copy configuration.

use serde::Deserialize;

/// Describes one optional source-to-output copy operation.
#[derive(Debug, Deserialize)]
pub(crate) struct CopyConfig {
    /// Identifies the project-relative source path.
    pub(crate) source: String,
    /// Identifies the output-relative destination path.
    pub(crate) output: String,
}
