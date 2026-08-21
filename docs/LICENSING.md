# Licensing and packaging decisions

This file records decisions that are expensive to re-derive, so they do not get
re-litigated later.

## The licence

The project is `AGPL-3.0-or-later`. Every source file carries an
`SPDX-License-Identifier` header; the two program entry points carry the full notice. The
**About** tab in the running program shows the notice, which is what AGPL section 5(d)
asks of an interactive program.

The AGPL network clause is inert for a desktop program: the simulator has no network
interaction, so in practice the terms behave like GPL-3.0. It becomes live if anyone ships
a **WASM or hosted web build**, which is plausible for a Bevy project. A hosted version
must offer its Corresponding Source to the people using it over the network.

## Dependency audit

All 550 entries in `Cargo.lock` were audited. Every third-party crate is permissive and
GPL-compatible: predominantly `MIT OR Apache-2.0`, with a tail of `BSD-2-Clause`,
`BSD-3-Clause`, `ISC`, `Zlib`, `0BSD`, `MIT-0`, `Unlicense`, `CC0-1.0`, `BSL-1.0`,
`Unicode-3.0` and `Unicode-DFS-2016`. There is no MPL, no GPL-2.0-only, and nothing
source-available.

`deflector-core` is AGPL-3.0, matching this project.

Twenty-two crates are `Apache-2.0` only, including `winit`, `nalgebra`, `simba` and
`cpal`. Apache-2.0 into AGPL-3.0 is fine, because GPLv3 resolved the patent-clause
incompatibility. It is one-directional, so this project can never be relicensed to
GPL-2.0-only. That is not a practical constraint, but it is worth knowing before anyone
proposes it.

`deny.toml` pins the audit as an allow-list, and CI runs `cargo-deny`. A `cargo update`
that pulls in an incompatible licence fails the pull request rather than quietly creating
a conflict. Adding an entry to that list is a deliberate decision, not a formality.

`cargo deny check advisories` currently reports pre-existing RUSTSEC findings in the Bevy
dependency tree. They are unrelated to licensing and are left for triage.

## Assets

The ship sprite is CC-BY 4.0 by arin48, which requires attribution but imposes no
share-alike or non-commercial condition. It is compiled into the binary, so the
attribution travels in `crates/simulator/assets/ATTRIBUTION.md`, in
`THIRD-PARTY-NOTICES.md`, in every release artifact, and in the About tab.

The documentation site vendors the Latin Modern math fonts under the GUST Font License.

## Why not musl

The Linux release is an AppImage plus a tarball, built against glibc 2.35 inside an
`ubuntu:22.04` container. A statically linked musl binary was considered and rejected.

A fully static musl binary cannot `dlopen`. `wgpu` must `dlopen` `libvulkan.so.1` or
`libGL.so.1`, which the host GPU driver supplies and which are built against glibc. This
is [bevyengine/bevy#898](https://github.com/bevyengine/bevy/issues/898), still open.
`bevy_mod_imgui` additionally compiles a C++ translation unit that needs libstdc++. The
realistic outcome is a link failure or a segfault at startup.

Flatpak solves the driver problem properly, but `flatpak-builder` builds without network
access, so every Cargo dependency would have to be vendored into a generated sources
manifest. That is more machinery than a research tool needs.

The release job runs in a **container** rather than on the `ubuntu-22.04` runner label,
because that label began deprecation in September 2026. The container pins the glibc floor
independently of the runner image lifecycle.

## Why the fmt gate is narrow

CI runs `cargo fmt` on `deflector-manual` only. The simulator crate is hand-formatted on
purpose, and rustfmt would rewrite 31 of its 34 files. Match the surrounding style when
editing it. Clippy and the test suite run across the whole workspace.
