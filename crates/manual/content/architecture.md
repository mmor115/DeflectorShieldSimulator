This chapter explains how a frame is computed: the order the systems run in, what makes a
run reproducible, and which effects on screen are physics rather than display.

You do not need this chapter to use the simulator. You do need it to trust an exported
dump, or to modify the program.

## Three clocks

The program is built on Bevy, an entity-component-system engine. Work is split across
three rates, described in [Physics, units and time](units.html).

The outer schedule runs at a fixed 2000 Hz. Inside it a timer gates the physics at 60 Hz
multiplied by the **Simulation Speed** factor. Rendering and input run once per displayed
frame, independently of both.

Separating them means the trajectory does not depend on your frame rate. A slow machine
produces the same numbers as a fast one, and takes longer to do it.

## The update chain

Everything in the fixed schedule runs in one explicit chain, in this order:

1. **Pre-update physics** — advance the simulation clock and decide whether this tick
   integrates.
2. **Update ship** — step the ship along its geodesic.
3. **Update explosions, update bubbles, spawn particles** — these three do not depend on
   each other, so they run as a group.
4. **Update particles** — integrate every particle, and cull the ones outside the window.
5. **Pre-snapshot validators** — check for NaN and for lost normalization.
6. **Take snapshot** — record a checkpoint.
7. **Post-snapshot validators** — check again.
8. **Post-update physics** — apply a pending speed change, and set the keep-up warning.

The order is fixed deliberately rather than left to the scheduler. Two places depend on
it.

Steps 5 and 7 straddle step 6, and that is the whole design of the validator system. A
validator set to remove or explode a bad particle runs at step 5, so the bad state never
reaches the recording. A validator set to terminate runs at step 7, so the bad state is
captured in the dump and you can inspect it. See [Validate numerics](validation.html).

Step 2 precedes step 4, so every particle is integrated against a ship position from the
same tick.

## Parallel integration

Particles are integrated in parallel across threads. This is sound because the model has
no back-reaction: particles do not affect the metric and do not interact with each other,
so each trajectory depends only on the shared parameters and its own state.

The integrator itself is an adaptive fourth-order Runge-Kutta step from `deflector-core`,
taken over a fixed outer step of `0.1` in physics units.

## Determinism

One seeded generator drives everything random in the program: the position and velocity of
each new particle, whether it is a photon, and the UUID it carries.

The generator is `Xoshiro256++`. It is seeded from operating-system entropy at startup, so
two fresh runs differ. Its full internal state and its original seed are written into
every checkpoint.

That is what makes replay exact. Restoring a checkpoint re-inserts the generator state, so
the continuation draws the same numbers in the same order, spawns the same particles at
the same times, and gives them the same identities. Tags survive a replay for the same
reason: a tag is stored against a UUID, and the UUID is regenerated identically. See
[Run a reproducible replay](replays.html).

Nothing else in the update chain is a source of nondeterminism. Parallel integration does
not change the result, because no particle reads another particle's state.

## Recording

A checkpoint is taken every physics tick, unless the simulation is paused. Each one holds
the whole configuration, the simulation time, the generator state, the ship state, and
every particle state.

Recording is always on and cannot be switched off, and history is held in memory. A long
run therefore grows without bound. **Trim History** discards everything except the newest
checkpoint. See [Export trajectory data](exporting.html).

## What is display, not physics

Four things you see are presentation only.

| On screen | Reality |
|---|---|
| Particles vanish at the edges | A culling window in game units. No physics happens there. |
| Particle color | The $z$ coordinate, mapped across the range $-15$ to $15$ in physics units. |
| The two rings | Drawn at the bubble radius and one transition width beyond it. They mark the warp bubble, not the deflector shell. The field is smooth, not a hard edge. |
| Explosions | A one-second marker where a validator rejected a particle. Not a collision. |

There is no collision detection anywhere in the program. Nothing in the model can hit
anything else.
