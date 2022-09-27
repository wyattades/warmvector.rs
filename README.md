# WarmVector

Shooting bad guys, randomly generated levels, destructable terrain, what's not to love?

The [original WarmVector](https://github.com/wyattades/WarmVector_Client_Singleplayer) was created with my 2D Java game engine, which mainly served as a learning experience while I experimented with Java multi-threading and OpenGL.

This new version is rewritten in Rust and built with the Bevy game engine; another learning opportunity!

<!-- ![](https://i.imgur.com/nGcNT4U.gif)   -->

## Dev Setup

Install the following:

- [rustup](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- `node.js` v16 and `yarn` v2

## Compilation

You have a few options:

- Build for web development: `yarn dev`
- Build for web production: `yarn build`
- Build for desktop development: `cargo run`
- Build for desktop production: `cargo build --release`

## Known Bugs

- TBD

## Future Feature Ideas

- Create more weapon/bullet types, e.g. rockets, explosives, magic??
- Continue developing level difficulty progression
- Add random boxes to middle of rooms (with enough space to move around them!)
