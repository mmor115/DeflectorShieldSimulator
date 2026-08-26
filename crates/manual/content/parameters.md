This chapter lists every control in the Parameters window, with its range, default,
symbol, and meaning.

## Drive

| Control | Options | Default | Symbol | Meaning |
|---|---|---|---|---|
| **Warp Drive** | **Ours**, **Natario** | **Ours** | `warp_drive_kind` | Which warp-drive metric the simulation evolves. |

**Ours** is the CCT warp drive of the paper, with an optional deflector-shield term
constructed in the same style as the drive. Everything else in this chapter describes
it. **Natario** is a zero-expansion drive with no deflector; see
[The Natario drive](natario-drive.html).

The control is disabled while the bubble is shut down.

> [!WARNING]
> Changing drives does not re-solve the ship state, so the ship keeps the velocity it had
> under the previous drive. See [Known limitations](known-limitations.html).

## Bubble

| Control | Range | Default | Symbol | Meaning |
|---|---|---|---|---|
| **Radius** | 1 to 4 | 4 | `radius` | "The radius of the inner shield." |
| **Sigma** | 0.1 to 4 | 4 | `sigma` | "The width of the transition between the inner and outer shield regions." |
| **Speed** | 0.0 to 0.9 | 0.5 | `u` | The bubble's speed. Raising it also raises **Drag** by the same amount. |
| **Drag** | 0.0 to 0.9 | 0.5 | `u0` | How far the ship's velocity lags behind the bubble. The ship's x-velocity is `u - u0`. |
| **Deflection Strength** | 0.0 to 0.9 | 0.1 | `k0` | The deflector's strength. Zero turns the added shield off. |
| **Sigma Pushout** | 0.0 to 4.0 | 0.8 | `deflector_sigma_pushout` | How far beyond **Radius** the deflecting shell sits, in multiples of **Sigma**. The shell is centered at $R + p\,\sigma$. `1.0` reproduces the fixed shell of the published metric; the default of `0.8` is deliberate, see [The deflector shield](deflector-shield.html). |
| **Sigma Factor** | 0.0 to 4.0 | 1.0 | `deflector_sigma_factor` | How thick the deflecting shell is, in multiples of **Sigma**. The shell reaches $q\,\sigma$ to either side of its center. |
| **Deflector Back** | 0.0 to 1.0 | 1.0 | `deflector_back` | `1.0` deflects all round the ship. `0.0` deflects ahead of the ship only and switches off behind it. Intermediate values interpolate. |

> [!NOTE]
> **Radius**, **Sigma**, **Speed** and **Drag** belong to the warp bubble. **Deflection
> Strength**, **Sigma Pushout**, **Sigma Factor** and **Deflector Back** belong to the
> optional deflector. The **Radius** and **Sigma** tooltips say "shield" because the UI
> draws those bubble surfaces as the inner and outer rings; the deflector is a separate
> shell, placed by **Sigma Pushout** and **Sigma Factor**.

> [!NOTE]
> **Drag**, **Deflection Strength**, **Sigma Pushout**, **Sigma Factor** and **Deflector
> Back** belong to the CCT drive only. Select **Natario** on the Drive tab and all five
> gray out, showing "Selected warp drive does not use this parameter." on hover. The
> defaults listed above are the values the CCT drive starts from.

> [!NOTE]
> The source marks these three fields `// TODO: Document`. Their behavior is derived
> here from the deflector function in `deflector-core` and from the published metric. The
> paper fixes the shell at $R + \sigma$ with a reach of $\sigma$, which corresponds to
> **Sigma Pushout** `1.0` and **Sigma Factor** `1.0`. See
> [The deflector shield](deflector-shield.html) for the equations.

See [The deflector shield](deflector-shield.html) for what each parameter does to the
metric, and [Tune the shield](tune-the-shield.html) for a task-oriented walkthrough.

> [!NOTE]
> Raising **Speed** shifts **Drag** by the same amount, to hold the ship's velocity
> steady. If the result would put `u0` outside 0.1 to 0.9, the slider reverts to its
> previous value and shows "Shield Drag out of range!"

> [!WARNING]
> Dragging **Drag** calls `update_u0`, which re-solves the ship state from the new
> value. If the solver rejects the result, the call panics with `update_u0 failed` and
> the program closes, with no warning first. This risk applies only outside shutdown;
> while the bubble is shut down, **Drag** stores the value directly and does not
> re-solve.

### Shut down and shut up

Clicking **Shut down** sets **Speed**, **Drag**, and **Deflection Strength** to zero, and
saves the other Bubble tab values for later. It does not change the ship's velocity, so
the ship keeps drifting at whatever speed it already had.

While the bubble is shut down, the Bubble tab sliders edit the saved values, not the
live, zeroed ones. Clicking **Shut up** restores the saved values and recomputes the
ship's velocity from them.

> [!WARNING]
> **Shut up** re-solves the ship state the same way **Drag** does outside shutdown, and
> can panic and close the program if the restored `u - u0` exceeds 1. Loading a
> configuration on the Save & Load tab carries the same risk, because it calls the same
> re-solve path. See [Known limitations](known-limitations.html).

## Particles

