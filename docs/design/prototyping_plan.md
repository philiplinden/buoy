# Buoy — Prototyping Brief

A build order for the rewrite. Each piece is prototyped standalone, with a
pass/fail test that doesn't require any other piece to work. Nothing gets wired
into Bevy until it has passed its own test in isolation.

## Ground rules

**Prototype headless.** Every piece below should be testable as a plain Rust
function or a `cargo test`, with no Bevy app, no renderer, no ECS. Bevy comes
later, when the pieces are known-good and the only remaining question is
scheduling. The existing repo already has a CLI/headless split — keep it.

**One unknown at a time.** Each prototype holds everything else fixed. If the
shape solver is under test, altitude is constant and pressure is a hardcoded
number. This is the whole point of the ordering below.

**Write the test before the implementation.** Not for TDD's sake — because the
pass/fail criteria are where the physics is. If you can't state what "the shape
settled correctly" means numerically, you don't yet know what you're building.

**Salvage list.** `atmosphere.rs` and the gravity/constants code carry over
untouched. Everything else in `forces.rs` is coupled to avian and is being
replaced.

---

## Piece 1 — Membrane shape under fixed pressure

**Build first.** This is the piece most likely to fail, and everything
downstream assumes it works.

**Scope.** A ring of nodes on an axisymmetric generating curve. Hookean springs
between adjacent nodes. A constant outward pressure `ΔP` applied as a normal
force per node. Explicit integration with configurable substeps.

**Held fixed.** `ΔP` is a hardcoded constant. No gas law, no altitude, no
thermal, no flight. Gravity optional — try it both with and without.

**Inputs.** Node count, rest lengths, spring constant `k`, damping `c`, `ΔP`,
substep count, `dt`.

**Outputs.** Node positions over time. Derived: enclosed volume, cross-sectional
area.

**Passes when:**
- Starting from a perturbed shape, the nodes settle to a stable configuration
  and stay there. No drift, no slow oscillation that never decays.
- The settled shape is symmetric when the initial conditions are symmetric.
  Asymmetry here means a bug in the force assembly, not physics.
- Doubling `ΔP` at fixed `k` produces a visibly larger settled volume, and
  halving it produces a smaller one. Monotonic, in the right direction.
- Increasing `k` at fixed `ΔP` produces a stiffer, smaller response.
- It survives a stiffness sweep: hold `dt` fixed, raise `k` until it goes
  unstable. Record that threshold. You need to know where the cliff is before
  you build on top of this.

**Fails as:** nodes flying off to infinity, energy growing every step, or the
shape never settling. All three are the same underlying problem — the explicit
integrator can't handle the stiffness at your timestep.

**When it fails, in order:** raise substep count first, add damping second,
lower `k` third. Only if all three fail do you revisit the method. Do not reach
for XPBD or implicit integration until you have measured that the cheap fixes
are insufficient. Record the numbers when you do.

**Deliberately deferred.** Bending stiffness, anisotropy, tendons, wrinkling,
viscoelasticity, self-collision. All of these are real and none of them are
needed to answer "does the shape settle."

---

## Piece 2 — Gas state and the volume coupling

**Scope.** Lumped ideal gas: `P = nRT/V`. Volume comes from Piece 1's settled
shape. Pressure differential is `P_gas - P_ambient`, feeding back into Piece 1.

**Held fixed.** Temperature is a constant. Ambient pressure is a constant. No
thermal model, no altitude change yet.

**The actual question.** This piece exists to resolve the one non-acyclic edge
in the loop: volume sets pressure, pressure sets shape, shape sets volume. Test
both resolutions and pick on evidence.

- **Lagged:** compute pressure from last tick's volume. One step behind.
- **Iterated:** loop gas-state and shape 2–3 times within a tick until volume
  stops changing by more than some tolerance.

**Passes when:**
- The coupled system reaches an equilibrium volume and stays there.
- The lagged and iterated versions converge to the same equilibrium. If they
  don't, one of them is wrong.
- No pumping: volume shouldn't oscillate with growing amplitude. If it does,
  the lag is destabilising the loop and you need the iterated form.

