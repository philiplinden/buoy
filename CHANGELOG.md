# Changelog

Notable changes to this project will be documented in this file.

## [unreleased]

### 🧪 Experimental

- Physics & Geometry update - janky but it works (#41) ([`7354de7`](https://github.com/philiplinden/buoy/commit/7354de7fbf79af0a58d1cde93f3144068bb4e2bc))

### ⚙️ Repository

- Remove big_space options (#39) ([`b03eff2`](https://github.com/philiplinden/buoy/commit/b03eff2fb9175f28a235db4cfe9b6da1b0aeb828))

## [0.2.0](https://github.com/philiplinden/buoy/compare/v0.1.0..v0.2.0) - 2025-06-27

### 🚀 Features

- Mesh tools and a simple scene (#37) ([`d9d7bc2`](https://github.com/philiplinden/buoy/commit/d9d7bc29ad77b49017b29eb6dc673a638bddac7a))
- Add egui side panel (#35) ([`1c5dff4`](https://github.com/philiplinden/buoy/commit/1c5dff46f16cf93c01641ddbb4625dc69aabc544))

### ♻️ Refactor

- Merge buoy-common with buoy-runtime (#38) ([`9e8fd7e`](https://github.com/philiplinden/buoy/commit/9e8fd7e4f29aae8f5b18840d4225965704e0b581))
- Run headless by default (#36) ([`012f179`](https://github.com/philiplinden/buoy/commit/012f179b78804c3cd393d2d4d304e9e55befb2f9))
- Buoy-core -> buoy-physics, add buoy-common, grid space is optional (#34) ([`a8e770e`](https://github.com/philiplinden/buoy/commit/a8e770e58c88a950f74c46a53fc069474c044607))
- Remove python experiments (#33) ([`089042b`](https://github.com/philiplinden/buoy/commit/089042b574e39ba263ec528affa14a5448073341))

### 🐛 Bug Fixes

- Disable mesh utils for now and focus on headless ([`db8ac1f`](https://github.com/philiplinden/buoy/commit/db8ac1f90a3a683c5d5f7902302ff1f66faa871a))

### ⚙️ Repository

- Refresh the changelog on pushes to main ([`26a2e6a`](https://github.com/philiplinden/buoy/commit/26a2e6a3f26df28b7d622901302cc12f579b98dc))
- Package the bevy-ui binary for releases ([`064e94c`](https://github.com/philiplinden/buoy/commit/064e94cd37ee1a161564b2b5c04f80533033742d))
- Include unreleased changes in log ([`c9fa447`](https://github.com/philiplinden/buoy/commit/c9fa4478d781a42c0e25d37669a5157b390f4195))

## [0.1.0] - 2025-03-02

### 📚 Documentation

- Update readme ([`5685be7`](https://github.com/philiplinden/buoy/commit/5685be7df46da343be3b831e7e5c15e024227a1c))
- More resources ([`696266b`](https://github.com/philiplinden/buoy/commit/696266b175e90f3e36d5de8494ef2f7977d41b43))

### ♻️ Refactor

- Disallow dead code in most places, add placeholders for payload ([`edb98d3`](https://github.com/philiplinden/buoy/commit/edb98d3a6aff3c3633de027930e3406e3fdd3bb0))

### 🎨 UI & Styling

- Improved plots ([`6d1bba1`](https://github.com/philiplinden/buoy/commit/6d1bba1862d9be682693fe6932148195a8db3ea7))
- Switch to egui, add plots (#15) ([`bdc6113`](https://github.com/philiplinden/buoy/commit/bdc6113a3d77993f56df5796358879371002e8f9))
- Deconflict debug text and console screen space ([`837a626`](https://github.com/philiplinden/buoy/commit/837a626c0a4781d22150d1fe3ea12e71519d5729))
- Add gizmos, flatten app module (#3) ([`119dade`](https://github.com/philiplinden/buoy/commit/119dade9ffb889d48ddcccb3381958bff7eae594))
- Add a gas monitor ([`3de7b41`](https://github.com/philiplinden/buoy/commit/3de7b4167d26622d516bd4cf9a11c5c584d724b0))
- Some debug ui practice ([`c80aa64`](https://github.com/philiplinden/buoy/commit/c80aa64d2bbd4cc2c1932889863bada2f8a1d0c2))
- Sim state monitoring, play/pause ([`88ce33b`](https://github.com/philiplinden/buoy/commit/88ce33b46f65d1e592057e51128b995d65eb34eb))
- Drop egui, add iyes_perf_ui ([`8cf8096`](https://github.com/philiplinden/buoy/commit/8cf8096625dd3ea4cfe98125a4f8d1d148eb5fbb))

### 🐛 Bug Fixes

- Fix up physics states ([`b311fe4`](https://github.com/philiplinden/buoy/commit/b311fe4d9e38f67e2c4fa5ee20b268da6797afe1))

### 🧪 Experimental

- Scaffold things in python for prototyping (#25) ([`45f8e98`](https://github.com/philiplinden/buoy/commit/45f8e98b571bff0a2bf8aac607f6745b0cc2743c))
- Smooth out artifacts ([`11ee325`](https://github.com/philiplinden/buoy/commit/11ee325c5d2f418f33b6db4e483ed94fb9f58709))
- Experimenting with plots and big space ([`8a57fb7`](https://github.com/philiplinden/buoy/commit/8a57fb7f7f6ce7151209f9f8f577bf3d248b7277))
- Experimenting with meshes and modules ([`2d75f55`](https://github.com/philiplinden/buoy/commit/2d75f5528cecac68c87851a3f5e1a4388741c1e1))
- Forces are stable! (#14) ([`5e0dbef`](https://github.com/philiplinden/buoy/commit/5e0dbefea4e4adf7eda26d73c3f7f9eb711020ad))

### ⚙️ Repository

- Yet another reorg - more submodules ([`2ac04de`](https://github.com/philiplinden/buoy/commit/2ac04de27df4b41f1223042e15e2c5a817b1bcb3))
- Skip deploys ([`2db5450`](https://github.com/philiplinden/buoy/commit/2db5450b77d736bbd545dab3b71a3681782d62f9))
- I guess docs need sound? ([`eb141a6`](https://github.com/philiplinden/buoy/commit/eb141a6a446042670815ea3cfb9a5598b7f5e036))
- Replace cache step with the one from leafwing ([`99642d9`](https://github.com/philiplinden/buoy/commit/99642d9cd84ff8eda7d29904e2fc7c22c3fd8f08))
- Add bevy deps ([`56ed0f3`](https://github.com/philiplinden/buoy/commit/56ed0f3d6c41e6f2b114d4cacfda22bd3e0f2a0a))


## New Contributors ❤️

* @philiplinden made their first contribution
<!-- generated by git-cliff -->
