This chapter lists rough edges in the current release, and the workaround for each.

## Re-solving can terminate the program

Dragging the **Drag** slider feeds the new value straight into the geodesic solver and
re-solves the ship state. If the core rejects the resulting parameters, the program
panics and closes.

> [!WARNING]
> This can happen at any point while you drag the slider, without a warning first. Move
> it in small steps, and save your configuration before you experiment with large
> changes.

Loading a configuration file goes through the same re-solve path. A configuration with an
out-of-range combination of parameters can panic and close the program on load, for the
same reason.

## Changing drives leaves the ship state alone

Selecting a different **Warp Drive** on the Drive tab swaps the metric but does not
re-solve the ship state. The ship keeps the velocity it held under the previous drive,
which is wrong for the new one: the Natario drive expects a ship carried by the shift
vector, and the CCT drive expects one moving at `u - u0`.

Reload a configuration, or restart the simulator, after switching drives. Choosing the
drive in the configuration file rather than in the UI avoids the problem entirely; see
[File formats](file-formats.html).

## `warpplot.py` does not run from a fresh checkout

`scripts/warpplot.py` imports a module named `danger` for its directory paths, and that
module is not part of this repository. The script fails at import.

`scripts/warpsim.py` and `scripts/warpgen.py` have no such import and work as shipped.
See [Analyse dumps with warpsim.py](analysis.html).

## A bad ship state always terminates the program

A validator set to Ignore skips the checks it controls, but it does not intercept a bad
ship state. A bad ship state still terminates the program, regardless of the validator
setting. See [Validate numerics](validation.html) for what each validator setting does
control.

## History grows without bound

The history keeps every checkpoint recorded since the simulation started, or since the
last **Trim History**. Memory use grows for as long as the simulation runs. Click **Trim
History** periodically during a long run to release old checkpoints.

## Binary dumps are not self-describing

A `dump.bin` file carries no format version and no field names; it is a fixed binary
layout tied to the build that wrote it. A dump written by one build may fail to load, or
load with wrong values, in another build. Use the JSON export for any data you need to
keep or share beyond the session that produced it.

## No gamepad support, no key rebinding

The simulator reads keyboard and mouse input only, and the key bindings are fixed. There
is no way to reassign a control to a different key.

## Paths resolve against the working directory

A configuration path or a dump path that is not absolute resolves against the directory
the process was launched from, not against the location of the executable. Launching the
simulator from a different directory changes where a relative path points.

## The simulation is a 2D projection

The simulator draws the scene in two dimensions. The z coordinate of a particle affects
its colour and its draw order, but the display does not scale it the way it scales x and
y. See [Physics, units and time](units.html) for how the state vector uses all three
coordinates internally.
