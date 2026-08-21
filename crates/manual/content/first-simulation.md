This tutorial takes about ten minutes. You will steer the view, change the shield, follow
one particle, and finish with a data file on disk.

Follow it in order. Later chapters explain each step in depth; here the goal is to get
the feel of the program.

## Before you start

Install and launch the simulator. See [Install and run](install.html). You should have a
dark window with a ship at the center, two blue rings around it, and colored particles
entering from the right.

## 1. Stop the clock

Press <kbd>Space</kbd>.

The particles freeze. Press <kbd>Space</kbd> again to resume. Pausing is the most useful
control in the program, because it lets you read a configuration off the screen before it
changes.

Leave the simulation running for now.

## 2. Move the view

Scroll the wheel forward. The view zooms in, and the ship stays centered because the
camera follows it.

Hold the right mouse button and drag. The view pans, and the offset persists as the ship
moves.

Click the middle mouse button. The zoom and the pan both reset.

That is every camera control. See [Controls](controls.html).

## 3. Read the rings

The two blue rings are the shield. The inner ring is at the bubble radius. The outer ring
is one transition width further out, so the gap between them is the wall the particles
cross.

Watch a particle enter from the right. Outside the outer ring it travels in a straight
line. As it reaches the wall it turns aside.

## 4. Change the shield

Open the **Bubble** tab of the **Parameters** window.

Drag **Radius** from `4` to `2`. Both rings shrink, and particles now pass much closer to
the ship before turning.

Drag **Radius** back to `4`, then drag **Deflection Strength** from `0.1` to `0.6`. The
rings do not move, but the particles turn much harder.

Now set **Deflection Strength** back to `0.1`.

> [!WARNING]
> Leave the **Drag** slider alone for this tutorial. Moving it re-solves the state of the
> ship, and a value the physics core rejects terminates the program. See
> [Known limitations](known-limitations.html).

## 5. Narrow the stream

Open the **Particles** tab.

Drag **y-Position Spread** down to about `20`. Instead of filling the screen, particles
now arrive in a narrow band aimed at the ship. This makes single trajectories much easier
to follow.

## 6. Follow one particle

Press <kbd>Space</kbd> to pause.

Click a particle that has not yet reached the shield. It turns magenta. You have tagged
it, and the tag is stored against the identity of that particle rather than its position.

Open the **Visuals** tab and switch on **Hide untagged particles**. Every other particle
disappears. Your tagged particle stays visible.

Press <kbd>Space</kbd> to resume, and watch that one trajectory cross the shield with
nothing else in the way.

## 7. Slow it down

Open the **Speed** tab and drag **Simulation Speed** to `0.3`.

Simulated time now advances more slowly, so you can watch the turn in detail. The step
size does not change, so the trajectory is exactly the one you would have gotten at full
speed. See [Physics, units and time](units.html).

## 8. Export the state

Open the **Save & Load** tab.

Look at the `Checkpoints:` counter. It has been climbing since the program started,
because every physics tick records a checkpoint. Recording is always on.

Click **Dump Single Checkpoint**. A file called `dump.bin` appears in the directory you
started the program from, holding the exact state of every particle at this instant.

Now switch the format radio to **JSON** and click **Dump Single Checkpoint** again. You
get `dump.json`, which is larger and which you can open in a text editor. Find the
`particle_states` array and look at one entry: seven numbers, a UUID, and a particle type.
Those seven numbers are $[x, y, z, v^x, v^y, v^z, e]$.

## 9. Save the configuration

In the same tab, click **Save Config**. This writes `sim.json`, holding every slider
position you have set.

Keep that file. Loading it restores this exact setup, and it is the right thing to attach
to a result you publish. See [Save and reuse a configuration](configurations.html).

## Where to go next

You now have a configuration file and two data files.

| If you want to | Read |
|---|---|
| Understand what the sliders mean physically | [The deflector shield](deflector-shield.html) |
| Export a whole run rather than one instant | [Export trajectory data](exporting.html) |
| Re-run a trajectory exactly | [Run a reproducible replay](replays.html) |
| Read a dump in Python | [Analyze dumps with warpsim.py](analysis.html) |
