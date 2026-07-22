# Proposed Fixes — DeflectorShieldSimulator

Findings from a joint review of **DeflectorShields**, **DeflectorShieldSimulator**, and **WarpDrivePaper**. This document covers bugs and consistency issues in the Bevy interactive simulator and its integration with `deflector-core`.

Cross-repo partners:

- Core: `../DeflectorShields/proposed_fixes.md`
- Paper: `../WarpDrivePaper/proposed_fixes.md`

---

## Priority legend

| Priority | Meaning |
|----------|---------|
| P0 | Wrong simulation physics or data corruption |
| P1 | Feature breaks, panics on normal UI paths, paper/tool mismatch |
| P2 | Visual / UX / robustness |
| P3 | Polish |

---

## P0 — Simulation correctness

### 1. Global time is advanced **before** the RK4 step

**Where:**

- `src/game_systems/fixed_update/pre_update_physics.rs` — `incr_global_time()`
- `src/physics/physics_manager.rs` — `step_particle` passes `self.global_time` into RK4
- Chain order in `src/main.rs`: `pre_update_physics` → `update_ship` / `update_space_dust` → …

**Problem:**

1. First step integrates over \([dt, 2dt]\), never \([0, dt]\).
2. After a step, particle state is at \(t+dt\), but bubble drawing uses `bubble_x_position()` at still-unbumped-or-wrong \(t\) relative to the integrated state.
3. History snapshots label `global_time` inconsistently with “state at that time.”

Standalone (`deflector-standalone`) integrates at time \(t\) then advances — correct ordering relative to the paper’s RK4 description.

**Proposed fix:**

- Option A (preferred): step particles with current `global_time`, **then** `incr_global_time()`.
- Option B: pass `global_time - step_size` into RK4 if increment stays first (more fragile).
- After the change, confirm bubble mesh `x` uses the same time as the post-step particle state (or document ship-frame plotting carefully).
- Revisit snapshot semantics: store time at **end** of step (state time) or **start** (integration time), consistently.

**Paper alignment:** Paper § numerical method describes advancing by RK4 with adaptive half-steps; it does not describe skipping the first interval or desyncing the bubble.

---

### 2. Switching drive type to Natário does not reset the ship

**Where:** `src/physics/physics_parameters.rs` — `WarpDriveImpl::new_from`

**Problem:**

- `Ours → Ours` / resume path updates ship via `WarpDriveOurs::resume` → `shut_up`.
- `* → Natário` only constructs `WarpDriveNatario { ... }` and **leaves** `ship_state` unchanged (e.g. leftover slip \(V^x = u-u_0\)).

Paper / core expect Natário ship Eulerian velocity \(V^i=0\).

**Proposed fix:** After building Natário parameters, set ship state via `make_ship_state(global_time)` (or equivalent), and update the ship transform.

**Also depends on core P0:** Natário `shut_up` currently sets \(V^x=u\) (wrong). Fix core first or bypass with explicit zeros here.

---

### 3. Config “Load & Apply” always goes through `shut_up` (type-locked + wrong Natário)

**Where:** `src/game_systems/ui.rs` — Load & Apply Config

```rust
physics_manager.physics_parameters.shut_up(global_time, ship_state, &params);
```

**Problems:**

- Panics if current drive kind ≠ config drive kind (`Ours` vs `Natário` arms).
- Cannot switch drives by loading a config.
- Inherits Natário ship-velocity bug from core.
- Does not update ship **transform** after changing physics state (sprite catches up next tick only).

**Proposed fix:**

- If kinds differ: replace `warp_drive` from config (via `PhysicsParameters::from`), then initialize ship with `make_ship_state` / kind-appropriate `shut_up`.
- If kinds match: keep restart semantics of `shut_up` **or** fully apply `x0,t0,...` from file depending on product intent (document which).
- Always refresh ship transform after load.

---

## P1 — Data integrity and UI paths

### 4. Binary history dump does not truncate

**Where:** `src/game_systems/ui.rs` — Dump history, `HistoryFormat::Bin`

```rust
OpenOptions::new().read(true).write(true).create(true);
// missing .truncate(true)
```

Writing a shorter dump over a longer file leaves trailing garbage → later loads can fail or deserialize incorrectly.

**Proposed fix:** Add `.truncate(true)` (and typically drop `.read(true)` for write-only dumps).

---

### 5. History resume incomplete / unsafe edge cases

**Where:** `src/game_systems/ui.rs` — resume from loaded history

**Problems:**

- Restores visual / particle / shutdown settings but **not** `validator_settings` from the snapshot’s `global_config`.
- “Resume from End” uses `snapshots_len - 1` with no guard when `snapshots_len == 0` (underflow / panic).

**Proposed fix:** Restore validators; disable resume buttons when empty; clamp index.

---

### 6. `sim.json` and related configs incomplete

**Where:** repo-root `sim.json` and possibly other hand-written configs

**Problem:** Missing `warp_drive_kind` (required by `PhysicsConfig` serde). Load fails.

**Proposed fix:** Add `"warp_drive_kind": "Ours"` (or appropriate value) to all example configs; validate on CI with a small serde round-trip test.

---

### 7. Python tooling enum casing

**Where:** `scripts/warpsim.py` default template

```json
"warp_drive_kind": "ours"
```

Rust expects `"Ours"` / `"Natario"` (serde variant names). Real dumps use correct casing; the template does not.

**Proposed fix:** Default to `"Ours"`; accept case-insensitive set in Python if desired; document allowed values.

---

### 8. Pinned `deflector-core` revision skew

**Where:** `Cargo.toml`

```toml
deflector-core = { git = "ssh://git@github.com/lucass-carneiro/DeflectorShields.git", rev = "7ee3b48" }
```

