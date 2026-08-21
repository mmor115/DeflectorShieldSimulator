Exporting writes checkpoints or a full run history to disk, in JSON or binary, so you can analyze it later or share it.

## Recording happens automatically

The simulator appends a checkpoint to its live history on every physics tick, whether or not you plan to export it. Pausing the simulation pauses recording too. Nothing you do in the **Save & Load** tab is required to start it.

Because recording never stops on its own, the live history grows without bound during a long session. The **Checkpoints:** counter in the **Save & Load** tab shows its current length. Use **Trim History**, described below, to keep it a manageable size.

## Dump a checkpoint or the whole history

1. Open the Parameters window and select the **Save & Load** tab.
2. Type a destination in **History Path**. Leave it blank to use the default name for the selected format: `dump.bin` for Binary, `dump.json` for JSON.
3. Choose **Binary** or **JSON** with the radio buttons next to **History Format**.
4. Optionally enable **Dump only tagged particles** (see [tagging.html](tagging.html)) to drop every untagged particle's state from the file.
5. Click **Dump Single Checkpoint** to write only the most recent checkpoint.
6. Click **Dump Entire History** instead to write every checkpoint recorded so far.

> [!WARNING]
> **History Path** and **Config Path** resolve against the working directory of the running process. That is not necessarily the location of the executable, or of any file you last loaded. A relative path can land somewhere unexpected.

## Choose a format

**Binary** encodes the history with bincode. It writes quickly and produces a small file that no text editor can read.

**JSON** writes the same data as readable text. It writes slowly and produces a much larger file, but you can open it directly. It is also the only format the Python helper in [analysis.html](analysis.html) reads.

Dumping the entire history in JSON can take a long time and produce a very large file; the button is colored to warn you. Prefer Binary for a long run. Reach for **Dump only tagged particles** first if you only care about one particle's trajectory.

## Trim the live history

Click **Trim History** to discard every checkpoint except the newest, freeing the memory the rest occupied. Recording keeps appending after a trim, so a long session needs trimming more than once.

Trimming affects only the live history. It has no effect on a history you have loaded for a replay, and no effect on any file already on disk.

## The exported record layout

A dump has two top-level parts: a list of checkpoints, and the tag set current when you dumped it. Each checkpoint carries the physics and UI settings in effect, the global simulation time, the RNG's internal state, the ship's state, and every particle's state. See [file-formats.html](file-formats.html) for the field-by-field schema.

> [!NOTE]
> A dump written with **Dump only tagged particles** keeps every checkpoint but strips the untagged particles from each one. Loading such a dump and resuming from it (see [replays.html](replays.html)) restores only the particles that were tagged when you dumped it.
