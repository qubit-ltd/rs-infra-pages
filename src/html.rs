// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Converts the supported README subset into standalone HTML.

/// Converts headings and paragraphs from the supported Markdown subset to HTML.
///
/// The `title` argument is retained for the renderer interface but is not
/// emitted by this conversion step. Text is escaped before the limited inline
/// markup is applied.
///
/// # Parameters
///
/// * `markdown` - Markdown text containing headings, paragraphs, and supported
///   inline markers.
/// * `title` - Renderer title retained for the interface but not emitted.
///
/// # Returns
///
/// Returns the converted HTML fragment without a document wrapper.
pub(crate) fn markdown_to_html(markdown: &str, title: &str) -> String {
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

/// Converts supported inline Markdown markers into HTML tags.
///
/// # Parameters
///
/// * `value` - Escaped inline text containing the supported marker characters.
///
/// # Returns
///
/// Returns HTML with supported strong and code markers replaced by tags.
fn inline(value: &str) -> String {
    html_text(value).replace("**", "<strong>").replace('`', "<code>")
}

/// Escapes text characters that have HTML significance.
///
/// # Parameters
///
/// * `value` - Text to escape.
///
/// # Returns
///
/// Returns text with ampersands and angle brackets replaced by HTML entities.
pub(crate) fn html_text(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Escapes text for safe use inside an HTML attribute.
///
/// # Parameters
///
/// * `value` - Attribute text to escape.
///
/// # Returns
///
/// Returns text escaped for HTML text and double-quoted attribute contexts.
pub(crate) fn html_attr(value: &str) -> String {
    html_text(value).replace('"', "&quot;")
}

/// Wraps rendered content in the standalone site page template.
///
/// # Parameters
///
/// * `title` - Document title inserted into the escaped `<title>` element.
/// * `body` - Rendered HTML fragment inserted into the document body.
/// * `links` - Already-rendered navigation links inserted into the document
///   nav.
///
/// # Returns
///
/// Returns a complete standalone HTML document.
pub(crate) fn page(title: &str, body: &str, links: &str) -> String {
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title><style>body{{max-width:960px;margin:2rem auto;padding:0 1rem;font:16px system-ui;line-height:1.6}}code{{background:#eef;padding:.1rem .3rem}}</style></head><body><nav>{}</nav>{}</body></html>\n",
        html_text(title),
        links,
        body
    )
}
