# buoy-runtime

Runs the simulation and handles spawning objects.

This crate handles the Bevy application lifecycle, including spawning objects
and running the simulation. It also handles the application state, including
starting, stopping, and faulting the simulation.

## Features

Runs headless by default. Enable `gui` to enable `buoy-ui`. Enable `debug-ui` to
enable debug gizmos and `bevy-inspector-egui`.
