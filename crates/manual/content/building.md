This chapter covers the toolchain, the system packages, and the commands that build the simulator and the manual from source.

## Toolchain

The workspace uses edition 2024, so the build needs a recent stable Rust. Install Rust with
[`rustup`](https://rustup.rs); it tracks stable releases and keeps the toolchain up to date.

The workspace has two crates:

| Crate | Path | What it is |
|---|---|---|
| `deflector-shield-simulator` | `crates/simulator` | The application |
| `deflector-manual` | `crates/manual` | The generator that builds this documentation |

## Build the app

1. Install the system packages listed below.
2. Run `cargo build --release -p deflector-shield-simulator` from the workspace root.
3. Find the binary at `target/release/deflector-shield-simulator`.

### Linux system packages

The build needs development headers for the windowing system, input devices, audio and
Wayland. On Debian and Ubuntu, install these with `apt`:

| Package | Purpose |
|---|---|
| `build-essential` | C compiler and linker |
| `pkg-config` | Locates system libraries during the build |
| `libx11-dev` | X11 client library |
| `libxcursor-dev` | X11 cursor themes |
| `libxi-dev` | X11 input extension |
| `libxrandr-dev` | X11 display configuration |
| `libxkbcommon-dev` | Keyboard handling |
| `libasound2-dev` | ALSA audio |
| `libudev-dev` | Device enumeration |
| `libwayland-dev` | Wayland client library |
| `libxcb-render0-dev` | XCB rendering extension |
| `libxcb-shape0-dev` | XCB shape extension |
| `libxcb-xfixes0-dev` | XCB fixes extension |

Other distributions package the same libraries under different names. Consult your
distribution's package search before reporting a missing-header build failure.

### The `wayland` feature

Run `cargo build --release --features wayland` to compile the Wayland backend into the
binary alongside X11. The two backends coexist in one binary; Bevy picks the correct one
at startup based on the session. Release builds published by this project enable the
`wayland` feature, so one binary serves both X11 and Wayland sessions.

### Runtime requirement: Vulkan

The simulator renders through Vulkan. At runtime, Linux needs a working Vulkan driver:
`mesa-vulkan-drivers` for open-source Intel and AMD graphics, `vulkan-radeon` for AMD, or
`vulkan-intel` for Intel. NVIDIA users need the proprietary NVIDIA driver, which includes
its own Vulkan support.

### The `deflector-core` dependency

`deflector-core` supplies the geodesic solver. Cargo fetches it as a git dependency over
HTTPS from `https://github.com/lucass-carneiro/DeflectorShields.git`, pinned to a fixed
revision. The repository is public, so the build needs no credentials.

> [!NOTE]
> The build script checks the first bytes of `assets/images/ship.png` and stops with a
> clear error if the file is a Git LFS pointer rather than image data. This guards against
> a checkout made without `git-lfs` installed; it does not affect a normal build.

## Build the manual

1. Run `cargo run -p deflector-manual -- --out dist --base-url ""` to render the site into
   `dist/`.
2. Serve `dist/` with any static file server, for example `python3 -m http.server -d dist`.
3. Open the address the server prints to preview the manual.

Run `cargo run -p deflector-manual -- --check` to validate every internal link and
cross-reference without writing any files. Use it before publishing a change to this
manual.

> [!NOTE]
> A debug build runs the physics noticeably slower than a release build. The workspace
> dev profile already raises dependency optimisation to level 3, so most of the slowdown
> comes from the simulator's own unoptimised code, not from its dependencies.
