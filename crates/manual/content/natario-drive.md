This chapter describes the second metric the simulator can integrate: the Natário
zero-expansion drive.

## Why there are two drives

The **Drive** tab selects which warp-drive metric the simulator evolves.

| Drive | What it is |
|---|---|
| **Ours** | The CCT warp drive of the paper, with an optional deflector term. This is the default and the subject of the rest of this manual. |
| **Natario** | A Natário zero-expansion drive. It carries a ship but has no deflector. |

Both drives share the same ADM form and the same bubble motion described in
[The deflector shield](deflector-shield.html):

$$ds^2 = -\,\mathrm{d}t^2 + \sum_i \left(\mathrm{d}x^i - v^i\,\mathrm{d}t\right)^2$$

$$x_{\mathrm{b}}(t) = x_0 + u\,(t - t_0)$$

They differ in the shift vector $\vec{v}$.

## The transition function

The Natário drive is spherically symmetric about the bubble center. It uses the radius

$$r = \sqrt{x^2 + y^2 + z^2 + \epsilon}$$

and a transition function that is $1$ inside the bubble, $0$ outside it, and smooth
across the shell between:

$$
f(r) =
\begin{cases}
1 & r < R \\[4pt]
\dfrac{(R + \sigma - r)^4 \left[20 (r - R)^3 + 10 (r - R)^2 \sigma + 4 (r - R)\sigma^2 + \sigma^3\right]}{\sigma^7} & R \le r \le R + \sigma \\[8pt]
0 & r > R + \sigma
\end{cases}
$$

$f$ and its first two derivatives are continuous at both $r = R$ and $r = R + \sigma$, so
the shift vector has no kinks for the integrator to trip over.

Note the shell here runs from $R$ to $R + \sigma$, matching the inner and outer surfaces
the simulator draws. The deflector shell of the other drive is positioned separately by
**Sigma Pushout** and **Sigma Factor**, which is why those sliders do nothing here.

## What this drive does not have

The Natário drive has no deflector term and no drag term. Five controls on the
**Bubble** tab are therefore grayed out while it is selected:

- **Drag** ($u_0$)
- **Deflection Strength** ($k_0$)
- **Sigma Pushout**
- **Sigma Factor**
- **Deflector Back**

Hovering a grayed-out slider shows "Selected warp drive does not use this parameter."
Only **Radius**, **Sigma** and **Speed** have any effect.

Because there is no drag, there is no slippage between the ship and the bubble, and none
of the "Shield Drag out of range!" behavior described in
[Parameters](parameters.html) applies. **Speed** moves freely over its whole range.

## The ship

Started fresh, the ship sits at the bubble center with zero coordinate velocity and is
carried by the shift vector alone. This is the usual warp-drive picture: the ship is at
rest locally, and the geometry does the traveling.

> [!WARNING]
> Switching drives on the **Drive** tab does not re-solve the ship state. The ship keeps
> whatever velocity it had under the previous drive, which for a switch away from the
> CCT drive means it keeps moving at $u - u_0$ rather than riding with the bubble.
> Reload a configuration, or restart, to get a ship state that matches the drive. See
> [Known limitations](known-limitations.html).

**Shut down** sets **Speed** to zero, as it does for the other drive. **Shut up**
restores **Radius**, **Sigma** and **Speed**, and re-solves the ship state from them.

## Comparing the two drives

Select this drive to watch particle geodesics in a bubble that has *no* deflector, and
compare them against the same configuration with the deflector switched on.
