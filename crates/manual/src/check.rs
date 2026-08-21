// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

//! Consistency checks that run before anything is published.
//!
//! The point of generating the manual with our own crate rather than a general-purpose
//! tool is that the build can fail on documentation that has drifted from the code.

use crate::site::Site;
use anyhow::{Result, bail};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;

/// Every `href` and every `id` found in the generated HTML.
struct Anchors {
    /// page path (as served) -> ids defined on that page
    ids: HashMap<String, HashSet<String>>,
    /// (source page, raw href)
    links: Vec<(String, String)>,
}

/// Templating escapes attribute values, so `/` arrives as `&#x2f;`. Undo the entities the
/// escaper produces before an href is interpreted as a path.
fn unescape(value: &str) -> String {
    value
        .replace("&#x2f;", "/")
        .replace("&#x27;", "'")
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        // Ampersand last, so an escaped entity is not decoded twice.
        .replace("&amp;", "&")
}

/// Deliberately small: the generator controls the markup, so a full HTML parser would be
/// more machinery than the job needs.
fn scan(site: &Site) -> Anchors {
    let mut ids: HashMap<String, HashSet<String>> = HashMap::new();
    let mut links = Vec::new();

    for page in &site.pages {
        let served = page.path.to_string_lossy().replace('\\', "/");
        let entry = ids.entry(served.clone()).or_default();

        for chunk in page.html.split(" id=\"").skip(1) {
            if let Some(end) = chunk.find('"') {
                entry.insert(chunk[..end].to_string());
            }
        }

        for chunk in page.html.split(" href=\"").skip(1) {
            if let Some(end) = chunk.find('"') {
                links.push((served.clone(), unescape(&chunk[..end])));
            }
        }
    }

    Anchors { ids, links }
}

/// Turns an href into the page path it addresses, relative to the output root.
///
/// `from` is the page the link appears on, so a document-relative href such as
/// `units.html` resolves against that page's directory rather than against the root.
fn resolve(base: &str, from: &str, href: &str) -> Option<(String, Option<String>)> {
    if href.starts_with("http://") || href.starts_with("https://") || href.starts_with("mailto:") {
        return None;
    }

    let (path, fragment) = match href.split_once('#') {
        Some((p, f)) => (p, Some(f.to_string())),
        None => (href, None),
    };

    if path.is_empty() {
        // A bare "#id" points at the current page.
        return Some((from.to_string(), fragment));
    }

    let resolved = if let Some(rooted) = path.strip_prefix('/') {
        // Root-relative. Under a project Pages site every such link must carry the base
        // path, or it breaks once deployed.
        if base.is_empty() {
            rooted.to_string()
        } else {
            let trimmed = base.trim_start_matches('/');

            match rooted.strip_prefix(trimmed) {
                Some(rest) => rest.trim_start_matches('/').to_string(),
                None => return Some((format!("!missing-base-prefix!{path}"), fragment)),
            }
        }
    } else {
        // Document-relative: join to the directory holding `from`.
        let dir = match from.rsplit_once('/') {
            Some((d, _)) => d,
            None => "",
        };

        let mut parts: Vec<&str> = if dir.is_empty() {
            Vec::new()
        } else {
            dir.split('/').collect()
        };

        for segment in path.split('/') {
            match segment {
                "" | "." => {}
                ".." => {
                    parts.pop();
                }
                other => parts.push(other),
            }
        }

        parts.join("/")
    };

    // A directory URL is served by its index.html.
    let normalized = if resolved.is_empty() || resolved.ends_with('/') {
        format!("{resolved}index.html")
    } else {
        resolved
    };

    Some((normalized, fragment))
}

/// Every content file must be listed in manual.toml, or it silently never ships.
fn check_orphans(content_dir: &Path, listed: &BTreeSet<String>) -> Result<Vec<String>> {
    let mut problems = Vec::new();

    // Fragments composed into the landing page rather than listed as chapters.
    let known_extras: HashSet<&str> = ["home-intro.md", "home-quickstart.md"].into();

    for entry in std::fs::read_dir(content_dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();

        if !name.ends_with(".md") {
            continue;
        }

        if !listed.contains(&name) && !known_extras.contains(name.as_str()) {
            problems.push(format!(
                "content/{name} exists but is not listed in manual.toml, so it is never published"
            ));
        }
    }

    Ok(problems)
}

pub fn run(site: &Site, content_dir: &Path, base: &str, listed: &BTreeSet<String>) -> Result<()> {
    let anchors = scan(site);
    let mut problems = check_orphans(content_dir, listed)?;

    let known_assets: HashSet<&str> = [
        "assets/site.css",
        "assets/math/styles.css",
        "assets/manual.js",
        "assets/home.js",
        "assets/favicon.svg",
    ]
    .into();

    for (from, href) in &anchors.links {
        let Some((path, fragment)) = resolve(base, from, href) else {
            continue;
        };

        if let Some(rest) = path.strip_prefix("!missing-base-prefix!") {
            problems.push(format!(
                "{from}: link {rest:?} is root-relative but omits the base path {base:?}"
            ));
            continue;
        }

        let target = path;

        if known_assets.contains(target.as_str()) {
            continue;
        }

        let Some(ids) = anchors.ids.get(&target) else {
            problems.push(format!(
                "{from}: link {href:?} points at a page that is not generated"
            ));
            continue;
        };

        if let Some(fragment) = fragment
            && !fragment.is_empty()
            && !ids.contains(&fragment)
        {
            problems.push(format!(
                "{from}: link {href:?} has no matching id on {target}"
            ));
        }
    }

    if problems.is_empty() {
        println!("check: {} pages, no problems", site.pages.len());
        return Ok(());
    }

    problems.sort();
    problems.dedup();

    for problem in &problems {
        eprintln!("  {problem}");
    }

    bail!("{} problem(s) found", problems.len());
}
