# DeflectorShieldSimulator

[![CI](https://github.com/max-morris/DeflectorShieldSimulator/actions/workflows/ci.yml/badge.svg)](https://github.com/max-morris/DeflectorShieldSimulator/actions/workflows/ci.yml)

An interactive simulator for particle geodesics in a warp-drive deflector-shield
spacetime.

The simulator fires particles at a moving Alcubierre-type warp bubble and integrates
their geodesics through the resulting metric, so you can watch how the bubble deflects
incoming matter and radiation.

- Homepage and downloads: <https://max-morris.github.io/DeflectorShieldSimulator/>
- Manual: <https://max-morris.github.io/DeflectorShieldSimulator/manual/>
- Paper: [Exploring Particle Geodesics in a Warp Drive Spacetime](https://arxiv.org/abs/2608.08213) (arXiv:2608.08213 [gr-qc], accepted in Classical and Quantum Gravity)
- Physics crate: <https://github.com/lucass-carneiro/DeflectorShields>

## Install

Download a build for your system from the [homepage](https://max-morris.github.io/DeflectorShieldSimulator/).

To build from source instead, follow the Build from source appendix in the manual.

## Repository layout

| Path | Contents |
|---|---|
| `crates/simulator` | The simulator application |
| `crates/manual` | The documentation generator that builds the manual and the homepage |
| `configs/` | Example configuration files |
| `scripts/warpsim.py` | The Python helper for analyzing exported trajectory data |
| `docs/` | Contributor documentation |

## Citing

Cite the paper if the simulator, or the `deflector-core` physics crate on its own,
contributed to your work:

> L. T. Sanches, M. Morris and S. R. Brandt, "Exploring Particle Geodesics in a Warp Drive
> Spacetime", Classical and Quantum Gravity (accepted); arXiv:2608.08213 [gr-qc].

To also cite a specific release of the software, see [`CITATION.cff`](CITATION.cff) or
use GitHub's "Cite this repository" button on the repository page.

## Copyright and license

Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt.

The source code is licensed under [AGPL-3.0-or-later](LICENSE).

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

The `deflector-core` physics crate is also AGPL-3.0, so the combined work is
redistributable under those terms.

The embedded ship sprite is licensed separately, under CC-BY 4.0; see the attribution in
[`crates/simulator/assets/ATTRIBUTION.md`](crates/simulator/assets/ATTRIBUTION.md).
Third-party Rust dependency notices are in [`THIRD-PARTY-NOTICES.md`](THIRD-PARTY-NOTICES.md).
