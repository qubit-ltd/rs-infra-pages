// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Build static project pages and prepare GitHub Pages artifacts.

mod artifact;
mod config;
mod html;
mod site;

pub use artifact::create_artifact;
pub use artifact::deploy_github_pages;
pub use artifact::publish_github_pages;
pub use site::build;
