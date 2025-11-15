# Chartrs

Chartrs is aiming to be a pure-Rust plotting library with a concise dependency tree. Uses `raqote` as the rendering backend, optionally uses `rusttype` for fonts, and `minifb` to display plots interactively.

## Capabilities

- [x] Plot to window
- [x] Plot to PNG
- [x] Scatter plot
- [ ] Line plot
- [ ] Bar plot
- [ ] Histogram


## Cargo Features

- `text`: Enables rendering text. Enabled by default.
- `interactive`: Enables interactive plots with `minifb`. If using Wayland or X11, they should further be specified. Enabled by default.
  - `wayland`: Enables `minifb` Wayland-specific features: `wayland` and `dlopen`.
  - `x11`: Enables the `x11` feature in `minifb`. Enabled by default.
- `png`: Enables `raqote`'s `png` feature for PNG graph export.
- `unstable`: Enables unstable features in relevant crates.

## Dependencies
### Core
- `raqote`: 2D rendering library (https://crates.io/crates/raqote)
- `euclid`: Geometry primitives used by `raqote` and Chartrs (https://crates.io/crates/euclid)
### Interactive Plotting
- `minifb`: A cross-platform window setup library (https://crates.io/crates/minifb)
### Text Support
- `rusttype`: Rust font loading library (https://crates.io/crates/rusttype)
