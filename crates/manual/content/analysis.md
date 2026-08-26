`scripts/warpsim.py` reads a JSON history dump into Python objects, and builds a JSON input file from particles you specify yourself.

## Requirements

The script needs Python 3.9 or newer; an earlier interpreter rejects its `list[Snapshot]` type annotation. It imports nothing beyond the standard library: `json`, `os`, `enum`, `math`, and `typing`.

It reads only the JSON format described in [exporting.html](exporting.html). It has no decoder for the Binary format, so dump with **JSON** selected if you plan to load the file with this script.

## The `WarpSim` class

Construct one with `WarpSim()`. With no arguments, it starts from a single built-in default checkpoint with no particles, ready for `add_particle` calls. Call `load()` to replace that checkpoint data with a real dump instead.

### Properties

Each property reads and writes a field of the first checkpoint's config, or of the ship's state.

| Property | Underlying field |
|---|---|
| `radius` | `physics_config.radius` |
| `sigma` | `physics_config.sigma` |
| `warp_drive_kind` | `physics_config.warp_drive_kind` |
| `deflector` | `physics_config.k0` |
| `deflector_sigma_pushout` | `physics_config.deflector_sigma_pushout` |
| `deflector_sigma_factor` | `physics_config.deflector_sigma_factor` |
| `deflector_back` | `physics_config.deflector_back` |
| `u_bubble` | `physics_config.u` |
| `u_drag` | `physics_config.u0` |
| `u_ship` | `ship_state.physics.vx` |

Setting `u_ship` also renormalizes the ship's state, recomputing its energy component from its velocity.

`warp_drive_kind` takes the string `"Ours"` or `"Natario"`, spelled exactly as the simulator writes it. See [The Natario drive](natario-drive.html).

### Methods

`load(fname="dump.json")` reads a JSON file and replaces the object's checkpoints with the ones it contains.

`add_particle(pos, vel, ty, trackme=False)` appends a particle to the first checkpoint. `pos` and `vel` are each a 3-tuple; `ty` is `MASSIVE` or `PHOTON`. Each call assigns the next id in a plain counter, not a UUID, since these particles have no simulator-assigned id yet. Pass `trackme=True` to also add the new id to `tagged_particles`, which tags the particle in the simulator without clicking it (see [tagging.html](tagging.html)).

`generate(fname="idump.json")` writes the current checkpoints to a JSON file, after checking the constraint described below.

## The normalization `add_particle` applies

After adding a particle, `add_particle` normalizes its velocity for its type:

- A **photon** gets its velocity rescaled to unit length. A zero velocity input becomes `(-1, 0, 0)` instead of a divide-by-zero.
- A **massive** particle keeps its velocity as given, but gets its energy component set to `1 / sqrt(1 - v²)`. `add_particle` asserts `v² < 1` first, and raises if you pass a superluminal velocity.

## The `u_bubble` constraint

`generate()` asserts `u_bubble == u_ship + u_drag` before writing the file. The simulator enforces the same constraint on the **Speed** and **Drag** sliders (see [tune-the-shield.html](tune-the-shield.html)). The bubble's asymptotic speed must equal the ship's own speed plus the bubble's drag on it. Set all three consistently before calling `generate()`, or the assertion fails.

> [!NOTE]
> Assigning a name the class does not define creates a plain Python attribute instead of raising an error, and the value never reaches the configuration. Check spelling against the property list above when a setting appears to have no effect.

## The `VISUALIZER_HOME_DIR` environment variable

Running the file directly, with `python warpsim.py`, executes a demo under `if __name__ == "__main__":`. That demo reads `VISUALIZER_HOME_DIR` from the environment and writes its generated file to `$VISUALIZER_HOME_DIR/idump.json`. Set that variable before running the script this way, or it raises `KeyError`.

## `scripts/warpgen.py`

`warpgen.py` is a worked example of building an input file rather than reading one. It
configures a Natario run, places a line of three tagged particles ahead of the ship and
a triad of three more on the bubble surface at 120 degree spacing, and writes the result.

```sh
python3 warpgen.py -o idump.json
```

The output path also reads from the `OUTPUT` environment variable, and defaults to
`idump.json`. Load the file with **Load History** and then **Resume from Start** to run
it; see [Run a reproducible replay](replays.html).

Copy it and edit the particle placement to set up a case of your own. It imports only
`warpsim`, `math`, `os`, `sys` and `argparse`.

## `scripts/warpplot.py`

`warpplot.py` plots trajectories from a dump with matplotlib. It needs `matplotlib`, `numpy` and `pandas`, and on Linux it renders its labels with LaTeX, so a working TeX installation is required there as well.

> [!WARNING]
> `warpplot.py` imports a module named `danger` for its directory paths. That module is not part of this repository, so the script does not run from a fresh checkout. Use `warpsim.py`, which has no third-party imports, or supply your own `danger` module defining `visualizer_home` and `simulator_home`. See [Known limitations](known-limitations.html).

## Worked example: read one particle's trajectory

This assumes a dump written with **Dump Entire History** and **Dump only tagged particles** both enabled (see [exporting.html](exporting.html) and [tagging.html](tagging.html)). Each checkpoint then holds only the tagged particle.

```python
from warpsim import WarpSim

sim = WarpSim()
sim.load("dump.json")

target_id = sim.tagged_particles[0]

trajectory = []
for snapshot in sim.snapshots:
    for particle in snapshot.particle_states:
        if particle.pid == target_id:
            p = particle.physics
            trajectory.append((snapshot.global_time, p.x, p.y, p.z))

print(trajectory)
```

Each entry in `trajectory` is that particle's simulated time and position at one checkpoint. A gap in the checkpoint indices means the particle left the simulated region and was despawned at that point (see [exporting.html](exporting.html)).
