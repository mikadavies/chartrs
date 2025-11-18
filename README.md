# Chartrs

[![Minimum Supported Rust Version]][Rust 1.93.0-nightly]

Chartrs is aiming to be a pure-Rust plotting library with a concise dependency tree. Uses `raqote` as the rendering backend, optionally uses `rusttype` for fonts, and `minifb` to display plots interactively.

By making the `raqote` backend visible, Chartrs should be highly customisable.

This is a small hobby project for now, and is only in its very early phases. For the time being, it will only cover simple plotting tasks. Contributions are welcome.

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

## Minimum Supported Rust Version
- **MSRV: v1.93.0-nightly**
> [!NOTE]
> Chartrs has mainly been developed using Rust Nightly, and has only been verified on that channel for now. It likely works on the latest stable rust as well, as it does not currently use any Nightly-specific features.
