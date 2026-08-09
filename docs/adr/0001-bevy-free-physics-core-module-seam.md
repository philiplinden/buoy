---
status: accepted
---

# Keep buoy-physics as one crate; separate Bevy from the physics core with a module seam, not a crate split

**Context.** `buoy-physics` currently has `use bevy` in nearly every file. The long-term goal is
running buoy's physics for hardware-in-the-loop / software-in-the-loop test harnesses outside
Bevy, and Bevy's dependency graph measurably slows iteration on code that's otherwise pure
physics: ~150-170s clean build with Bevy (396 dependencies) vs. ~12-13s estimated without it (17
dependencies) — see [DEV-4](https://linear.app/linden-scientific/issue/DEV-4/measureestimate-current-compile-time-cost-bevy-imposes-on-buoy-physics).
A full crate split was already attempted once, in
[PR #42](https://github.com/philiplinden/buoy/pull/42) ("remove bevy dependency from
buoy-physics") — it's stale and was never merged; the team's actual follow-up
([PR #43](https://github.com/philiplinden/buoy/pull/43), merged) went the opposite direction and
consolidated `buoy-runtime` back into `buoy-physics`.

**Decision.** Keep `buoy-physics` as a single crate. Separate Bevy from the physics core with a
*module seam*: bevy-free core modules (plain functions and types — math, geometry, gas law,
atmosphere) versus ECS modules (`Component`/`Resource` derives, systems, `plugin()` fns), enforced
by a `--no-default-features` CI check that fails loud on a stray `use bevy::prelude::*` in a core
module. `buoy-ui` is unaffected — it's already its own workspace crate and stays that way.

Keep the physics core's geometry/math layer lean and hand-rolled (extending today's
`geometry.rs` pattern), rather than depending on `parry3d` for shapes/mass-properties.

For debug telemetry once the seam exists, use a separate `Reflect`-native descriptor/config type
for construction-time data plus `Resource`-based aggregate diagnostics for runtime state — no
per-entity component mirroring.

There is no specific trigger condition gating this work: build the module seam soon,
opportunistically, in the next available work session. The compile-time cost is already fully
present today, not a future threshold to wait for, and the seam itself is cheap to build
(reorganization + a CI check, not a rewrite).

## Considered Options

- **Full crate split** (`buoy-core`/`buoy-physics` bevy-free + a thin ECS wrapper crate), as
  PR #42 attempted. Rejected: mining that PR
  ([DEV-2](https://linear.app/linden-scientific/issue/DEV-2/mine-pr-42-for-concrete-crate-split-migration-costs))
  showed the real, hard cost is at the ECS boundary — any type used as a `Bundle` field must stay
  `Component`, forcing newtype wrappers and call-site rewrites on every one of them — while
  math/geometry types cross for free (glam types are literal re-exports, not conversions). The
  team's own directional history (PR #43) already chose fewer crate boundaries once.
- **Feature-flagged Bevy** (bevy as an optional Cargo feature on the current crate). Considered
  but not chosen, in favor of the module seam's clearer static enforcement (a CI-checked
  `--no-default-features` build) over feature-flag discipline alone.
- **`parry3d` for geometry/shapes.** Seriously considered — dimforge migrated `parry` off
  `nalgebra` onto `glam` via the `glamx` bridge crate (a drop-in re-export, no type-conversion
  tax), and it provides real shape/mass-properties/query machinery. Rejected: it's ~55
  dependencies, and most of its surface (spatial partitioning, mesh decomposition, arbitrary
  shape trait objects) solves collision-detection problems buoy doesn't have — `buoy` already
  removed Avian for the same reason (collision physics engines solve a problem this
  ideal-gas/buoyancy sim doesn't have). `geometry.rs` was already parry/Avian-collider-aware once
  and was deliberately stripped back to today's minimal `Shape` enum. Direction instead: stay
  lean and purpose-built, in the spirit of the `particular` crate (narrow scope, minimal deps) —
  pull in a narrow single-purpose crate later only if a specific hard sub-problem (e.g.
  inertia-tensor math for a convex-hull shape) justifies it.
- **`avian`'s telemetry machinery directly.** Rejected in favor of just the *pattern* it
  demonstrates (a `Reflect`-native descriptor type for construction-time data, plus `Resource`
  diagnostics for runtime state) — `avian` is a general-purpose library for many consumers; buoy
  is one app with a much narrower need.
- **Gating on a compile-time threshold or an external consumer appearing**, as the trigger
  condition
  ([DEV-6](https://linear.app/linden-scientific/issue/DEV-6/decide-the-concrete-trigger-condition-for-executing-the-split)).
  Rejected: the compile-time cost doesn't grow toward a future breach point, it's already fully
  present; gating on a rare external event (a HIL/SIL harness appearing) has no timeline, and the
  work is cheap enough not to need a forcing function.

## Consequences

- The physics core stays a stateless library of pure functions and plain types; Bevy's ECS
  remains the sole owner of physics state. This specifically rules out `bevy_rapier`'s dual-world
  + writeback-sync pattern as a template — Rapier needs that sync because it owns its own
  simulation state, buoy's module seam doesn't.
- The eventual deformable-membrane solver (mass-spring/XPBD-style, the long-term "endgame" beyond
  today's naive analytic envelope shapes) is expected to also fit as a pure function over
  explicit mesh-state data — not yet confirmed, worth revisiting when that solver is actually
  designed.
- Because there's no full crate split, buoy still can't be depended on as a truly bevy-free
  library by an external HIL/SIL harness without further work: Cargo `default-features = false`
  on `buoy-physics` gets the fast/bevy-free compile path once the module seam exists, but the
  crate is still `buoy-physics` itself, not a separately-publishable bevy-free crate. If a real
  external consumer appears, revisit whether the module seam is sufficient or a full split is
  warranted.

## Cross-references

Worked through the wayfinder map
[Bevy-free physics core: architecture + trigger decision](https://linear.app/linden-scientific/issue/DEV-1/bevy-free-physics-core-architecture-trigger-decision)
(Linear DEV-1) and its tickets:
[DEV-2](https://linear.app/linden-scientific/issue/DEV-2/mine-pr-42-for-concrete-crate-split-migration-costs) (PR #42 cost mining),
[DEV-3](https://linear.app/linden-scientific/issue/DEV-3/survey-ecosystem-patterns-for-debuginspector-access-to-non-ecs-physics) (telemetry ecosystem survey),
[DEV-4](https://linear.app/linden-scientific/issue/DEV-4/measureestimate-current-compile-time-cost-bevy-imposes-on-buoy-physics) (compile-time measurement),
[DEV-5](https://linear.app/linden-scientific/issue/DEV-5/decide-the-target-architecture-shape-for-separating-buoy-physics-from) (architecture-shape decision),
[DEV-6](https://linear.app/linden-scientific/issue/DEV-6/decide-the-concrete-trigger-condition-for-executing-the-split) (trigger-condition decision).
