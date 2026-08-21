This chapter says what the simulator models, who it is for, and what you may and may not
conclude from a run.

## The question

The Alcubierre warp drive is normally discussed as a way to travel faster than light.
This simulator asks a narrower and more practical question. At sub-light speed, does the
warp bubble deflect the debris a ship would otherwise collide with?

At any appreciable fraction of light speed, a grain of dust carries the energy of an
artillery shell. A ship that cannot avoid debris cannot travel. So a bubble that turns
debris aside is useful even to a ship that never exceeds $c$.

## What the program does

The simulator integrates geodesics through a warp-drive metric that has been modified to
push matter away from the axis of travel. You get:

- A live view of particles falling through the shield, with the ship at the centre and
  the two shield boundaries drawn as rings.
- Sliders for every parameter of the geometry, applied while the simulation runs.
- Massive particles and photons, mixed in whatever proportion you choose.
- A second metric, a Natario zero-expansion drive with no deflector, to compare against.
- Deterministic replay from any recorded checkpoint.
- Export of every particle state to JSON or to a compact binary format.

The physics comes from the [`deflector-core`](https://github.com/lucass-carneiro/DeflectorShields)
crate, which is the same code the paper used. This program is the interactive front end
for it.

## Who it is for

Two readers, and the manual serves both.

If you are here out of curiosity, read [Install and run](install.html) and
[Your first simulation](first-simulation.html), then stop. Those two chapters get you a
running program and a feel for what the parameters do.

If you are here to do research, the chapters that matter are
[Run a reproducible replay](replays.html), [Export trajectory data](exporting.html) and
[Physics, units and time](units.html). Read
[The deflector shield](deflector-shield.html) for the metric, and
[Limits of the model](limits.html) before you cite anything.

## What a run does and does not establish

The particles carry no charge and feel no force. Every deflection you see comes from the
geometry transporting the coordinates the particle moves through.

A run therefore tells you about the **kinematics** of the shield: whether a given
geometry turns a given debris field aside, by how much, and at what standoff distance.

A run does not tell you whether the geometry is physically realisable. The simulator
never evaluates the stress-energy required to hold the metric, and never checks an energy
condition. It is also a two-dimensional view of a three-dimensional problem. These limits
are set out in full in [Limits of the model](limits.html), and you should read that
chapter before drawing a conclusion from a figure.

## The paper

> [!NOTE]
> L. T. Sanches, M. Morris and S. R. Brandt, "Exploring Particle Geodesics in a Warp Drive
> Spacetime", *Classical and Quantum Gravity* (accepted); [arXiv:2608.08213](https://arxiv.org/abs/2608.08213) [gr-qc].

Cite it if either this simulator or `deflector-core` contributed to your work. See
[How to cite](citing.html). The configuration used in the paper is listed in
[The deflector shield](deflector-shield.html).
