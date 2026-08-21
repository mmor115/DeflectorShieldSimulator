# Documentation style

The manual in `crates/manual/content/` follows these rules. They are adapted from
ASD-STE100 Simplified Technical English, which was written so that maintenance manuals
read the same way to every reader, including readers whose first language is not
English. The manual serves two audiences at once — someone who has never heard of a warp
metric, and someone who reviews gr-qc papers — so it has to be plain without being vague.

Apply these rules when you write or edit any chapter.

## Sentences

- One instruction per sentence.
- Keep instructions to 20 words or fewer. Keep descriptive sentences to 25 or fewer.
- Use the active voice. Write "the validator removes the particle", not "the particle is
  removed".
- Use the present tense for how the program behaves.
- Use the imperative for steps the reader performs: "Click the particle."
- Keep paragraphs to six lines or fewer.

## Words

- Use one word for one meaning, everywhere. The particles are **particles**, never
  "dust", "specks", "motes" or "grains". The window is the **Parameters window**. A saved
  simulation state is a **checkpoint**; a sequence of them is a **history**.
- Keep articles. Write "the shield radius", not "shield radius".
- Do not stack more than three nouns. Rewrite "particle velocity spread slider value" as
  "the value of the velocity spread slider".
- Physics precision beats simplification. Keep *geodesic*, *metric*, *normalised state*,
  *geometrized units*. Define each once, in the Reference part, and link to it.
- Spell UI labels exactly as they appear, in bold: **Deflection Strength**, not
  "deflection strength".
- Mark keys with `<kbd>`: <kbd>Space</kbd>.

## Things not to write

These make documentation sound generated, and they waste the reader's time.

- Filler openers: "It is important to note that", "Keep in mind that", "Simply put".
- Empty intensifiers: "simply", "just", "easily", "seamlessly", "powerful", "robust",
  "comprehensive", "leverage", "utilize", "delve".
- Rule-of-three flourishes: "fast, flexible, and fun".
- A dash that restates the clause before it in different words.
- A closing paragraph that summarises the section the reader has just read.
- Promises about the future: "will be supported soon".
- Praise for the software. Describe what it does and let the reader judge.

## Structure

Chapters follow Diátaxis. Do not mix the modes within one chapter.

| Part | Mode | The reader wants |
|---|---|---|
| Getting started | Tutorial | To succeed at something once, guided |
| How-to guides | Task | To finish a specific job they already have |
| Reference | Description | To look up an exact value |
| Explanation | Discussion | To understand why the thing works |

- Start every chapter with one sentence saying what the reader gets from it.
- In how-to guides, number the steps. Put the outcome at the end of the last step.
- In reference chapters, put the facts in a table. Every number must be traceable to a
  line of source; check it against the code rather than against another chapter.
- Use `> [!NOTE]` for context the reader can skip, and `> [!WARNING]` for something that
  loses data or crashes the program.

## Honesty

State the limits. If a control can crash the simulator, say so and say when. If a number
is a display bound rather than physics, say that. A research tool earns trust by being
accurate about what it cannot do, and the manual is where that happens.
