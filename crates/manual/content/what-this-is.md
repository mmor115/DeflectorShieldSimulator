This chapter says what the simulator models and how to start using it.

## The question

The Alcubierre warp drive is normally discussed as a way to travel faster than light.
This simulator asks a narrower and more practical question. At sub-light speed, does the
warp bubble protect the ship from interstellar debris?

At any appreciable fraction of light speed, a grain of dust carries the energy of an
artillery shell. A ship that cannot avoid debris cannot travel. So a bubble that turns
debris aside is useful even to a ship that never exceeds $c$.

## What the program does

The simulator integrates geodesics through a warp-drive metric. An optional deflector
shield, constructed in the same style as the drive itself, can be added as an extra
shift-vector term that pushes matter away from the axis of travel. You get:

- A live view of particles falling through the bubble, with the ship at the center and
  the two bubble surfaces drawn as rings.
- Sliders for every parameter of the drive and of the shield, applied while the
  simulation runs.
- Massive particles and photons, mixed in whatever proportion you choose.
- A second metric, a Natario zero-expansion drive with no deflector, to compare against.
- Deterministic replay from any recorded checkpoint.
- Export of every particle state to JSON or to a compact binary format.

The physics comes from the [`deflector-core`](https://github.com/lucass-carneiro/DeflectorShields)
crate, which is the same code the paper used. This program is the interactive front end
for it.

## Getting Started
Start by reading [Install and run](install.html) and
[Your first simulation](first-simulation.html). Those two chapters get you a
running program and a feel for what the parameters do.

For studying the particle physics on a deeper level, read
[Run a reproducible replay](replays.html), [Export trajectory data](exporting.html) and
[Physics, units and time](units.html). Read
[The deflector shield](deflector-shield.html) for the metric.

## What a run establishes

The particles carry no charge and feel no force. Every deflection you see comes from the
geometry transporting the coordinates the particle moves through.

A run therefore tells you about the **kinematics** of the geometry: whether a given
drive, with or without the shield, turns a given debris field aside, by how much, and
at what standoff distance.

## The paper

> [!NOTE]
> L. T. Sanches, M. Morris and S. R. Brandt, "Exploring Particle Geodesics in a Warp Drive
> Spacetime", *Classical and Quantum Gravity* (accepted); [arXiv:2608.08213](https://arxiv.org/abs/2608.08213) [gr-qc].

Cite it if either this simulator or `deflector-core` contributed to your work. See
[How to cite](citing.html). The configuration used in the paper is listed in
[The deflector shield](deflector-shield.html).