**Record.** How many iterations the iterated form actually needs from a
warm start (previous shape as initial guess). If it's 1–2, iteration is cheap
and you should just always iterate. If it's 10+, take the lag.

**Deliberately deferred.** Venting, ballonets, gas leakage, superpressure limits.

---

## Piece 3 — Thermal

**Scope.** Lumped radiative balance on the envelope: direct solar, albedo,
ground IR, emitted IR, convective exchange with ambient. Output is film
temperature and gas temperature, which feed Piece 2.

**Held fixed.** Fixed altitude, fixed sun position, fixed shape. Vary only time
of day.

**Inputs.** Solar constant at altitude, absorptivity/emissivity/transmissivity
of the film, ground albedo and IR emissivity, ambient temperature and density,
surface area from the shape.

**Passes when:**
- Gas temperature converges to a steady value under constant illumination
  rather than running away or collapsing.
- Day/night cycling produces the expected qualitative behaviour: warming to a
  daytime plateau, cooling toward ambient at night, gas temperature lagging
  film temperature.
- Order of magnitude sanity: superheat (gas temp above ambient) in daylight
  lands in the tens of kelvin, not hundreds and not fractions.

**Deliberately deferred.** Spatial temperature distribution across the envelope,
internal convection, separate day/night film properties, cloud effects.

---

## Piece 4 — Orientation-dependent drag

**Scope.** `Cd` as a function of angle between body axis and relative velocity,
plus projected area computed directly from the generating curve rather than
from a collision mesh.

**Held fixed.** Rigid shape — use a frozen output from Piece 1. No deformation
during this test. No wind (relative velocity is just body velocity).

**Inputs.** Shape (as `r(s), z(s)`), body orientation, relative velocity vector,
ambient density, Reynolds number.

**Outputs.** Drag force vector. Projected area. Optionally lift/side force.

**Passes when:**
- Projected area computed from the generating curve matches an independent
  check (analytic for a sphere, numerical integration for anything else) to
  within a percent or so.
- Projected area varies smoothly with orientation. No discontinuities as the
  body rotates.
- `Cd` at α=0 matches the known value for the shape class.
- Drag opposes relative velocity and scales as `v²`.

**Reynolds note.** Include `Re` in the `Cd` lookup from the start, even if the
first implementation returns a constant. A HAB crosses the drag crisis on
ascent, and retrofitting `Re` dependence later means touching every call site.

**Deliberately deferred.** Torque and attitude dynamics. Getting a force vector
right is a separate problem from getting a moment right, and the moment needs a
strip-theory rework of the force assembly. Note where it would go; don't build
it.

---

## Piece 5 — Integration and scheduling

**Only after 1–4 pass individually.** This piece adds no new physics. It is
purely: own the timestep, own the phase ordering, write results into
`Transform`.

**Scope.** A fixed-timestep loop with explicit phase ordering — environment,
thermal, gas, shape, forces, integrate. In Bevy, `SystemSet`s in `FixedUpdate`
with `.chain()`.

**Passes when:**
- A full ascent runs to float altitude without blowing up.
- Float altitude is within a sensible range of a hand-calculated equilibrium
  (where buoyancy equals weight at the equilibrium volume).
- Results are reproducible run-to-run given the same inputs.
- Phase 3's substep count can be changed without changing the equilibrium the
  sim converges to — only how fast it gets there. If the answer depends on
  substep count, something upstream is wrong.

**Deliberately deferred.** Wind, editor UI, burst modelling, payload dynamics,
parachute descent.

---

## Validation targets

Once Piece 5 runs, check against published behaviour rather than intuition:

- **NASA BalloonAscent** (Farley, GSFC) — ascent and float profiles for
  spherical, zero-pressure natural shape, and superpressure classes. Closest
  published analogue to what this is doing.
- **Baginski et al.** — natural-shape and pumpkin equilibrium shapes. Compare
  your settled shape against the published generating curves for the same
  pressure and loading.
- **Real flight data** — ascent rate versus altitude from any documented HAB
  flight. Ascent rate is the easiest thing to get visibly wrong and the easiest
  to check.

## What this brief deliberately does not cover

Repo structure and module layout. The phase boundaries become the module
boundaries once the phases exist; deciding now means deciding twice.