| Control | Range | Default | Symbol | Meaning |
|---|---|---|---|---|
| **Spawn Particles** | on/off | on | `spawning_enabled` | Enables or disables particle spawning. |
| **y-Position Spread** | 0.01 to 150, game units | 150 | `y_position_variance` | Half-width of the y range new particles spawn in. |
| **z-Position Spread** | 0.0 to 150, game units | 0 | `z_position_variance` | Half-width of the z range new particles spawn in. |
| **x-Velocity Spread** | 0.0 to 0.9, physics units | 0.0 | `x_velocity_variance` | Half-width of the x-velocity range new particles spawn with. |
| **y-Velocity Spread** | 0.0 to 0.9, physics units | 0.0 | `y_velocity_variance` | Half-width of the y-velocity range new particles spawn with. |
| **z-Velocity Spread** | 0.0 to 0.9, physics units | 0.0 | `z_velocity_variance` | Half-width of the z-velocity range new particles spawn with. |
| Normalized velocity spread readout | display only | — | — | Shows `vx² + vy² + vz²` for the three velocity spreads, floored to 6 decimal places. |
| **Photon Spawn Ratio** | 0.0 to 1.0 | 0.0 | `photon_chance` | Fraction of newly spawned particles that are photons rather than massive particles. |

> [!NOTE]
> The position spreads are in game units; the velocity spreads are in physics units, as
> commented in the source. See [Physics, units and time](units.html) for how the two
> relate.

> [!NOTE]
> The three velocity spread sliders are cross-clamped so that `vx² + vy² + vz² < 1`. If a
> change would break that bound, the slider that changed reverts to the largest value
> that still satisfies it, and shows "Normalized velocity is too great!"

See [Control the particle beam](particle-beam.html) for a task-oriented walkthrough of
this tab.

## Visuals

| Control | Range | Default | Symbol | Meaning |
|---|---|---|---|---|
| **Draw inner shield** | on/off | on | `show_inner_bubble` | Shows or hides the inner bubble mesh. |
| **Draw outer shield** | on/off | on | `show_outer_bubble` | Shows or hides the outer bubble mesh. |
| **Hide untagged particles** | on/off | off | `hide_untagged_particles` | Hides every particle without a tag. Tagged particles stay visible either way. |

> [!NOTE]
> Shutting down the bubble hides both rings regardless of these checkboxes, and
> restores them to the checkbox state once you shut up.

## Save & Load

| Control | Range | Default | Symbol | Meaning |
|---|---|---|---|---|
| **Config Path** | text field | empty, falls back to `sim.json` | `config_path_buf` | Path used by **Save Config** and **Load & Apply Config**. |
| **Save Config** | button | — | — | Writes the Bubble, Particles, Visuals, Shutdown, and Validation settings as pretty-printed JSON. |
| **Load & Apply Config** | button | — | — | Reads a config file and applies it, re-solving the ship state. |
| **History Path** | text field | empty, falls back to `dump.json` or `dump.bin` | `history_path_buf` | Path used by the dump and load buttons below. |
| **Binary** / **JSON** | radio buttons | Binary | `history_format` | Chooses the dump and load file format. "Fast. Produces small, indecipherable files." for Binary; "Slow. Produces large, human-readable files." for JSON. |
| **Dump only tagged particles** | on/off | off | `dump_only_tagged_particles` | Restricts the next dump to tagged particles only. |
| **Dump Single Checkpoint** | button | — | — | Writes only the current checkpoint. |
| **Dump Entire History** | button | — | — | Writes every checkpoint recorded so far. "This can be very slow and produce very large files!" |
| **Trim History** | button | — | — | Releases every checkpoint recorded before now. |
| **Load History** | button | — | — | Reads a history file into memory for resuming. |
| **Unload History** | button | — | — | Discards the history loaded into memory. |
| **Resume from Start** / **Resume from** / **Resume from End** | button, plus an index field for **Resume from** | index 0 | `resume_idx_buf` | Restarts the simulation from the given checkpoint in the loaded history. |

> [!NOTE]
> Resuming from an index outside the loaded history shows "Index out of range: 0 <= idx
> < len" instead of resuming.

> [!WARNING]
> **Load & Apply Config** re-solves the ship state from the loaded values, the same way
> **Shut up** does. A config with an out-of-range combination of parameters can panic
> and close the program on load. See [Known limitations](known-limitations.html).

See [Save and reuse a configuration](configurations.html) and [Run a reproducible
replay](replays.html) for task-oriented walkthroughs of this tab.

## Speed

| Control | Range | Default | Symbol | Meaning |
|---|---|---|---|---|
| **Pause** / **Unpause** | on/off | Pause shown (unpaused) | `paused` | Toggles the pause state; the button label reflects the current state. |
| **Reset Speed** | button | — | — | Sets **Simulation Speed** back to 1. |
| **Simulation Speed** | 0.1 to 6 | 1 | `tick_rate_factor` | Multiplies the base tick rate of 60 Hz. |

> [!NOTE]
> A tooltip reading "Can't keep up!" appears over **Simulation Speed** whenever a
> frame's render time exceeds the current tick interval, `1 / (tick_rate_factor * 60)`.

## Validation

| Control | Range | Default | Symbol | Meaning |
|---|---|---|---|---|
| **Check for NaN**: Ignore, Die, Remove Particle, Explode Particle | radio buttons | Die | `nan_validator` | What happens when a state vector contains NaN. |
| **Check for non-normalized state**: Ignore, Die, Remove Particle, Explode Particle | radio buttons | Explode Particle | `normalization_validator` | What happens when a state vector falls outside the normalization tolerance. |
| **Tolerance** | 1e-12 to 1e-1, in steps of one power of ten | 1e-6 | `normalized_tolerance` | Allowed deviation from a normalized state. |

See [Validate numerics](validation.html) for a task-oriented walkthrough of what each
setting does, and [Physics, units and time](units.html) for what a normalized state is.

> [!NOTE]
> Choosing Ignore for either check also skips the unconditional ship-state check built
> into the same validator system. Die, Remove Particle, and Explode Particle all still
> check the ship state and panic if it fails, whichever behavior is set for particles.
