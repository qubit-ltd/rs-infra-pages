//! Small Rust static site builder for project README and CI artifacts.

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Config {
    site_title: Option<String>,
    default_language: String,
    languages: std::collections::BTreeMap<String, Language>,
}

#[derive(Debug, Deserialize)]
struct Language {
    readme: String,
    output: String,
    label: String,
}

pub fn build(project: &Path, output: &Path) -> Result<()> {
    let config_path = project.join(".infra/ci/pages.json");
    let config: Config = if config_path.is_file() {
        serde_json::from_str(&fs::read_to_string(config_path)?)?
    } else {
        Config {
            site_title: None,
            default_language: "en".into(),
            languages: [
                (
                    "en".into(),
                    Language {
                        readme: "README.md".into(),
                        output: "index.html".into(),
                        label: "English".into(),
                    },
                ),
                (
                    "zh_CN".into(),
                    Language {
                        readme: "README.zh_CN.md".into(),
                        output: "zh_CN/index.html".into(),
                        label: "中文".into(),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        }
    };
    if config.languages.is_empty() {
        bail!("pages configuration must define at least one language");
    }
    let output = if output.is_absolute() {
        output.to_owned()
    } else {
        project.join(output)
    };
    fs::create_dir_all(&output)?;
    let mut links = String::new();
    for (language, definition) in &config.languages {
        let source = project.join(&definition.readme);
        if !source.is_file() {
            bail!(
                "README for language {language} was not found: {}",
                source.display()
            );
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
    let _ = &config.default_language;
    println!("Built site at {}", output.display());
    Ok(())
}

pub fn publish_github_pages(output: &Path) -> Result<()> {
    let artifact = output
        .canonicalize()
        .context("pages output directory was not found")?;
    let status = Command::new("tar")
        .args(["-czf", "pages-artifact.tar.gz", "-C"])
        .arg(&artifact)
        .arg(".")
        .status()?;
    if !status.success() {
        bail!("failed to package GitHub Pages artifact");
    }
    println!("Prepared GitHub Pages artifact from {}", artifact.display());
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
}
