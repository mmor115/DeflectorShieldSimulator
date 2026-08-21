Tagging marks one particle so you can track it, keep it on screen when others are hidden, and find it again after a replay.

## Tag a particle

1. Move the pointer over the particle in the simulation view.
2. Click it. The picking system finds only the topmost particle under the pointer, so a click never tags two overlapping particles at once.
3. Look at the particle. A tagged particle turns magenta, regardless of its depth or particle type.
4. Click the same particle again to untag it. It returns to its usual color, which encodes depth and particle type (see [units.html](units.html)).

## How a tag is stored

The simulator keeps tags as a set of particle ids, not as a list of entities. Each particle carries a `SpaceDustId`, a UUID assigned when it spawns. Tagging adds that UUID to the set; untagging removes it.

Storing ids rather than entities matters across a reload. Entities do not survive a save, but the UUIDs in the set do, and the simulator matches them against whichever particles are alive.

## Keep a tagged particle visible

Use this when you want to watch one particle's geodesic without the rest of the beam in the way.

1. Open the Parameters window and select the **Visuals** tab.
2. Enable **Hide untagged particles**.
3. Confirm the result: every untagged particle disappears, and every tagged particle stays visible.

Untagging a particle while this option is on hides it immediately, since it no longer counts as tagged.

## Tags in history dumps and replays

A tag set belongs to the whole history, not to one checkpoint. Dumping a history (see [exporting.html](exporting.html)) writes the current tag set alongside the checkpoints. Loading a history and resuming from it (see [replays.html](replays.html)) restores that same tag set into the running simulation, whichever checkpoint you resume from.

> [!NOTE]
> **Dump only tagged particles** removes the untagged particles' states from the file, but the tag set itself is always written in full.

## Why a tag survives a replay

A particle's UUID is not arbitrary. The simulator draws it from the seeded RNG at the moment the particle spawns, the same RNG that drives spawn position and initial velocity. A replay restores that RNG's exact internal state before continuing (see [replays.html](replays.html)), so it draws the same sequence of values afterward, including particle ids.

The particle you tagged gets the same UUID every time you replay past its spawn point. The tag keeps matching the same particle, run after run.
