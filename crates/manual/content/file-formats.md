This chapter lists every field of the configuration file and the history dump, and explains the difference between the JSON and bincode dump formats.

## The configuration file

The simulator reads and writes a single JSON file for its settings, from the **Save & Load** tab of the **Parameters** window. The default path is `sim.json`. The file holds one `GlobalConfig` object with five sections: `physics_config`, `particle_settings`, `visual_settings`, `shutdown_config`, and `validator_settings`.

### `physics_config`

| Field | Type | Meaning |
|---|---|---|
| `radius` | number | The shield radius. |
| `sigma` | number | The width of the shield's transition region. |
| `u` | number | The shield speed. |
| `u0` | number | The dragging speed the shield applies to particles it carries along. |
| `k0` | number | The deflection strength. |
| `x0` | number | The shield position recorded at time `t0`. The simulator updates this itself whenever the shield changes speed; treat it as internal state, not a value to hand-tune. |
| `t0` | number | The simulation time at which `x0` was recorded. |
| `gamma` | number | The time dilation strength. Every shipped configuration sets this to `0.0`; the source marks the field for eventual removal. |
| `epsilon` | number | A small number substituted for zero to avoid division by zero in the metric. Shipped configurations use `1e-12`. |
| `deflector_back` | number | `1.0` deflects all round the ship; `0.0` deflects ahead of it only. Defaults to `1.0` if omitted. |
| `deflector_sigma_factor` | number | Thickness of the deflecting shell, in multiples of `sigma`. Defaults to `1.0` if omitted. |
| `deflector_sigma_pushout` | number | Offset of the deflecting shell beyond `radius`, in multiples of `sigma`. Defaults to `0.8` if omitted. |
| `warp_drive_kind` | string | Which metric to evolve: `"Ours"` or `"Natario"`. Defaults to `"Ours"` if omitted. |

> [!NOTE]
> `deflector_back`, `deflector_sigma_factor`, `deflector_sigma_pushout` and `warp_drive_kind` were added after the first configuration files were written. Omit any of them and the simulator substitutes the default shown above, not `0.0`. A config file saved before these fields existed still loads correctly.

> [!NOTE]
> `warp_drive_kind` is spelled exactly as shown, with a leading capital. `"ours"` is not accepted and fails to load.

A file with `"warp_drive_kind": "Natario"` still carries `u0`, `k0`, `gamma` and the three `deflector_*` fields, because every configuration has the same shape. The Natario drive ignores them, and writes them out as `0.0`. See [The Natario drive](natario-drive.html).

### `particle_settings`

| Field | Type | Meaning |
|---|---|---|
| `spawning_enabled` | boolean | Whether the simulator spawns new particles. Defaults to `true` if omitted. |
| `y_position_variance` | number (game units) | Half-width of the range of y positions a new particle can spawn at. The simulator draws each spawn position uniformly from `-y_position_variance` to `y_position_variance`. |
| `z_position_variance` | number (game units) | The same, for the z position. |
| `x_velocity_variance` | number (physics units) | Half-width of the range of x velocity a new particle can spawn with. |
| `y_velocity_variance` | number (physics units) | The same, for the y velocity. |
| `z_velocity_variance` | number (physics units) | The same, for the z velocity. |
| `photon_chance` | number, `0.0`–`1.0` | The probability that a newly spawned particle is a photon rather than a massive particle. |

See [Control the particle beam](particle-beam.html) for how these variances combine, and [Physics, units and time](units.html) for game units versus physics units.

### `visual_settings`

| Field | Type | Meaning |
|---|---|---|
| `show_inner_bubble` | boolean | Whether the simulator draws the inner shield surface. |
| `show_outer_bubble` | boolean | Whether the simulator draws the outer shield surface. |
| `hide_untagged_particles` | boolean | Whether the simulator hides every particle that is not tagged. Defaults to `false` if omitted. |

### `shutdown_config`

| Field | Type | Meaning |
|---|---|---|
| `in_shutdown_state` | boolean | While `true`, the simulator hides the shield and stops spawning particles. |
| `temporary_parameters` | object or `null` | A `physics_config` object used only while `in_shutdown_state` is `true`, or `null` when not in use. |

### `validator_settings`

| Field | Type | Meaning |
|---|---|---|
| `nan_validator` | string | The behavior for a NaN in a state vector. One of `"Ignore"`, `"Die"`, `"RemoveParticle"`, `"ExplodeParticle"`. |
| `normalization_validator` | string | The behavior for a state vector that is not normalized within tolerance. Same four values. |
| `normalized_tolerance` | number | The tolerance the normalization validator checks against, for example `1e-6`. |
| `normalized_tolerance_power` | integer, `-12`–`-1` | The exponent that produced `normalized_tolerance`. The simulator recomputes `normalized_tolerance` as `10^normalized_tolerance_power` whenever the **Tolerance** slider moves, so edit this field to change the tolerance by hand and keep the two consistent. |