Local DeflectorShields may be ahead (field-plot, trait changes). Paper/experiments assume core behavior from research runs.

**Proposed fix:**

- Prefer path dependency for local multi-repo work:
  `deflector-core = { path = "../DeflectorShields/deflector-core" }`
- After core P0 fixes land, bump `rev` / tag and lockfile.
- Document in README which core rev matches paper figures.

---

### 9. Defaults disagree with paper figure geometry

**Where:** `src/physics/physics_parameters.rs` — `Default for PhysicsParameters`

```rust
WarpDriveOurs::new(4., 4., 0.5, 0.5, 0.1, 0.8, 1.0, 1.0)
// radius, sigma, u, u0, k0, pushout=0.8, factor=1, back=1
```

Paper base deflector and published dumps use **`deflector_sigma_pushout = 1.0`**, and first deflector study uses **`k0 = 0.9`**, not `0.1` / `0.8`.

**Proposed fix:** Align defaults with paper “plain” or document that UI defaults are exploratory. Suggested paper-aligned default pushout `1.0`.

---

### 10. Depends on core: Natário restart / photon normalize / parfiles

Simulator UI paths call into core:

| Path | Core issue |
|------|------------|
| Shut up / load config / `new_from` resume | Natário `shut_up` ship \(V^x=u\) |
| Photon spawn + variance | Photon `make_normalized_state` assert \(v^2<1\) |
| Zero photon velocity | Hardcoded leftward photon |

See `../DeflectorShields/proposed_fixes.md` P0 items. Fix core, then retest simulator Drive tab and particle spawn.

---

## P2 — Visual / UX / robustness

### 11. Colors use 0–255 with Bevy `Color::srgb` (0–1)

**Where:**

- `src/game_systems/setup.rs` — `TAGGED_PARTICLE_COLOR = Color::srgb(255., 0., 255.)`
- `src/game_entities/explosion.rs` — `EXPLOSION_COLOR = Color::srgb(255., 0.3, 0.)`

Bubble colors correctly use 0–1. Tagged particles / explosions are wrong (HDR / clipped).

**Proposed fix:** Use `Color::srgb(1., 0., 1.)` and `Color::srgb(1., 0.3, 0.)` (or `srgba` with alpha).

---

### 12. Ship Z when adjusting drag

**Where:** `ui.rs` drag slider sets

```rust
ship_transform.translation = physics_to_game(ship_state.0).xy().extend(-10.);
```

`update_ship` later overwrites with full `physics_to_game` (physical \(z\), usually 0). Inconsistent layering hack.

**Proposed fix:** Centralize sprite Z for ship (constant render layer) separate from physics \(z\).

---

### 13. Speed / timer vs fixed timestep

App uses `Time::<Fixed>::from_hz(2000.0)` while physics steps only when `PhysicsUpdateTimer` fires (~60 Hz base). Not wrong, but easy to confuse with “2000 physics Hz.”

**Proposed fix:** Comment in `main.rs` / README: fixed schedule is high-rate; actual geodesic steps follow the UI timer and `PHYSICS_STEP_SIZE`.

---

### 14. `PHYSICS_STEP_SIZE` vs paper

| | Paper | Simulator |
|--|-------|-----------|
| Default \(dt\) | 0.2 | 0.1 (`main.rs`) |

**Proposed fix:** Align with paper after authors choose one value; note adaptive half-step threshold `1e-8` / norm tol `1e-6` already match paper.

---

## P3 — Polish

### 15. Panic-heavy UI paths

Many `.expect("… failed")` on `shut_up`, `update_u0`, `make_normalized_state`. Prefer UI error text (existing `SaveLoadErr` popup pattern) instead of hard process kill where possible.

### 16. Comment in validators about dump-on-die

Validators note dumping bad state “once that's implemented” — history already snapshots; clarify or implement explicit crash dump.

---

## Experiment dumps (for regression tests)

Paper and this repo share experiment parameters (from `experiments/*/idump.json`). Useful golden table:

| Experiment | \(u\) | \(u_0\) | \(u_s\) | \(k_0\) | \(B\) | pushout |
|------------|------|--------|--------|--------|------|---------|
| plain_warp | 0.5 | 0.5 | 0 | 0 | 1 | 1 |
| positive_slippage | 0.5 | 0.4 | +0.1 | 0 | 1 | 1 |
| negative_slippage | 0.5 | 0.6 | −0.1 | 0 | 1 | 1 |
| deflector | 0.5 | 0.5 | 0 | 0.9 | 1 | 1 |
| deflector_noback | 0.5 | 0.5 | 0 | 0.9 | 0 | 1 |
| optimal | 0.5 | 0.51 | −0.01 | **0.5** | 0 | 1 |
| natario | 0.5 | n/a | 0 | n/a | n/a | n/a |

Paper text claims optimal \(k=0.45\); dumps use **0.5** (paper fix — see WarpDrivePaper).

**Proposed fix:** Add a small test or script that loads these JSONs into `PhysicsConfig` / `PhysicsParameters` without panic.

---

## Suggested fix order (this repo)

1. Time-step ordering (P0) — highest impact on all trajectories  
2. Binary dump truncate  
3. Drive switch + config load ship/drive handling (after core Natário fix)  
4. Defaults / example JSON / warpsim casing  
5. Color srgb range  
6. History resume edge cases  
7. Bump core pin after DeflectorShields P0  

---

## Out of scope here (see other repos)

- Core geodesic RHS, form functions, Natário \(v^i\) formulas  
- Standalone parfiles and overshoot loop  
- Paper analytic \(v_f\) formulas, prose, figure captions  
