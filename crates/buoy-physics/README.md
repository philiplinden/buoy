# buoy-physics

This is the core simulation engine for buoyancy simulator, buoy. It contains the
core physics calculations for buoyancy, drag, weight, ideal gas law, and basic
elasticity.

It also runs the simulation itself: the Bevy application lifecycle, spawning
objects, and the running/stopped/faulted simulation state. The `buoy` binary
in this crate runs the simulation headless; `buoy-ui` layers a GUI on top of
the same `BuoyDefaultPlugins`/`BuoyPhysicsPlugin` plugins.
