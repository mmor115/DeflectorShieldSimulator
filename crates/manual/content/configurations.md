This chapter covers saving a run to a file, loading it back, and what to send a colleague
so they can reproduce your setup.

A configuration holds the parameters of a run: the bubble and deflector geometry, the
particle settings, the display options and the validator settings. It does not hold the
particles themselves. To capture those, export a history instead. See
[Export trajectory data](exporting.html).

## Save the current setup

1. Open the **Save & Load** tab of the **Parameters** window.
2. Type a name in the **Config Path** field, or leave it empty to use `sim.json`.
3. Click **Save Config**.

The file is written as indented JSON, so you can read it and edit it by hand.

> [!WARNING]
> The path is resolved against the directory the program was started from, not the
> directory the executable lives in. If you launched from a desktop menu, that may not be
> where you expect. Give an absolute path when it matters.

## Load a saved setup

1. Put the file name in **Config Path**.
2. Click **Load & Apply Config**.

Every parameter is restored at once, and the ship state is rebuilt for the loaded speed
and drag.

> [!WARNING]
> Loading goes through the same code path as the **Drag** slider. A configuration whose
> speed and drag the physics core rejects terminates the program. Save your current work
> before loading a file you did not write yourself. See
> [Known limitations](known-limitations.html).

If the file cannot be read or parsed, a dialog reports the error and nothing changes.

## Use the shipped examples

The repository has two configurations in `configs/`.

| File | What it shows |
|---|---|
| `cool.json` | A bubble running with both rings hidden, at a small positive slippage |
| `pos_slippage.json` | Positive slippage with an incoming velocity spread, the case the name describes |

Copy one next to the executable, put its name in **Config Path**, and load it.

## Edit a configuration by hand

The format is stable and documented field by field in
[File formats](file-formats.html). Editing by hand is the practical way to set a
parameter precisely, because a slider cannot land on an exact value.

Three points make hand-editing safe:

- Fields added after a file was written have defaults, so an older configuration still
  loads. The three deflector fields default to `1.0`, `1.0` and `0.8`.
- `validator_settings` may be omitted entirely.
- Any field you do include must have the right type. A string where a number belongs
  fails the whole load.

## Reproduce the published runs

To match the paper, set **Sigma Pushout** and **Sigma Factor** to `1.0` and use the
parameter values listed in
[The deflector shield](deflector-shield.html#reproducing-the-published-configuration).
Save the result and keep it with your figures.

## What to publish with a result

Send three things, and someone else can reproduce your figure exactly:

1. The release tag of the simulator, from the **About** tab.
2. The configuration file.
3. A history dump, if the result depends on the specific particles rather than on the
   geometry alone.

See [How to cite](citing.html).
