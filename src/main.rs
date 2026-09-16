// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Provides the command-line interface for building and publishing project pages.

use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use qubit_infra_pages::build;
use qubit_infra_pages::create_artifact;
use qubit_infra_pages::deploy_github_pages;
use qubit_infra_pages::publish_github_pages;

/// Defines the command-line options for the pages tool.
#[derive(Debug, Parser)]
#[command(name = "rs-infra-pages")]
struct Cli {
    /// Selects the project directory to process.
    #[arg(long, default_value = ".")]
    project: PathBuf,
    /// Selects the directory where generated pages are written.
    #[arg(long, default_value = "public")]
    output: PathBuf,
    /// Selects the artifact path used by artifact and deploy commands.
    #[arg(long, default_value = "pages-artifact.tar.gz")]
    artifact: PathBuf,
    /// Selects the operation to perform.
    #[command(subcommand)]
    command: Command,
}

/// Lists the operations supported by the command-line interface.
#[derive(Debug, Subcommand)]
enum Command {
    /// Renders configured README files into the output directory.
    Build,
    /// Creates a compressed pages artifact.
    Artifact,
    /// Creates a compressed artifact for deployment by GitHub Pages Actions.
    Deploy,
    /// Creates the default GitHub Pages artifact in the project directory.
    PublishGithubPages,
}

/// Parses command-line arguments and executes the selected operation.
///
/// # Errors
///
/// Returns an error when the project path cannot be canonicalized or the selected
/// operation cannot complete.
///
/// # Returns
///
/// Returns `()` after the selected command completes successfully.
fn main() {
    let cli = Cli::parse();
    let operation = command_name(&cli.command);
    match execute(cli) {
        Ok(message) => println!("rs-infra-pages: SUCCESS: {message}"),
        Err(error) => {
            eprintln!("rs-infra-pages: FAILURE ({operation}): {error:#}");
            std::process::exit(1);
        }
    }
}

/// Executes one parsed command and returns its final user-facing summary.
fn execute(cli: Cli) -> Result<String> {
    let project = std::fs::canonicalize(cli.project)?;
    match cli.command {
        Command::Build => {
            build(&project, &cli.output)?;
            let output = resolve_from_project(&project, &cli.output);
            Ok(format!("built site at {}", output.display()))
        }
        Command::Artifact => {
            let artifact = resolve_from_project(&project, &cli.artifact);
            create_artifact(&cli.output, &artifact)?;
            Ok(format!("created artifact at {}", artifact.display()))
        }
        Command::Deploy => {
            let artifact = resolve_from_project(&project, &cli.artifact);
            deploy_github_pages(&cli.output, &artifact)?;
            Ok(format!(
                "prepared deploy artifact at {}",
                artifact.display()
            ))
        }
        Command::PublishGithubPages => {
            publish_github_pages(&cli.output)?;
            Ok("published pages-artifact.tar.gz".to_owned())
        }
    }
}

/// Returns the stable name used in failure summaries.
fn command_name(command: &Command) -> &'static str {
    match command {
        Command::Build => "build",
        Command::Artifact => "artifact",
        Command::Deploy => "deploy",
        Command::PublishGithubPages => "publish-github-pages",
    }
}

/// Resolves a relative path from the project directory and preserves absolute paths.
///
/// # Parameters
///
/// * `project` - Base directory for relative paths.
/// * `path` - Path to resolve.
///
/// # Returns
///
/// Returns an owned absolute path, preserving `path` unchanged when it is already
/// absolute.
fn resolve_from_project(project: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        project.join(path)
    }
}
