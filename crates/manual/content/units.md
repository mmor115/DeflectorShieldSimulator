This chapter fixes the conventions the rest of the manual relies on: which units a number
is in, what the seven components of a state vector mean, and how simulated time relates
to wall-clock time.

## Geometrized units

The simulator works in geometrized units, so the speed of light is $c = 1$. Every
velocity is therefore a fraction of the speed of light, and a velocity of `0.5` means half
light speed. Lengths and times share one unit, so a distance of `1` is the distance light
travels in one unit of time.

Nothing in the program is expressed in meters or seconds. The metric has no intrinsic
length scale, so results scale to whatever bubble radius you care to assume.

## The state vector

Each particle, and the ship, carries a seven-component state:

$$\left[\, x,\; y,\; z,\; v^x,\; v^y,\; v^z,\; e \,\right]$$

| Component | Meaning |
|---|---|
| $x, y, z$ | Position in the global coordinate chart, in physics units |
| $v^x, v^y, v^z$ | Coordinate velocity $\mathrm{d}x^i/\mathrm{d}t$, as a fraction of $c$ |
| $e$ | The energy factor that keeps the state normalized |

This is the layout you get in an exported dump. See
[File formats](file-formats.html) for the surrounding structure.

### Normalization

A state is valid only if it satisfies the right normalization for its particle type.

- A **massive** particle obeys $e = 1/\sqrt{1 - v^2}$, the Lorentz factor.
- A **photon** obeys $|v| = 1$.

The simulator can check this every step and act when it fails. See
[Validate numerics](validation.html).

Two consequences follow. A massive particle can never reach $|v| = 1$, which is why the
velocity spread sliders are clamped so that $v_x^2 + v_y^2 + v_z^2 < 1$. And a photon
cannot be given a speed at all, only a direction; the simulator renormalises it.

## Two length scales

The program keeps physics coordinates and screen coordinates apart.

| Constant | Value | Role |
|---|---|---|
| `PHYSICS_SCALING_FACTOR` | `10.0` | Multiplies $x$ and $y$ to get game units |
| `DUST_SPAWN_LEAD` | `275.0` | Where new particles appear ahead of the ship, in game units |
| `DUST_CULL_DRAG_X` | `275.0` | Longitudinal culling distance, in game units |
| `DUST_CULL_DRAG_Y` | `160.0` | Transverse culling distance, in game units |

So a physics length of `4` draws as `40` game units. A slider labeled in game units is
ten times the physics quantity.

> [!NOTE]
> The $z$ coordinate is deliberately **not** scaled. In a two-dimensional view $z$ only
> orders the draw and picks the color, and multiplying it by ten made particles vanish
> through the near clipping plane at high zoom. The physics still uses the true $z$; only
> the transform to screen space leaves it alone.

Position spread sliders are in game units. Velocity spread sliders are in physics units.
The [Parameters](parameters.html) reference marks each one.

## Time

Three clocks run at once, and it helps to keep them apart.

| Clock | Rate | What it drives |
|---|---|---|
| Bevy `FixedUpdate` | 2000 Hz | The schedule that contains everything below |
| Physics tick | 60 Hz, times the speed factor | One integration step |
| Frame | Display refresh | Camera, input, the Parameters window |

A physics tick advances the simulation time by `PHYSICS_STEP_SIZE = 0.1` in physics
units, and takes one adaptive Runge-Kutta 4 step. The **Simulation Speed** slider
multiplies the 60 Hz gate, so at `2.0` the simulator takes physics ticks twice as often
and simulated time advances twice as fast per second of wall-clock time. It does not
change the step size, so it does not change accuracy.

When the machine cannot take ticks fast enough, the **Speed** tab shows
`Can't keep up!`. The simulation stays correct and runs slower than requested.

> [!NOTE]
> Simulated time is the `global_time` recorded in every checkpoint. It is the only clock
> that matters for reproducibility. Two runs that reach the same `global_time` from the
> same checkpoint agree exactly, whatever the frame rate was.
