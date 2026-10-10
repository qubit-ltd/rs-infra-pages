// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Provides the command-line interface for building and publishing project
//! pages.

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

/// Parses command-line arguments, executes the selected operation, and reports
/// its outcome to the user.
///
/// Exits with status code 1 when the selected operation fails.
fn main() {
    let cli = Cli::parse();
    let operation = command_name(&cli.command);
    match execute(cli) {
        Ok(message) => println!("✅ rs-infra-pages: SUCCESS: {message}"),
        Err(error) => {
            eprintln!("❌ rs-infra-pages: FAILURE ({operation}): {error:#}");
            std::process::exit(1);
        }
    }
}

/// Executes one parsed command and returns its final user-facing summary.
///
/// # Parameters
///
/// * `cli` - Parsed command-line options and the operation to execute.
///
/// # Returns
///
/// Returns a summary of the completed operation.
///
/// # Errors
///
/// Returns an error if the project directory cannot be canonicalized or the
/// selected operation fails.
fn execute(cli: Cli) -> Result<String> {
    let project = std::fs::canonicalize(cli.project)?;
    match cli.command {
        Command::Build => {
            build(&project, &cli.output)?;
            let output = resolve_from_project(&project, &cli.output);
            Ok(format!("built site at {}", output.display()))
        }
        Command::Artifact => {
            let output = resolve_from_project(&project, &cli.output);
            let artifact = resolve_from_project(&project, &cli.artifact);
            create_artifact(&output, &artifact)?;
            Ok(format!("created artifact at {}", artifact.display()))
        }
        Command::Deploy => {
            let output = resolve_from_project(&project, &cli.output);
            let artifact = resolve_from_project(&project, &cli.artifact);
            deploy_github_pages(&output, &artifact)?;
            Ok(format!("prepared deploy artifact at {}", artifact.display()))
        }
        Command::PublishGithubPages => {
            let output = resolve_from_project(&project, &cli.output);
            publish_github_pages(&output)?;
            Ok("published pages-artifact.tar.gz".to_owned())
        }
    }
}

/// Returns the stable command name used in failure summaries.
///
/// # Parameters
///
/// * `command` - Selected command whose display name is needed.
///
/// # Returns
///
/// Returns the command's stable kebab-case name.
fn command_name(command: &Command) -> &'static str {
    match command {
        Command::Build => "build",
        Command::Artifact => "artifact",
        Command::Deploy => "deploy",
        Command::PublishGithubPages => "publish-github-pages",
    }
}

/// Resolves a relative path from the project directory and preserves absolute
/// paths.
///
/// # Parameters
///
/// * `project` - Base directory for relative paths.
/// * `path` - Path to resolve.
///
/// # Returns
///
/// Returns an owned absolute path, preserving `path` unchanged when it is
/// already absolute.
///
/// # Errors
///
/// This function does not perform I/O and cannot fail.
fn resolve_from_project(project: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        project.join(path)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use clap::Parser;
    use tempfile::tempdir;

    use super::Cli;
    use super::execute;

    #[test]
    fn execute_builds_and_archives_using_project_relative_paths() {
        let project = tempdir().expect("temporary project directory should be created");
        fs::write(project.path().join("README.md"), "# Pages").expect("README should be written");
        let project_path = project.path().to_str().expect("project path should be UTF-8");

        let build = Cli::try_parse_from(["rs-infra-pages", "--project", project_path, "build"])
            .expect("build command should parse");
        assert!(execute(build).expect("site should build").starts_with("built site at "));

        let artifact = Cli::try_parse_from([
            "rs-infra-pages",
            "--project",
            project_path,
            "--output",
            "public",
            "--artifact",
            "artifacts/site.tar.gz",
            "artifact",
        ])
        .expect("artifact command should parse");
        assert!(
            execute(artifact)
                .expect("artifact should be created")
                .starts_with("created artifact at ")
        );
        assert!(project.path().join("artifacts/site.tar.gz").is_file());

        let deploy = Cli::try_parse_from([
            "rs-infra-pages",
            "--project",
            project_path,
            "--output",
            "public",
            "--artifact",
            "artifacts/deploy.tar.gz",
            "deploy",
        ])
        .expect("deploy command should parse");
        assert!(
            execute(deploy)
                .expect("deploy artifact should be created")
                .starts_with("prepared deploy artifact at ")
        );
        assert!(project.path().join("artifacts/deploy.tar.gz").is_file());
    }

    #[test]
    fn execute_returns_an_error_for_a_missing_project() {
        let project = tempdir().expect("temporary project directory should be created");
        let missing = project.path().join("missing");
        let cli = Cli::try_parse_from([
            "rs-infra-pages",
            "--project",
            missing.to_str().expect("project path should be UTF-8"),
            "build",
        ])
        .expect("build command should parse");

        assert!(execute(cli).is_err());
    }
}
