//! Small Rust static site builder for project README and CI artifacts.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use flate2::Compression;
use flate2::write::GzEncoder;
use serde::Deserialize;
use tar::Builder;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Config {
    site_title: Option<String>,
    default_language: String,
    languages: std::collections::BTreeMap<String, Language>,
    assets: Vec<String>,
    coverage: Option<CopyConfig>,
    coverage_badge: Option<CopyConfig>,
    metadata: Option<CopyConfig>,
}

#[derive(Debug, Deserialize)]
struct Language {
    readme: String,
    output: String,
    label: String,
}

#[derive(Debug, Deserialize)]
struct CopyConfig {
    source: String,
    output: String,
}

pub fn build(project: &Path, output: &Path) -> Result<()> {
    let config = load_config(project)?;
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

fn load_config(project: &Path) -> Result<Config> {
    let config_path = [
        project.join(".infra/ci/pages.json"),
        project.join(".rs-ci-page.json"),
    ]
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

pub fn publish_github_pages(output: &Path) -> Result<()> {
    create_artifact(output, Path::new("pages-artifact.tar.gz")).map(|_| ())
}

pub fn deploy_github_pages(output: &Path, artifact: &Path) -> Result<()> {
    create_artifact(output, artifact)?;
    println!("GitHub Pages deploy artifact is ready for actions/deploy-pages");
    Ok(())
}

fn markdown_to_html(markdown: &str, title: &str) -> String {
    let mut html = String::new();
    for line in markdown.lines() {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", inline(heading)));
        } else if let Some(heading) = trimmed.strip_prefix("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", inline(heading)));
        } else if trimmed.is_empty() {
            continue;
        } else {
            html.push_str(&format!("<p>{}</p>\n", inline(trimmed)));
        }
    }
    let _ = title;
    html
}

fn inline(value: &str) -> String {
    html_text(value)
        .replace("**", "<strong>")
        .replace("`", "<code>")
}

fn html_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn html_attr(value: &str) -> String {
    html_text(value).replace('"', "&quot;")
}

fn page(title: &str, body: &str, links: &str) -> String {
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title><style>body{{max-width:960px;margin:2rem auto;padding:0 1rem;font:16px system-ui;line-height:1.6}}code{{background:#eef;padding:.1rem .3rem}}</style></head><body><nav>{}</nav>{}</body></html>\n",
        html_text(title),
        links,
        body
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_default_bilingual_site() {
        let project = tempfile::tempdir().unwrap();
        fs::write(project.path().join("README.md"), "# Hello\n\nRust project.").unwrap();
        fs::write(
            project.path().join("README.zh_CN.md"),
            "# 你好\n\nRust 项目。",
        )
        .unwrap();
        let output = project.path().join("public");
        build(project.path(), &output).unwrap();
        assert!(output.join("index.html").is_file());
        assert!(output.join("zh_CN/index.html").is_file());
    }

    #[test]
    fn builds_configured_assets_and_optional_reports() {
        let project = tempfile::tempdir().unwrap();
        fs::create_dir_all(project.path().join(".infra/ci")).unwrap();
        fs::write(project.path().join("README.md"), "# Hello").unwrap();
        fs::create_dir(project.path().join("assets")).unwrap();
        fs::write(project.path().join("assets/site.css"), "body {}").unwrap();
        fs::write(project.path().join("ci-summary.json"), "{}\n").unwrap();
        fs::write(
            project.path().join(".infra/ci/pages.json"),
            r#"{"languages":{"en":{"readme":"README.md","output":"index.html","label":"English"}},"assets":["assets"],"metadata":{"source":"ci-summary.json","output":"ci-summary.json"}}"#,
        )
        .unwrap();

        let output = project.path().join("public");
        build(project.path(), &output).unwrap();
        assert!(output.join("assets/site.css").is_file());
        assert_eq!(
            fs::read_to_string(output.join("ci-summary.json")).unwrap(),
            "{}\n"
        );
    }

    #[test]
    fn creates_github_pages_artifact_without_external_tools() {
        let project = tempfile::tempdir().unwrap();
        fs::write(project.path().join("index.html"), "hello").unwrap();
        let artifact = project.path().join("pages.tar.gz");
        create_artifact(project.path(), &artifact).unwrap();

        let file = fs::File::open(artifact).unwrap();
        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);
        let names: Vec<_> = archive
            .entries()
            .unwrap()
            .map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        assert!(names.iter().any(|name| name.ends_with("index.html")));
    }
}
