This chapter covers the **Particles** tab: how to aim the incoming debris, how fast it
arrives, and how to mix photons into it.

## Turn spawning on and off

**Spawn Particles** is on by default. Switch it off to stop new particles appearing.
Existing particles keep moving, so this is how you clear the field to watch the last few
trajectories finish.

Spawning is also suppressed while the shield is shut down.

## Aim the stream

Two sliders set where particles appear across the beam. Both are in **game units**, which
are ten times the physics units. See [Physics, units and time](units.html).

| Slider | Range | Default | Effect |
|---|---|---|---|
| **y-Position Spread** | `0.01` – `150.0` | `150.0` | Particles appear at a $y$ drawn uniformly from $-v$ to $+v$ |
| **z-Position Spread** | `0.0` – `150.0` | `0.0` | The same for $z$ |

At the default the beam fills the screen, which is right for seeing the shape of the
shield. Reduce **y-Position Spread** to around `20` to aim a narrow stream at the ship,
which is right for following one trajectory.

**z-Position Spread** moves particles out of the plane you are looking at. It changes the
physics, but on screen it only changes the color and the draw order, so a particle with a
large $z$ can look closer to the ship than it is. Leave it at `0` unless you specifically
want out-of-plane trajectories, and read the exported data rather than the picture when
you do.

Particles appear 275 game units ahead of the ship and are removed once they fall 275 game
units behind it, or 160 game units to either side. Those bounds keep the program
responsive and have no physical meaning.

## Set the incoming velocity

Three sliders give each particle a random initial velocity. These are in **physics
units**, so a value of `0.01` is one per cent of the speed of light.

| Slider | Range | Default |
|---|---|---|
| **x-Velocity Spread** | `0.0` – `0.9` | `0.0` |
| **y-Velocity Spread** | `0.0` – `0.9` | `0.0` |
| **z-Velocity Spread** | `0.0` – `0.9` | `0.0` |

Each component is drawn uniformly from $-v$ to $+v$. At the default of zero, particles
start at rest in the global frame and everything you see is the geometry moving them.

The **Normalized velocity spread** readout below the sliders shows the combined
magnitude. Watch it rather than the individual sliders when you care about the total.

> [!WARNING]
> A massive particle cannot reach the speed of light, so the three sliders are clamped
> together to keep $v_x^2 + v_y^2 + v_z^2 < 1$. Raise one too far and it snaps back to the
> largest value the other two allow, showing `Normalized velocity is too great!` in red.
> This is a constraint of the physics, not a limitation of the interface. See
> [Physics, units and time](units.html).

The paper's debris fields use velocities of about one per cent of $c$, so set the spreads
to `0.01` to match.

## Mix in photons

**Photon Spawn Ratio** runs from `0.0` to `1.0`, default `0.0`. It is the probability that
each new particle is a photon rather than a massive particle.

Photons obey a different normalization: their speed is fixed at $c$, so the velocity
spread sliders set only their direction. The simulator renormalises them on creation.

Photons are drawn blue to cyan; massive particles are drawn red to yellow. In both cases
the shade encodes the $z$ coordinate. Set the ratio to `0.5` to compare the two side by
side through the same shield.

## What comes out

Every particle carries a UUID drawn from the seeded generator, which is why a replay
regenerates the same particles with the same identities. See
[Run a reproducible replay](replays.html).

To follow a specific particle rather than a random one, tag it. See
[Tag particles](tagging.html). To build a debris field with particles at positions you
choose rather than random ones, use the Python helper. See
[Analyze dumps with warpsim.py](analysis.html).