The whole `validator_settings` section may be omitted. If it is missing, the simulator applies its defaults: `nan_validator: "Die"`, `normalization_validator: "ExplodeParticle"`, `normalized_tolerance: 1e-6`, `normalized_tolerance_power: -6`. See [Validate numerics](validation.html) for what each behavior does.

### A complete example

This is `configs/pos_slippage.json`, unmodified:

```json
{
  "physics_config": {
    "radius": 4.0,
    "sigma": 4.0,
    "u": 0.5,
    "u0": 0.404855,
    "k0": 0.1,
    "x0": 0.0,
    "t0": 0.0,
    "gamma": 0.0,
    "epsilon": 1e-12,
    "deflector_back": 1.0,
    "deflector_sigma_factor": 1.0,
    "deflector_sigma_pushout": 0.8
  },
  "particle_settings": {
    "spawning_enabled": true,
    "y_position_variance": 150.0,
    "z_position_variance": 0.0,
    "x_velocity_variance": 0.014563,
    "y_velocity_variance": 0.014563,
    "z_velocity_variance": 0.0,
    "photon_chance": 0.0
  },
  "visual_settings": {
    "show_inner_bubble": true,
    "show_outer_bubble": true,
    "hide_untagged_particles": false
  },
  "shutdown_config": {
    "in_shutdown_state": false,
    "temporary_parameters": null
  },
  "validator_settings": {
    "nan_validator": "Die",
    "normalization_validator": "ExplodeParticle",
    "normalized_tolerance": 1e-6,
    "normalized_tolerance_power": -6
  }
}
```

Some values worth noting:

- `x_velocity_variance` and `y_velocity_variance` are both `0.014563`, and `z_velocity_variance` is `0.0`. Particles spawn with a small, equal spread of velocity in x and y, and none in z.
- `z_position_variance` is `0.0`, so every particle spawns at the same z position, and `y_position_variance` is `150.0`, so particles spawn across a wide range of y.
- `in_shutdown_state` is `false` and `temporary_parameters` is `null`: the shield is not in a shutdown transition.

## The history dump

A history dump is a serialised `SimulationHistory`:

```
SimulationHistory
├── snapshots: [GlobalSnapshot, ...]
└── tagged_particles: { tagged_particles: [<uuid>, ...] }
```

Each `GlobalSnapshot` holds the complete state of one simulation tick:

```
GlobalSnapshot
├── global_config: GlobalConfig        (the sections described above)
├── global_time: number
├── seeded_rng: SeededRng
├── ship_state: { physics: [x, y, z, vx, vy, vz, e] }
└── particle_states: [SpaceDustStateSnapshot, ...]
```

Each entry in `particle_states` is a `SpaceDustStateSnapshot`:

| Field | Type | Meaning |
|---|---|---|
| `physics` | array of 7 numbers | The particle's state vector: `[x, y, z, vx, vy, vz, e]`. |
| `id` | string | The particle's UUID. |
| `particle_type` | string | `"Massive"` or `"Photon"`. |

`ship_state.physics` uses the same 7-element layout. The simulator always treats the ship as a massive body; there is no `particle_type` field for it.

> [!NOTE]
> `particle_type` defaults to `"Massive"` if it is missing from a particle entry, so a dump written before this field existed still loads.

> [!NOTE]
> `e` is the seventh component of the state vector. For a massive particle it is the Lorentz factor; for a photon it carries no independent value beyond what normalization requires. See [Physics, units and time](units.html) for the full definition of a normalized state.

`seeded_rng` holds two things: the internal state of the xoshiro256++ generator, and the original 32-byte seed the generator started from. Both are required for a reproducible replay. The seed alone reproduces only the start of a run. Resuming from a later checkpoint needs the generator's exact state at that tick. See [Run a reproducible replay](replays.html).

`tagged_particles` lists every particle ID tagged at dump time, independent of which snapshot you resume from. See [Tag particles](tagging.html).

## JSON vs bincode

The **Save & Load** tab writes a history dump in either format, and reads either back with **Load History**.

| | JSON | Bincode |
|---|---|---|
| File size | Large | Small |
| Write and read speed | Slow | Fast |
| Human-readable | Yes | No |
| Self-describing | Yes — unknown or missing fields can fall back to serde defaults | No — fields are packed positionally, with no field names and no type tags |

Because a bincode file carries no field names, it is tied to the exact structure of the program version that wrote it. The `#[serde(default)]` fallbacks that let an older JSON file load into a newer build do not help a bincode file. A bincode dump written by one version of the simulator can fail to decode, or decode into the wrong fields, when a different version reads it.

Use JSON to inspect a dump by eye, to post-process it with a script, or to move it between simulator versions. Use bincode for a long run, where file size and write speed matter more than portability.

> [!WARNING]
> Every path in the **Save & Load** tab — the config path, and the history path — is resolved relative to the process's working directory, not the location of the simulator executable. Launching the simulator from a different directory changes where `sim.json` and dump files are read from and written to.
