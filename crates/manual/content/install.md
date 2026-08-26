This chapter gets a working program on your machine and past the first-launch prompts.

## Download

Take the file for your system from the
[project page](https://max-morris.github.io/DeflectorShieldSimulator/#download).

| System | File |
|---|---|
| Linux | `DeflectorShieldSimulator-x86_64.AppImage` |
| Linux, plain binary | `DeflectorShieldSimulator-linux-x86_64.tar.gz` |
| Windows | `DeflectorShieldSimulator-windows-x86_64.zip` |
| macOS | `DeflectorShieldSimulator-macos-universal.dmg` |

> [!NOTE]
> The following systems have been tested and are known to work: Debian 13 and Windows 11.
> Please open an issue on GitHub if you encounter trouble running the simulator on your system.

## Linux

1. Make the file executable:

   ```sh
   chmod +x DeflectorShieldSimulator-x86_64.AppImage
   ```

2. Run it:

   ```sh
   ./DeflectorShieldSimulator-x86_64.AppImage
   ```

The AppImage carries the C++ runtime and the X11, Wayland, ALSA and udev libraries. Your
system supplies the C library and the graphics driver.

The build targets glibc 2.35, so it runs on Ubuntu 22.04, Debian 12, Fedora 36 and
anything newer. On an older distribution, build from source instead. See
[Build from source](building.html).

Both display backends are compiled in, so the same file works in an X11 session and in a
Wayland session.

### You need a Vulkan driver

The renderer needs Vulkan. Install the driver for your hardware:

| Hardware | Debian and Ubuntu | Fedora | Arch |
|---|---|---|---|
| Intel | `mesa-vulkan-drivers` | `mesa-vulkan-drivers` | `vulkan-intel` |
| AMD | `mesa-vulkan-drivers` | `mesa-vulkan-drivers` | `vulkan-radeon` |
| NVIDIA | the proprietary driver | the proprietary driver | `nvidia-utils` |

Check that a device is visible before reporting a problem:

```sh
vulkaninfo --summary
```

## Windows

1. Extract the zip anywhere.
2. Run `DeflectorShieldSimulator.exe`.

Windows 10 or later, 64-bit. The C runtime is linked statically, so there is no Visual C++
Redistributable to install.

SmartScreen may warn that the publisher is unknown, because the executable is not signed.
Choose **More info**, then **Run anyway**.

## macOS

macOS 11 or later. The disk image holds a universal build that runs natively on Apple
silicon and on Intel.

1. Open the `.dmg`.
2. Drag the application to **Applications**.
3. **Right-click the application and choose Open**, then confirm **Open** in the dialog.

> [!WARNING]
> Step 3 matters. The build is signed ad hoc rather than by a registered Apple developer,
> so a normal double-click gives an error saying the application cannot be opened or is
> damaged. Opening it from the right-click menu once records your consent, and every
> later launch works normally.

If the right-click route still fails, clear the quarantine flag from a terminal. This
command removes the attribute macOS attaches to downloaded files, for this application
only:

```sh
xattr -dr com.apple.quarantine /Applications/DeflectorShieldSimulator.app
```

Signing and notarising a build requires a paid Apple Developer account, which this project
does not have. The source is public, and you can build it yourself if you would rather not
run an unsigned binary. See [Build from source](building.html).

## First launch

You should see a dark window with a small ship at the center, two blue rings around it,
and colored particles entering from the right. The **Parameters** window sits at the top
right.

Continue with [Your first simulation](first-simulation.html).

## When it does not start

| Symptom | Cause and fix |
|---|---|
| Black window, no particles | The renderer found no Vulkan device. Install the driver above, then run `vulkaninfo --summary`. |
| `failed to find a suitable adapter` | The same cause. In a virtual machine, enable 3D acceleration or install a software Vulkan driver such as `mesa-vulkan-drivers` with `lavapipe`. |
| Nothing happens on Wayland | Start it under XWayland to isolate the problem: `env WAYLAND_DISPLAY= ./DeflectorShieldSimulator-x86_64.AppImage`. |
| Window opens, then exits at once | Run it from a terminal so you can read the panic message, then see [Known limitations](known-limitations.html). |
| The ship is missing but particles move | You are running a build made from a Git LFS checkout without the real sprite. Use an official release. |

To see more detail, raise the log level:

```sh
RUST_LOG=info ./DeflectorShieldSimulator-x86_64.AppImage
```
