This guide shows how to catch a NaN or a non-normalized state during a run, and how to choose what the simulator does when it finds one.

## What the validators check

The simulator runs two validators on every physics tick, on the **Validation** tab of the **Parameters** window: the NaN validator and the normalization validator. Each checks both the ship and every particle.

The NaN validator checks every component of a state vector — `x`, `y`, `z`, `vx`, `vy`, `vz`, `e` — for NaN.

The normalization validator checks whether a state is normalized, within a tolerance:

- For a massive particle, a normalized state satisfies `e = 1 / sqrt(1 - v²)`, where `v²` is the squared speed.
- For a photon, a normalized state satisfies `|v| = 1`.

See [Physics, units and time](units.html) for what a normalized state means physically.

## The four behaviors

Set a separate behavior for the NaN validator and the normalization validator, independently:

- **Ignore**. The simulator takes no action. A bad state stays in the simulation and keeps evolving under the physics update, which can produce more bad values on later ticks.
- **Die**. The simulator panics and exits, once it finds a bad state.
- **Remove Particle**. The simulator despawns the offending particle. The ship has no equivalent removal; see the warning below.
- **Explode Particle**. The simulator despawns the offending particle and spawns an explosion effect at its position.

## When the check runs, relative to the snapshot

This is the detail that matters for a dump you plan to inspect afterward.

**Remove Particle** and **Explode Particle** run *before* that tick's snapshot is taken. A particle removed this way never appears in the recorded history — the snapshot only sees the particles that survived.

**Die** runs *after* that tick's snapshot is taken. The bad state that triggered the panic is captured in the dump, in the last snapshot, right before the simulator exits.

This is deliberate. The source comment in `validators.rs` explains the reasoning. Killing the program preserves the failure for later inspection. Removing a particle keeps an ongoing run's history free of invalid states. Pick **Die** to capture a failure for analysis. Pick **Remove Particle** or **Explode Particle** to keep a long run clean and unattended.

> [!WARNING]
> A bad ship state always panics the program, whichever of **Die**, **Remove Particle**, or **Explode Particle** you selected. The ship has no despawn path. **Remove Particle** and **Explode Particle** only change what happens to particles. Set a validator to **Ignore** to skip its check entirely, for both the ship and particles.

## The Tolerance slider

The normalization validator compares against a tolerance, set by the **Tolerance** slider on the **Validation** tab. The slider holds an integer exponent from `-12` to `-1`, displayed as `1e%d`, and defaults to `-6` (a tolerance of `1e-6`). Moving the slider recomputes the tolerance as ten to that power.

To change the tolerance:

1. Open the **Parameters** window and select the **Validation** tab.
2. Drag the **Tolerance** slider to the exponent you want, from `-12` (tightest) to `-1` (loosest). The simulator applies the new tolerance immediately.

The NaN validator has no tolerance — a value either is NaN or it is not.

## Choosing a behavior

**For a long production run**, set both validators to **Remove Particle** or **Explode Particle**. The run continues unattended, and the recorded history never contains an invalid state. Avoid **Die** here, since the first bad state ends the run. Avoid **Ignore** too: a bad state left in place can propagate into further bad states, with no record of when it started.

**For debugging a specific blow-up**, set the relevant validator to **Die**. The snapshot taken just before the panic captures the exact bad values. Load the dump afterward to inspect the state that triggered the failure. See [Export trajectory data](exporting.html) and [Analyze dumps with warpsim.py](analysis.html).
