use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "rs-infra-pages")]
struct Cli {
    #[arg(long, default_value = ".")]
    project: PathBuf,
    #[arg(long, default_value = "public")]
    output: PathBuf,
    #[arg(long, default_value = "pages-artifact.tar.gz")]
    artifact: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Build,
    Artifact,
    Deploy,
    PublishGithubPages,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let project = std::fs::canonicalize(cli.project)?;
    match cli.command {
        Command::Build => qubit_infra_pages::build(&project, &cli.output),
        Command::Artifact => qubit_infra_pages::create_artifact(
            &cli.output,
            &resolve_from_project(&project, &cli.artifact),
        )
        .map(|_| ()),
        Command::Deploy => qubit_infra_pages::deploy_github_pages(
            &cli.output,
            &resolve_from_project(&project, &cli.artifact),
        ),
        Command::PublishGithubPages => qubit_infra_pages::publish_github_pages(&cli.output),
    }
}

fn resolve_from_project(project: &std::path::Path, path: &std::path::Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        project.join(path)
    }
}
