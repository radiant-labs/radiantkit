<h1 align="center">
  Radiant
</h1>

<p align="center">
    <b>Build Graphics Apps 10x Faster!</b> <br />
</p>

<h3 align="center">
  <a href="https://radiant-labs.github.io/radiant/">Website</a> &bull;
  <a href="https://radiant-labs.gitbook.io/radiant/">Docs</a> &bull;
  <a href="https://join.slack.com/t/radiant-canvas/shared_invite/zt-25isowtr6-jg3wHcQjRuLxyeT_fELO9Q">Community</a>
</h3>

# Radiant

Radiant is an in-development framework to build graphics applications (like Figma, Canva, Miro, etc) that's free and open source. 

It introduces a node-component-system for efficient rendering, while ensuring complete extensibility. It can support both native and web (via WebAssembly) platforms.

## Basic Example

Install rust and follow these steps:
1. `cd examples/basic`
2. `cargo run`

## Egui Integration Example

Install rust and follow these steps:
1. `cd examples/egui`
2. `cargo run`

## Web

Install yarn and follow these steps:
1. `cd examples/web`
2. `yarn install`
3. `yarn build:wasm`
4. `yarn start`

## Tauri

Follow steps for web till #3. Then, run `yarn tauri dev`.
