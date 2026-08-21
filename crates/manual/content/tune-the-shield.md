This chapter covers the **Bubble** tab: what each parameter changes, the constraint that
links **Speed** to **Drag**, and what the shutdown button does.

For the metric these parameters appear in, see
[The deflector shield](deflector-shield.html).

Everything here assumes the **Warp Drive** on the Drive tab is set to **Ours**, which is
the default. Select **Natario** instead and only **Radius**, **Sigma** and **Speed** stay
live; see [The Natario drive](natario-drive.html).

## Set the size of the bubble

Two sliders fix the geometry the ship sits in.

1. Drag **Radius** to set the extent of the flat interior. Range `1.0` to `4.0`, default
   `4.0`. The inner ring on screen moves with it.
2. Drag **Sigma** to set the thickness of the transition wall. Range `0.1` to `4.0`,
   default `4.0`. The outer ring sits one **Sigma** beyond the inner one.

Both are in physics units. On screen they appear ten times larger. See
[Physics, units and time](units.html).

Changing either value rebuilds the ring meshes, so the display follows immediately.

## Set the strength and reach of the deflector

Three sliders shape the shell that pushes particles aside.

| Slider | Range | Default | Effect |
|---|---|---|---|
| **Deflection Strength** | `0.0` – `0.9` | `0.1` | How hard the shell pushes. Particles are dragged transversely at up to this fraction of $c$. |
| **Sigma Pushout** | `0.0` – `4.0` | `0.8` | How far beyond **Radius** the shell sits, in multiples of **Sigma**. |
| **Sigma Factor** | `0.0` – `4.0` | `1.0` | How thick the shell is, in multiples of **Sigma**. |

To move the deflection further from the ship without changing the bubble, raise
**Sigma Pushout**. To spread the same deflection over a longer stretch of the trajectory,
raise **Sigma Factor**.

> [!NOTE]
> The published configuration uses **Sigma Pushout** `1.0` and **Sigma Factor** `1.0`,
> which puts the shell exactly one transition width beyond the bubble radius. The default
> of `0.8` sits slightly inside that. Set both to `1.0` to match the paper.

## Choose front-only or all-round deflection

**Deflector Back** takes one of two values in practice.

| Value | Behavior |
|---|---|
| `1.0` (default) | The shell deflects in every direction around the ship. |
| `0.0` | The shell deflects ahead of the ship and switches off behind it. |

Set it to `0.0` when you want debris turned aside on approach and then released, rather
than pushed for the whole passage. Values in between interpolate, but the two ends are
what the model is built around.

## Set the speed and the slippage

These two sliders interact, and this is the part worth reading carefully.

- **Speed** (`u`, range `0.0` – `0.9`, default `0.5`) is the speed of the bubble itself.
- **Drag** (`u0`, range `0.0` – `0.9`, default `0.5`) is the amplitude of the shift inside
  the bubble.

The ship travels at the difference:

$$u_s = u - u_0$$

Set them equal and the ship holds station while the bubble advances. Make **Drag** smaller
than **Speed** and the ship moves forward relative to the bubble, which the paper calls
positive slippage. Make it larger and the ship falls back.

### The constraint

The ship cannot be given a coordinate velocity above $c$, so $u - u_0 \leq 1$ must hold.
The two sliders enforce this differently, and the difference matters.

**Speed** is safe. If a change would push the resulting `u0` outside `[0.1, 0.9]`, the
slider reverts and shows `Shield Drag out of range!` in red. Nothing breaks.

> [!WARNING]
> **Drag** is not safe. Moving it calls into the physics core to re-solve the state of the
> ship, and a value the core rejects raises a panic that terminates the program without
> saving. Move it in small steps, and save your configuration first. The same path runs
> when you load a configuration file, so a bad file can also terminate the program. See
> [Known limitations](known-limitations.html).

## Turn the shield off and on

The button at the bottom of the tab reads **Shut down** while the shield is running.

Click **Shut down** to set **Speed**, **Drag** and **Deflection Strength** to zero. The
bubble and the deflector disappear, both rings are hidden, and particle spawning stops.
The current parameters are stashed rather than discarded, and the bubble keeps its
position, so any slippage you have accumulated survives.

The button now reads **Shut up**. Click it to restore the stashed parameters. The bubble
origin moves to wherever the ship has reached, and the ship state is rebuilt from there.

While the shield is shut down, the sliders in this tab edit the stashed values rather than
the live ones. Your edits take effect when you click **Shut up**.

## Keep a configuration you like

Once the shield behaves the way you want, open the **Save & Load** tab and click
**Save Config**. See [Save and reuse a configuration](configurations.html).
