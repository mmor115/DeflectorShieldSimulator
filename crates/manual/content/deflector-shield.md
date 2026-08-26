This chapter explains what the simulator is integrating: the warp-drive metric, the
deflector term added to it, and the job each slider does in that metric.

It describes the drive the **Warp Drive** control calls **Ours**, which is the default
and the one the paper is about. For the alternative, see
[The Natario drive](natario-drive.html).

## The geometry

The construction follows Alcubierre. Spacetime is flat everywhere except inside a
localised bubble, and the bubble carries a shift vector $\vec{v}$ that drags the
coordinates along with it. In ADM form the line element is

$$ds^2 = -\,\mathrm{d}t^2 + \sum_i \left(\mathrm{d}x^i - v^i\,\mathrm{d}t\right)^2$$

Everything the drive and the shield do lives in $\vec{v}$. A particle with no
coordinate velocity is still transported, because the coordinates themselves move.

The bubble travels along the $x$ axis at constant speed:

$$x_{\mathrm{b}}(t) = x_0 + u\,(t - t_0)$$

All the field functions below are evaluated in coordinates centered on the bubble, so
$x$ means $x - x_{\mathrm{b}}(t)$, and

$$r = \sqrt{x^2 + y^2 + z^2 + \epsilon}$$

The $\epsilon$ is a guard against dividing by zero at the center, not a physical quantity.

## Notation

The paper and the source use different names for the same quantities. This manual uses
the slider labels, because that is what you see on screen.

| Slider | Source field | Paper | Meaning |
|---|---|---|---|
| **Radius** | `radius` | $R$ | Bubble radius |
| **Sigma** | `sigma` | $\sigma$ | Transition width |
| **Speed** | `u` | $u_b$ | Bubble speed |
| **Drag** | `u0` | $u_d$ | Shift amplitude inside the bubble |
| **Deflection Strength** | `k0` | $k$ | Deflection strength |
| **Deflector Back** | `deflector_back` | $B$ | Rear deflection on or off |
| **Sigma Pushout** | `deflector_sigma_pushout` | fixed at 1 | Shell offset, in units of $\sigma$ |
| **Sigma Factor** | `deflector_sigma_factor` | fixed at 1 | Shell width, in units of $\sigma$ |
| — | — | $u_s$ | Ship speed, $u_s = u_b - u_d$ |

## The transition function

One smooth step function does all the shaping. Write $f(w; w_0, \Delta)$ for a function
that equals 1 below $w_0$, falls to 0 at $w_0 + \Delta$, and joins both ends smoothly:

$$f(w; w_0, \Delta) = \frac{\left(\Delta^3 + 4\Delta^2 s + 10\Delta s^2 + 20 s^3\right)\left(\Delta - s\right)^4}{\Delta^7}, \qquad s = w - w_0$$

It is a degree-seven polynomial with three continuous derivatives at both joins, which is
what keeps the geodesic integrator well behaved as a particle crosses the wall.

## The longitudinal part: carrying the ship

The $x$ component of the shift is one transition, keyed to the bubble radius:

$$v^x = u_0\, f(r;\, R,\, \sigma)$$

So $v^x = u_0$ throughout the interior, drops through the shell of thickness $\sigma$,
and vanishes outside. Two sliders control it:

- **Radius** sets $R$, the extent of the flat interior.
- **Sigma** sets $\sigma$, the thickness of the transition wall.

## The transverse part: the deflector

The deflector is an added component, constructed in the same style as the warp drive
itself: a localized shift-vector term that uses the same transition function, but pushing things out
transversely rather than along the axis of travel. It is optional. Set **Deflection
Strength** to zero and the metric is a drive with no shield. With it on, it pushes
matter sideways, away from the axis. Its magnitude is

$$v_{\mathrm{base}} = k_0 \cdot f_a \cdot f_b \cdot f_c$$

and it is applied along the transverse radial direction:

$$v^y = \frac{y}{\sqrt{y^2+z^2+\epsilon}}\, v_{\mathrm{base}}, \qquad v^z = \frac{z}{\sqrt{y^2+z^2+\epsilon}}\, v_{\mathrm{base}}$$

The three factors place and shape the deflecting shell. Let

$$r_0 = R + p\,\sigma, \qquad s_0 = q\,\sigma$$

where $p$ is **Sigma Pushout** and $q$ is **Sigma Factor**. Then

$$f_a = f(r - r_0;\, 0,\, s_0), \qquad f_b = f(r_0 - r;\, 0,\, s_0)$$

$f_a$ switches off outside $r_0$ and $f_b$ switches off inside it, so their product is a
bump centered on the sphere $r = r_0$ with a reach of $s_0$ to either side. In words:

- **Deflection Strength** ($k_0$) sets how hard the shell pushes.
- **Sigma Pushout** ($p$) sets how far outside the bubble radius the shell sits, in
  multiples of $\sigma$.
- **Sigma Factor** ($q$) sets how thick the shell is, in multiples of $\sigma$.

> [!NOTE]
> The published deflector function fixes the shell at $r = R + \sigma$ with a reach of
> $\sigma$, that is $p = 1$ and $q = 1$. **Sigma Pushout** and **Sigma Factor** are the
> simulator's generalisation of those two constants. The shipped default for **Sigma
> Pushout** is `0.8`, which places the shell slightly inside the published position. Set
> both sliders to `1.0` to reproduce the geometry in the paper.

