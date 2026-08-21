This chapter sets out what the simulator cannot tell you. Read it before you draw a
conclusion from a run.

## The geometry is assumed, not derived

The simulator takes the metric as given and integrates geodesics through it. It never
solves the Einstein field equations in the other direction, so it never computes the
stress-energy tensor the geometry would require.

This is the largest limitation and it is worth stating plainly. A shield configuration
that deflects debris beautifully on screen may demand a distribution of negative energy
density that no known matter provides. The simulator will not warn you, because it never
looks. Questions about energy conditions, about the total negative energy required, and
about quantum inequality bounds are outside what this program can answer.

## No back-reaction

Particles move in the metric. The metric does not respond to the particles. Their
stress-energy is not fed back into the geometry, and particles do not interact with each
other. Each trajectory is computed independently of every other one, which is also why
they can be integrated in parallel.

For a thin debris field this is a reasonable approximation. For a dense one it is not.

## Two dimensions on screen, three in the integrator

The integrator carries a full three-dimensional state. The display is a projection onto
the $x$–$y$ plane, and the $z$ coordinate only sets the colour and the draw order.

A particle with a large $z$ offset may look as though it passes close to the ship when it
misses by a wide margin out of the plane. Use the exported data rather than the picture
when the transverse geometry matters. See [Export trajectory data](exporting.html).

The $z$ coordinate is also not scaled by the same factor as $x$ and $y$, for reasons given
in [Physics, units and time](units.html). Do not read distances off the screen in $z$.

## The culling window is a display bound

Particles are removed once they pass roughly 275 game units from the ship along $x$, or
160 game units across it. That bound exists so the program stays responsive. It has no
physical meaning.

Two consequences follow. A trajectory recorded in a dump ends when the particle leaves
the window, not when anything physical happens to it. And a shield whose effect only
appears far downstream will look as though it does nothing, because the particle is gone
before the effect accumulates.

## Numerical accuracy is bounded and configurable

The integrator is an adaptive fourth-order Runge-Kutta step over a fixed outer step of
`0.1` in physics units. Error accumulates over a long run.

The normalisation validator is the instrument for judging this. If states drift outside
the tolerance you set, the trajectories have lost accuracy, whatever they look like. Treat
a run that trips the validator as suspect rather than as an interesting result. See
[Validate numerics](validation.html).

## Initial conditions are pseudo-random, not sampled

Particles are spawned from a seeded pseudo-random generator with uniform spreads in
position and velocity. This is not a physically motivated debris distribution, and it is
not a converged Monte Carlo sample.

The seeding makes a run reproducible, which is what the replay system relies on. It does
not make a single run statistically representative. If you need a specific debris field,
construct it with `scripts/warpsim.py` rather than by adjusting the spread sliders. See
[Analyse dumps with warpsim.py](analysis.html).

## The interior is not modelled

Nothing inside the ship is simulated. There is no hull, no crew, and no tidal analysis.
The ship is a single geodesic with a sprite drawn on it.

## What the simulator is good for

Given all of the above, the program answers one kind of question well: for this geometry
and this incoming debris field, where do the particles go? That is a kinematic question,
and the answer is quantitative and exportable.

Treat everything else as a question for the paper, or for a different tool.
