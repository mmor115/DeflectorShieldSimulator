// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

use std::path::Path;

/* The sprite is embedded into the binary at build time. It used to be stored in Git LFS,
   where a checkout without git-lfs yields a small text pointer instead of the image. That
   failure is silent at build time and only shows up as a missing ship in the running app,
   so check the magic bytes here instead. */
fn main() {
    let asset = Path::new("assets/images/ship.png");

    println!("cargo:rerun-if-changed={}", asset.display());

    let bytes = std::fs::read(asset)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", asset.display()));

    const PNG_MAGIC: &[u8] = b"\x89PNG\r\n\x1a\n";

    if bytes.starts_with(b"version https://git-lfs") {
        panic!(
            "{} is a Git LFS pointer, not an image. This file is no longer tracked by LFS; \
             fetch the real bytes with `git lfs pull` or check out a current revision.",
            asset.display()
        );
    }

    if !bytes.starts_with(PNG_MAGIC) {
        panic!(
            "{} is not a PNG (first bytes: {:02x?})",
            asset.display(),
            &bytes[..bytes.len().min(8)]
        );
    }
}
