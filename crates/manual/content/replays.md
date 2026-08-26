A replay reloads a saved checkpoint and continues the run from there, reproducing the same particle trajectories as the original run.

> [!NOTE]
> This is resume-from-checkpoint, not a scrubbing video player. There is no timeline to drag. You pick a checkpoint index and the simulator jumps straight to it.

## What recording gives you to resume from

The simulator records a checkpoint on every physics tick automatically; see [exporting.html](exporting.html) for how to write that history to disk. A replay always starts from one of these recorded checkpoints, loaded back from a file.

## Load a history and resume it

1. Open the Parameters window and select the **Save & Load** tab.
2. Under **Loaded History**, click **Load History**. This reads the file named in **History Path**, in the format chosen with the **Binary**/**JSON** radio buttons.
3. The loaded file sits in a staging area, separate from the live, running history, until you resume from it.
4. Check the **Checkpoints:** count shown under **Loaded History** to see how many checkpoints the file holds.
5. Click **Resume from Start** to resume from checkpoint 0.
6. To resume from a specific checkpoint instead, type its index into the field beside **Resume from**.
7. Click **Resume from**.
8. Click **Resume from End** to resume from the last checkpoint in the file.
9. If the index you chose is out of range, an error popup reports it. Nothing changes; pick a valid index and try again.

Click **Unload History** at any point to discard the staged file without resuming from it. This does not touch the live, running history.

## What resuming restores

Resuming from checkpoint *N* replaces the running state with everything that checkpoint recorded:

- The **Particles**, **Visuals**, and shutdown settings in effect at that checkpoint.
- The physics parameters: bubble radius, sigma, speed, drag, and every deflector parameter (see [parameters.html](parameters.html)).
- The global simulation time.
- The RNG's exact internal state, not only its original seed.
- The ship, despawned and respawned at its recorded state.
- Every recorded particle, despawned and respawned with its recorded position, velocity, and id.
- The tag set saved with the history (see [tagging.html](tagging.html)).

Resuming does not restore the **Validation** tab's settings. Whatever NaN and normalization behavior is active before you resume stays active afterward.

Resuming also skips the interpolated shutdown transition that **Load & Apply Config** uses. It assigns the checkpoint's physics parameters directly, matching the recorded state exactly instead of easing into it.

## Resuming truncates the live history

After a resume, the live history keeps only the checkpoints up to and including the one you resumed from. Every later checkpoint from before the resume is gone. Recording then continues forward from that point, appending new checkpoints as usual. The file you loaded from is never modified by this.

## Why the continuation is reproducible

Every random draw in the simulation comes from one seeded xoshiro256++ generator (see [units.html](units.html)), including a new particle's spawn position, initial velocity, and id. A checkpoint records that generator's exact internal state, not a summary of it.

Resuming installs that exact state before the next tick runs. From that point on, every call the simulator makes to the generator returns the same sequence of values it returned after the original checkpoint. New particles spawn at the same simulated times, in the same positions, with the same ids, and their geodesics through the bubble play out identically.

## Worked example: verify determinism yourself

1. Tag a particle partway through a run (see [tagging.html](tagging.html)).
2. Select **JSON** with the **History Format** radio buttons.
3. Set **History Path** to `run-a.json`.
4. Click **Dump Entire History**.
5. Note the checkpoint index at which you tagged the particle.
6. Let the simulation run for a while longer.
7. Click **Load History** to stage the file you wrote in step 4.
8. Enter the noted checkpoint index in the field beside **Resume from**.
9. Click **Resume from**.
10. Let the simulation run forward for the same number of ticks it ran after that point originally.
11. Set **History Path** to `run-b.json`.
12. Click **Dump Entire History** again.
13. Compare `run-a.json` and `run-b.json` at the matching checkpoint indices. The tagged particle's position, velocity, and id match exactly between the two files.
