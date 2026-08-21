This chapter describes the license on the source code, on the third-party dependencies, on the ship sprite, and on the manual's math fonts.

## Source code: AGPL-3.0-or-later

The source is licensed under the GNU Affero General Public License, version 3 or later.
You may use, study, modify and redistribute the source. A derivative work carries the
same license, and its source must be available to the people who use it.

The AGPL's network clause requires publishing source to users who interact with the
program over a network. That clause is inert for the simulator as distributed today, a
desktop program with no server component. It would apply if someone built and hosted a
web version that users reached over a network.

## Dependencies

The third-party Rust crates the simulator depends on use permissive licenses: MIT,
Apache-2.0 and similar. `THIRD-PARTY-NOTICES.md` in the repository, and in every release
artifact, lists each dependency and its license.

## The ship sprite

The ship sprite is a raster derivative of one of the vector ships in the *Top down
spaceships* set, and carries a license separate from the AGPL-3.0-or-later license on the
source code:

| | |
|---|---|
| Author | arin48 |
| Source | <https://opengameart.org/content/top-down-spaceships> |
| License | [Creative Commons Attribution 4.0 International (CC-BY 4.0)](https://creativecommons.org/licenses/by/4.0/) |
| Changes | One `ship_*.svg` from the set, rasterised to a 908x756 PNG with an alpha channel |

CC-BY 4.0 permits redistribution, including inside a binary and for commercial use, and
requires attribution. Keep this attribution in place if you redistribute a build; it is
compiled into the executable and appears in the simulator's **About** tab.

## Manual math fonts

The manual renders mathematics with Latin Modern, distributed under the GUST Font
License.

## Copyright

Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt.

## The physics crate

The `deflector-core` crate, which integrates the geodesics, is published under AGPL-3.0 at
<https://github.com/lucass-carneiro/DeflectorShields>. It is statically linked into every
released binary, and its terms match this project's, so the combined work is
redistributable under AGPL-3.0-or-later.
