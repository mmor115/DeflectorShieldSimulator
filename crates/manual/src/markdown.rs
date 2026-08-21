// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

//! Markdown to HTML, with LaTeX rendered to MathML at build time.
//!
//! Math is handled by rewriting comrak's syntax tree rather than by patching its HTML
//! output. Working on the tree means a `$...$` inside a code span or a link title cannot
//! be mistaken for an equation.

use anyhow::{Context, Result};
use comrak::nodes::{AstNode, NodeValue};
use comrak::{Anchorizer, Arena, Options, format_html, parse_document};
use pulldown_latex::{RenderConfig, Storage, config::DisplayMode, push_mathml};

/// A heading, for the in-page table of contents.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Heading {
    pub id: String,
    pub text: String,
    /// 2 or 3. `h1` is the page title and is not listed.
    pub level: u8,
}

pub struct Rendered {
    pub html: String,
    pub headings: Vec<Heading>,
    /// Plain text of the body, for the search index.
    pub text: String,
}

fn options() -> Options<'static> {
    let mut options = Options::default();

    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    // GitHub-style `> [!NOTE]` blocks carry the manual's caveats.
    options.extension.alerts = true;
    // $inline$ and $$display$$, plus ```math fences.
    options.extension.math_dollars = true;
    options.extension.math_code = true;
    // An empty prefix still turns on heading ids and anchor links.
    options.extension.header_id_prefix = Some(String::new());

    // The manual authors its own HTML (<kbd>, <abbr>), and the sources are all in-repo.
    options.render.r#unsafe = true;

    options
}

/// Render one LaTeX fragment to MathML.
fn mathml(latex: &str, display: bool) -> Result<String> {
    let storage = Storage::new();
    let parser = pulldown_latex::Parser::new(latex, &storage);

    let config = RenderConfig {
        display_mode: if display {
            DisplayMode::Block
        } else {
            DisplayMode::Inline
        },
        // Keeps the original TeX in the document for copy-paste and accessibility.
        annotation: Some(latex),
        ..RenderConfig::default()
    };

    let mut out = String::new();

    push_mathml(&mut out, parser, config).with_context(|| format!("rendering math: {latex}"))?;

    Ok(out)
}

/// Replace every math node with pre-rendered MathML.
fn render_math<'a>(root: &'a AstNode<'a>) -> Result<()> {
    // Collect first: rewriting values while the traversal holds a borrow would panic.
    let nodes: Vec<&AstNode> = root.descendants().collect();

    for node in nodes {
        let replacement = {
            let data = node.data.borrow();

            match &data.value {
                NodeValue::Math(math) => Some((mathml(&math.literal, math.display_math)?, false)),
                // A ```math fence is a code block, not a math node.
                NodeValue::CodeBlock(block) if block.info.trim() == "math" => {
                    Some((mathml(&block.literal, true)?, true))
                }
                _ => None,
            }
        };

        if let Some((html, block)) = replacement {
            let mut data = node.data.borrow_mut();

            data.value = if block {
                NodeValue::HtmlBlock(comrak::nodes::NodeHtmlBlock {
                    block_type: 6,
                    literal: format!("<div class=\"math-block\">{html}</div>\n"),
                })
            } else {
                NodeValue::HtmlInline(html)
            };
        }
    }

    Ok(())
}

/// Concatenated text of a node's descendants, used for headings and the search index.
fn node_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut out = String::new();

    for descendant in node.descendants() {
        let data = descendant.data.borrow();

        match &data.value {
            NodeValue::Text(text) => out.push_str(text),
            NodeValue::Code(code) => out.push_str(&code.literal),
            NodeValue::SoftBreak | NodeValue::LineBreak => out.push(' '),
            _ => {}
        }
    }

    out
}

pub fn render(source: &str) -> Result<Rendered> {
    let options = options();
    let arena = Arena::new();
    let root = parse_document(&arena, source, &options);

    // Headings are collected before the math pass so an equation in a heading still
    // contributes its source text to the anchor.
    let mut anchorizer = Anchorizer::new();
    let mut headings = Vec::new();
    let mut text = String::new();

    for node in root.descendants() {
        let data = node.data.borrow();

        match &data.value {
            NodeValue::Heading(heading) => {
                let content = node_text(node);
                // Anchorize every heading so the dedup suffixes match comrak's own.
                let id = anchorizer.anchorize(&content);

                if heading.level == 2 || heading.level == 3 {
                    headings.push(Heading {
                        id,
                        text: content.clone(),
                        level: heading.level,
                    });
                }

                text.push_str(&content);
                text.push(' ');
            }
            NodeValue::Paragraph => {
                text.push_str(&node_text(node));
                text.push(' ');
            }
            _ => {}
        }
    }

    render_math(root)?;

    let mut html = String::new();
    format_html(root, &options, &mut html).context("formatting HTML")?;

    Ok(Rendered {
        html,
        headings,
        text: text.split_whitespace().collect::<Vec<_>>().join(" "),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_inline_and_display_math_to_mathml() {
        let out = render("Speed $v$ and\n\n$$ds^2 = -dt^2$$\n").unwrap();

        assert!(
            out.html.contains("<math"),
            "no MathML emitted: {}",
            out.html
        );
        assert!(
            out.html.contains("display=\"block\""),
            "no block math: {}",
            out.html
        );
    }

    #[test]
    fn collects_headings_with_matching_ids() {
        let out = render("# Title\n\n## First section\n\n### Detail\n").unwrap();

        assert_eq!(out.headings.len(), 2);
        assert_eq!(out.headings[0].id, "first-section");
        assert_eq!(out.headings[0].level, 2);
        assert_eq!(out.headings[1].level, 3);
        // The rendered HTML must carry the same id the TOC links to.
        assert!(out.html.contains("id=\"first-section\""));
    }

    #[test]
    fn dollar_inside_code_is_not_math() {
        let out = render("Run `echo $HOME` now.\n").unwrap();

        assert!(
            !out.html.contains("<math"),
            "code span treated as math: {}",
            out.html
        );
    }

    #[test]
    fn tables_and_alerts_render() {
        let out = render("| a | b |\n|---|---|\n| 1 | 2 |\n\n> [!NOTE]\n> Careful.\n").unwrap();

        assert!(out.html.contains("<table"));
        assert!(out.html.to_lowercase().contains("note"));
    }
}
