// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

//! The declarative description of the site: which chapters exist, in what order, and
//! what goes on the landing page.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// `content/manual.toml`. Chapter order is the order of the `[[chapter]]` entries, so
/// nothing depends on file names and there are no numeric prefixes to renumber.
#[derive(Debug, Deserialize)]
pub struct ManualConfig {
    pub title: String,
    pub subtitle: String,
    pub version: String,
    pub repository: String,
    pub paper_url: String,
    pub paper_citation: String,
    pub license: String,
    pub chapter: Vec<ChapterEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterEntry {
    /// File name inside `content/`, e.g. `what-this-is.md`.
    pub file: String,
    pub title: String,
    /// Groups chapters in the sidebar. Consecutive chapters sharing a part are one group.
    pub part: String,
    /// One sentence, used for the meta description and the chapter list.
    #[serde(default)]
    pub summary: String,
}

impl ChapterEntry {
    /// URL slug, derived from the file name so the two can never disagree.
    pub fn slug(&self) -> &str {
        self.file.strip_suffix(".md").unwrap_or(&self.file)
    }
}

impl ManualConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;

        let config: ManualConfig =
            toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;

        if config.chapter.is_empty() {
            bail!("{} lists no chapters", path.display());
        }

        for entry in &config.chapter {
            if !entry.file.ends_with(".md") {
                bail!("chapter file {:?} must end in .md", entry.file);
            }
        }

        Ok(config)
    }

    /// Chapters grouped into parts, preserving declaration order.
    pub fn parts(&self) -> Vec<(String, Vec<&ChapterEntry>)> {
        let mut parts: Vec<(String, Vec<&ChapterEntry>)> = Vec::new();

        for entry in &self.chapter {
            match parts.last_mut() {
                Some((name, chapters)) if *name == entry.part => chapters.push(entry),
                _ => parts.push((entry.part.clone(), vec![entry])),
            }
        }

        parts
    }
}

/// `content/home.toml`. Everything on the landing page that is not prose.
#[derive(Debug, Deserialize)]
pub struct HomeConfig {
    pub tagline: String,
    pub release_base_url: String,
    pub download: Vec<Download>,
    pub link: Vec<Link>,
    pub bibtex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Download {
    /// Display name of the platform, e.g. `Linux`.
    pub platform: String,
    /// Release asset file name. Must be version-free so the
    /// `releases/latest/download/` URL stays valid for every release.
    pub file: String,
    pub label: String,
    /// The honest caveat for this platform: driver requirements, Gatekeeper, and so on.
    #[serde(default)]
    pub note: String,
    /// Substring matched against the browser's platform string to highlight one row.
    /// Purely a convenience; the page is complete without it.
    #[serde(default)]
    pub detect: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    pub label: String,
    pub url: String,
    #[serde(default)]
    pub note: String,
}

impl HomeConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;

        toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }
}