The default of `0.8` is deliberate, and it is chosen for the interactive case rather
than the published one. At `1.0`, with the rest of the shipped defaults, particles
gather in front of the ship instead of sliding around it: the shell sits far enough out
that arriving particles meet it close to head-on and are held there rather than turned
aside. Pulling it in to `0.8` restores the sliding behavior, which is what makes the
deflection legible on screen.

That is a statement about the default configuration, not about the physics. Nothing
stops you setting `1.0`, and you must set it to reproduce the paper. If you do and see
particles piling up ahead of the ship, that is this effect and not a fault.

## Front-only deflection

The last factor decides whether the deflector acts all the way around or only ahead of
the ship. With $b$ for **Deflector Back**:

$$f_c = (1 - b)\, f(\sigma - x;\, 0,\, \sigma) + b$$

Two settings matter, and the parameter is meant to take one of them.

| **Deflector Back** | $f_c$ | Effect |
|---|---|---|
| `1.0` | $f_c = 1$ everywhere | The shell deflects in every direction |
| `0.0` | $f_c = f(\sigma - x;\, 0,\, \sigma)$ | The shell deflects ahead of the ship and switches off behind it |

At $b = 0$ the factor is 1 for $x > \sigma$, falls through the region $0 < x < \sigma$,
and is 0 for $x < 0$. Debris is turned aside on approach and then released, rather than
being pushed for the whole passage. Values between 0 and 1 interpolate, but the physical
reading is clearest at the two ends.

## Speed, drag and slippage

Two parameters set the motion, and the difference between them is the interesting one.

- **Speed** ($u$) is the speed of the bubble itself, which fixes $x_{\mathrm{b}}(t)$.
- **Drag** ($u_0$) is the amplitude of the longitudinal shift inside the bubble.

The ship is placed on the geodesic whose coordinate velocity is

$$v^x_{\mathrm{ship}} = u - u_0$$

Set $u_0 = u$ and the ship holds station in the global chart while the bubble advances.
Set $u_0 = 0$ and the ship moves at the bubble speed. Anything in between leaves the ship
drifting relative to the bubble center, and the drift accumulates.

The paper calls this drift **slippage**, and its sign matters:

| Condition | Name | The ship |
|---|---|---|
| $u_d < u_b$ | Positive slippage | Moves forward relative to the bubble |
| $u_d = u_b$ | None | Holds station in the bubble frame |
| $u_d > u_b$ | Negative slippage | Falls back relative to the bubble |

The repository ships `configs/pos_slippage.json` as a worked case of positive slippage.

> [!WARNING]
> The construction requires $u - u_0 \leq 1$: the ship cannot be handed a superluminal
> coordinate velocity. The **Speed** slider reverts and reports `Shield Drag out of
> range!` when a change would break this. Moving **Drag** re-solves the ship state
> directly, and a rejected value terminates the program. See
> [Known limitations](known-limitations.html).

## Shutting down

**Shut down** sets $u$, $u_0$ and $k_0$ to zero, which removes the bubble and the
deflector and leaves flat space. It keeps the current bubble position rather than
resetting it, so accumulated slippage survives. **Shut up** restores the stashed
parameters, moves the bubble origin to wherever the ship now is, and rebuilds the ship
state from there.

## Reproducing the published configuration

The paper's simulations use the values below. Set them in the **Bubble** tab, or write
them into a configuration file and load it. See
[Save and reuse a configuration](configurations.html).

| Slider | Paper value |
|---|---|
| **Radius** | `4` |
| **Sigma** | `4` |
| **Speed** | `0.5` |
| **Sigma Pushout** | `1.0` |
| **Sigma Factor** | `1.0` |
| **Deflection Strength** | `0.9` for the full shield, `0.45` for a gentler one |
| **Deflector Back** | `0` for front-only, `1` for all-round |

> [!NOTE]
> Every value in that table is stated in the paper except **Sigma Pushout** and **Sigma
> Factor**, which are derived. The paper has no such parameters: its deflector shell is
> written as a fixed expression centered on $R + \sigma$ with a reach of $\sigma$. Those
> two sliders are the simulator's generalisation of that fixed shell, and `1.0` and
> `1.0` are the settings at which the generalisation collapses back to the published
> form.

The paper's preferred case runs at a slight negative slippage, $u_s = -0.01$, which means
setting **Drag** to $u_b - u_s = 0.51$. Its debris fields give the incoming particles
velocities of about one per cent of $c$, which corresponds to a velocity spread of `0.01`
on the **Particles** tab.

> [!NOTE]
> The paper analyzes the metric with a piecewise-linear transition function, because it
> keeps the algebra tractable. The simulator integrates the smooth $C^3$ polynomial
> version. The two agree on the geometry and differ in the detail of the wall.

## What a geodesic through this means

The particles carry no charge and feel no force. They follow geodesics of the metric
above, so every deflection you see is the shift vector transporting the coordinates. That
is the point of the exercise: a shield that never touches the debris, and deflects it by
reshaping the space it crosses.

What the simulator shows is the kinematics of that idea.
