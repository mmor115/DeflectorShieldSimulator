// SPDX-License-Identifier: AGPL-3.0-or-later
//
// DeflectorShieldSimulator - particle geodesics in a warp-drive deflector-shield spacetime
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt
//
// This program is free software: you can redistribute it and/or modify it under the
// terms of the GNU Affero General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
// PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

//! Generates the DeflectorShieldSimulator manual and landing page as static HTML.
//!
//!     manual --content <dir> --theme <dir> --out <dir> --base-url /Repo
//!     manual --check
//!
//! `--check` builds the site in memory and validates it without writing anything.

mod check;
mod config;
mod markdown;
mod site;

use anyhow::{Result, bail};
use config::ManualConfig;
use std::collections::BTreeSet;
use std::path::PathBuf;

struct Args {
    content: PathBuf,
    theme: PathBuf,
    out: PathBuf,
    /// Path prefix the site is served under, e.g. `/DeflectorShieldSimulator`.
    /// Empty when served from a domain root.
    base: String,
    check_only: bool,
}

const USAGE: &str = "\
usage: manual [options]

  --content <dir>   Markdown and TOML sources   (default: crates/manual/content)
  --theme <dir>     CSS, JS and fonts           (default: crates/manual/theme)
  --out <dir>       Output directory            (default: dist)
  --base-url <path> Path prefix the site is served under, e.g. /DeflectorShieldSimulator
  --check           Validate without writing any files
  -h, --help        Show this message
";

fn parse_args() -> Result<Args> {
    // The crate root, so the defaults work from anywhere in the workspace.
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut args = Args {
        content: crate_dir.join("content"),
        theme: crate_dir.join("theme"),
        out: PathBuf::from("dist"),
        base: String::new(),
        check_only: false,
    };

    let mut argv = std::env::args().skip(1);

    while let Some(arg) = argv.next() {
        let mut value = || {
            argv.next()
                .ok_or_else(|| anyhow::anyhow!("{arg} needs a value"))
        };

        match arg.as_str() {
            "--content" => args.content = PathBuf::from(value()?),
            "--theme" => args.theme = PathBuf::from(value()?),
            "--out" => args.out = PathBuf::from(value()?),
            "--base-url" => args.base = value()?,
            "--check" => args.check_only = true,
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            other => bail!("unrecognized argument {other:?}\n\n{USAGE}"),
        }
    }

    // Normalize to "" or "/prefix" with no trailing slash, so templates can always
    // write {{ base }}/path.
    args.base = args.base.trim_end_matches('/').to_string();

    if !args.base.is_empty() && !args.base.starts_with('/') {
        args.base = format!("/{}", args.base);
    }

    if !args.content.is_dir() {
        bail!(
            "content directory {} does not exist",
            args.content.display()
        );
    }

    Ok(args)
}

fn main() -> Result<()> {
    let args = parse_args()?;

    let site = site::build(&args.content, &args.base)?;

    let listed: BTreeSet<String> = ManualConfig::load(&args.content.join("manual.toml"))?
        .chapter
        .iter()
        .map(|c| c.file.clone())
        .collect();

    // Write first, then validate, so a failing check still leaves output to inspect.
    if !args.check_only {
        site::write(&site, &args.theme, &args.out)?;
    }

    let verdict = check::run(&site, &args.content, &args.base, &listed);

    if args.check_only {
        return verdict;
    }

    println!(
        "wrote {} pages to {}{}",
        site.pages.len(),
        args.out.display(),
        if args.base.is_empty() {
            String::new()
        } else {
            format!(" (served under {})", args.base)
        }
    );

    verdict
}
