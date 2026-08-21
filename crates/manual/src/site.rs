// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

//! Assembles the rendered chapters and the landing page into a static tree.

use crate::config::{HomeConfig, ManualConfig};
use crate::markdown;
use anyhow::{Context, Result, bail};
use minijinja::value::Value;
use minijinja::{Environment, context};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Serialize, Clone)]
pub struct ChapterRef {
    pub slug: String,
    pub title: String,
    pub summary: String,
}

#[derive(Serialize)]
struct PartRef {
    name: String,
    chapters: Vec<ChapterRef>,
}

/// One built page, kept in memory so `--check` can inspect the whole site before
/// anything touches the disk.
pub struct Page {
    /// Path relative to the output root, e.g. `manual/replays.html`.
    pub path: PathBuf,
    pub html: String,
}

#[derive(Serialize)]
pub struct SearchEntry {
    slug: String,
    title: String,
    part: String,
    headings: Vec<markdown::Heading>,
    text: String,
}

pub struct Site {
    pub pages: Vec<Page>,
    pub search_index: String,
}

fn environment() -> Result<Environment<'static>> {
    let mut env = Environment::new();

    // Templates are compiled in so the generator does not depend on its own source tree
    // being present at run time.
    env.add_template("base.html", include_str!("../templates/base.html"))?;
    env.add_template("chapter.html", include_str!("../templates/chapter.html"))?;
    env.add_template(
        "manual_index.html",
        include_str!("../templates/manual_index.html"),
    )?;
    env.add_template("home.html", include_str!("../templates/home.html"))?;

    Ok(env)
}

/// Marks already-rendered HTML as safe, so the template engine inserts it verbatim
/// instead of escaping it into visible markup. Only ever applied to comrak output.
fn html(rendered: String) -> Value {
    Value::from_safe_string(rendered)
}

/// Reads one content file and renders its Markdown.
fn read_content(content_dir: &Path, name: &str) -> Result<markdown::Rendered> {
    let path = content_dir.join(name);

    let source =
        std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;

    markdown::render(&source).with_context(|| format!("rendering {}", path.display()))
}

