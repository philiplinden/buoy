# buoy

A buoyancy simulation environment built with [Bevy](https://bevyengine.org/) in
Rust.

## Structure

The repository is structured as follows:

- `crates/buoy-runtime`: Runtime code. This is what dispatches the simulation loop, sets up the physics, UI, renderer.
- `crates/buoy-physics`: Physics simulation code.
- `crates/buoy-ui`: UI code.

## License

Except where noted (below and/or in individual files), all code in this
repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This
dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to
include both.
