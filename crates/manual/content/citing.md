This chapter gives the citation for the paper behind the simulator, and how to also cite the software itself.

## Cite the paper

Cite the paper if the simulator, or the `deflector-core` physics crate on its own,
contributed to your work:

> L. T. Sanches, M. Morris and S. R. Brandt, "Exploring Particle Geodesics in a Warp Drive
> Spacetime", Classical and Quantum Gravity (accepted); arXiv:2608.08213 [gr-qc].

BibTeX entry for the arXiv preprint:

```bibtex
@article{sanches2026geodesics,
  author        = {Sanches, L. T. and Morris, M. and Brandt, S. R.},
  title         = {Exploring Particle Geodesics in a Warp Drive Spacetime},
  eprint        = {2608.08213},
  archivePrefix = {arXiv},
  primaryClass  = {gr-qc},
  note          = {Accepted in Classical and Quantum Gravity},
  year          = {2026}
}
```

## Cite a specific software version

Cite the release tag alongside the paper when a result depends on the behaviour of a
particular build. Give the tag and the repository:

> DeflectorShieldSimulator, version v0.1.0.
> `https://github.com/max-morris/DeflectorShieldSimulator`

GitHub's "Cite this repository" button on the repository page reads the `CITATION.cff`
file and formats this citation for you, in several formats.

## What to record for reproducibility

When you publish a result produced with the simulator, record three things alongside it:

- The release tag you ran.
- The configuration file that set up the run.
- The history dump the run produced.

Together, these let another reader reproduce your trajectories: the tag fixes the code
and the physics, the configuration fixes the initial state, and the dump lets a reader
check their reproduction against yours without rerunning the simulation.