pub fn build(content_dir: &Path, base: &str) -> Result<Site> {
    let manual = ManualConfig::load(&content_dir.join("manual.toml"))?;
    let home = HomeConfig::load(&content_dir.join("home.toml"))?;
    let env = environment()?;

    // Anything a template needs on every page. `base` is a path we control, and it is
    // concatenated with literal paths in the templates, so escaping its slashes would
    // only make the emitted URLs harder to read.
    let common = context! {
        base => Value::from_safe_string(base.to_string()),
        site_title => manual.title.clone(),
        subtitle => manual.subtitle.clone(),
        version => manual.version.clone(),
        license => manual.license.clone(),
        repository => manual.repository.clone(),
        paper_url => manual.paper_url.clone(),
        paper_citation => manual.paper_citation.clone(),
        body_class => "",
    };

    let parts: Vec<PartRef> = manual
        .parts()
        .into_iter()
        .map(|(name, chapters)| PartRef {
            name,
            chapters: chapters
                .into_iter()
                .map(|c| ChapterRef {
                    slug: c.slug().to_string(),
                    title: c.title.clone(),
                    summary: c.summary.clone(),
                })
                .collect(),
        })
        .collect();

    let mut pages = Vec::new();
    let mut search = Vec::new();

    // ---- chapters ----
    let chapter_tmpl = env.get_template("chapter.html")?;

    for (i, entry) in manual.chapter.iter().enumerate() {
        let rendered = read_content(content_dir, &entry.file)?;

        let prev = i
            .checked_sub(1)
            .and_then(|j| manual.chapter.get(j))
            .map(|c| ChapterRef {
                slug: c.slug().to_string(),
                title: c.title.clone(),
                summary: String::new(),
            });

        let next = manual.chapter.get(i + 1).map(|c| ChapterRef {
            slug: c.slug().to_string(),
            title: c.title.clone(),
            summary: String::new(),
        });

        let description = if entry.summary.is_empty() {
            manual.subtitle.clone()
        } else {
            entry.summary.clone()
        };

        let html = chapter_tmpl.render(context! {
            ..common.clone(),
            ..context! {
                page_title => format!("{} — {} manual", entry.title, manual.title),
                page_description => description,
                title => entry.title.clone(),
                summary => entry.summary.clone(),
                part => entry.part.clone(),
                slug => entry.slug().to_string(),
                content => html(rendered.html),
                headings => rendered.headings.clone(),
                parts => &parts,
                prev => prev,
                next => next,
            }
        })?;

        pages.push(Page {
            path: PathBuf::from("manual").join(format!("{}.html", entry.slug())),
            html,
        });

        search.push(SearchEntry {
            slug: entry.slug().to_string(),
            title: entry.title.clone(),
            part: entry.part.clone(),
            headings: rendered.headings,
            text: rendered.text,
        });
    }

    // ---- manual index ----
    let index_html = env.get_template("manual_index.html")?.render(context! {
        ..common.clone(),
        ..context! {
            page_title => format!("{} manual", manual.title),
            page_description => manual.subtitle.clone(),
            parts => &parts,
        }
    })?;

    pages.push(Page {
        path: PathBuf::from("manual").join("index.html"),
        html: index_html,
    });

    // ---- landing page ----
    let intro = read_content(content_dir, "home-intro.md")?;
    let quickstart = read_content(content_dir, "home-quickstart.md")?;

    let home_html = env.get_template("home.html")?.render(context! {
        ..common.clone(),
        ..context! {
            page_title => format!("{} — {}", manual.title, manual.subtitle),
            page_description => home.tagline.clone(),
            body_class => "home-page",
            tagline => home.tagline,
            intro => html(intro.html),
            quickstart => html(quickstart.html),
            downloads => home.download,
            links => home.link,
            release_base_url => home.release_base_url,
            bibtex => home.bibtex,
        }
    })?;

    pages.push(Page {
        path: PathBuf::from("index.html"),
        html: home_html,
    });

    let search_index = serde_json::to_string(&search).context("serialising the search index")?;

    Ok(Site {
        pages,
        search_index,
    })
}

/// Copies the theme directory into `out/assets`, preserving structure.
fn copy_theme(theme_dir: &Path, out: &Path) -> Result<()> {
    let assets = out.join("assets");

    for entry in walkdir::WalkDir::new(theme_dir) {
        let entry = entry.context("walking the theme directory")?;

        if !entry.file_type().is_file() {
            continue;
        }

        let relative = entry
            .path()
            .strip_prefix(theme_dir)
            .expect("walkdir yields paths under its root");

        let target = assets.join(relative);

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }

        std::fs::copy(entry.path(), &target)
            .with_context(|| format!("copying {}", target.display()))?;
    }

    Ok(())
}

pub fn write(site: &Site, theme_dir: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        if !out.join("index.html").exists() && std::fs::read_dir(out)?.next().is_some() {
            // Refuse to clear a directory that does not look like our own output.
            bail!(
                "{} is not empty and does not look like a generated site; \
                 remove it or choose another --out",
                out.display()
            );
        }

        std::fs::remove_dir_all(out).with_context(|| format!("clearing {}", out.display()))?;
    }

    for page in &site.pages {
        let target = out.join(&page.path);

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }

        std::fs::write(&target, &page.html)
            .with_context(|| format!("writing {}", target.display()))?;
    }

    std::fs::write(
        out.join("manual").join("search-index.json"),
        &site.search_index,
    )
    .context("writing the search index")?;

    copy_theme(theme_dir, out)?;

    // GitHub Pages otherwise runs the output through Jekyll, which drops files
    // whose names begin with an underscore.
    std::fs::write(out.join(".nojekyll"), "").context("writing .nojekyll")?;

    Ok(())
}
