# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [Unreleased]

### Bug Fixes

- Fix up physics states
- Clean up after the ai

### Build

- Make some tweaks so it is easier to work with bevy cli

### CFG

- Rearchitecture the configs and reading from toml
- Fix toml parsing

### CI

- Add bevy deps
- Replace cache step with the one from leafwing
- I guess docs need sound?
- Skip deploys
- Update changelog triggers and build opts
- Fix cl formatting

### CLI

- Set up simulation cli

### Chore

- Update to bevy 0.15.1 and avian3d 0.2.0

### DEV

- Total rewrite, this time with docs

### Documentation

- Devlog for 2024-11-07
- Rm anims
- Corrections in the devlog
- Remove everything ([#9](https://github.com/philiplinden/buoy/issues/9))
- More placeholders

### REPO

- Sketch out the repo structure with ai placeholders

### Refactor

- Split into multiple crates

### Aero

- Placeholder drag from mesh
- Checkpoint

### Balloon

- Nice

### Bevy

- Round 1
- Round 2
- Round 3
- Round 4
- Round 5
- Round 6 - this is broken
- Round 7 - fix the ui, gut everything else
- Migrate atmo to use position, balloon to use components
- Start migrating dynamics to avian
- Structured thermodynamics
- Add minor windows for help dialogs
- A little beautification
- A little more separation of concerns
- Trying to make volume work
- Split thermodynamics and forces into components
- Clean
- Get working on bevy 0.15.0-rc.3
- Reintroduce bevy trait query on 0.15 branch

### Buoy_common

- Move configs from buoy_sim to buoy_common

### Buoy_sim

- Remove broken examples
- Clippy fix
- Remove fancy logging

### Camera

- Basic orbit cam, experiments with picking
- Orbit camera is finally working
- Smoother camera movement

### Checkpoint

- Feeling out bevy implementations
- 2024-11-23 morning
- 2024-11-23 afternoon
- Trying to debug volume change over time

### Cleanup

- Disallow dead code in most places, add placeholders for payload

### Core

- Remove redundant system set
- Make units not components, ease in to meshes
- Consolidate mass properties
- Fix the physics engine integration

### Crates

- Buoy_app is redundant to buoy_sim

### Dev

- Forces are stable! ([#14](https://github.com/philiplinden/buoy/issues/14))
- Experimenting with meshes and modules
- Experimenting with plots and big space
- Smooth out artifacts
- Scaffold things in python for prototyping ([#25](https://github.com/philiplinden/buoy/issues/25))
- Start devving a server-client arch, use best practices for bevy 16
- Actually run headless

### Doc

- More resources
- Update readme

### Forces

- Drop bevy-trait-query dependency
- Pseudo-components for forces
- ITS WORKING

### Gizmos

- Cleaner force arrows

### Grid

- Spawn arrays of cells with atmospheric properties ([#21](https://github.com/philiplinden/buoy/issues/21))

### Lint

- Clippy

### Mod

- Remove the assets module for now

### Physics

- Ball hits the ground
- Buoyancy and weight are working
- Impl buoyancy and weight as externalforces
- Restructure force systems
- Restructure force systems
- Start drag calc module
- Drag
- Force refactor
- Forces are reliable yay
- Fix drag force (placeholder)
- Fix drag, volume, density, and buoyancy

### Python

- Add plot option

### Repo

- Yet another reorg - more submodules
- Broken but closer to the real deal

### Ui

- Drop egui, add iyes_perf_ui
- Sim state monitoring, play/pause
- Some debug ui practice
- Add a gas monitor
- Add gizmos, flatten app module ([#3](https://github.com/philiplinden/buoy/issues/3))
- Deconflict debug text and console screen space
- Switch to egui, add plots ([#15](https://github.com/philiplinden/buoy/issues/15))
- Improved plots

[Unreleased]: https://github.com/philiplinden/buoy/compare/HEAD...HEAD

