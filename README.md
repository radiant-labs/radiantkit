<p align="center">
<a href="">
  <img src="banner.png" alt="RadiantKit Banner">
</a>
</p>

<h2 align="center">
    <b>Build Graphics Apps 10x Faster!</b> <br />
</h2>

# RadiantKit

RadiantKit is an in-development framework to build graphics applications (like Figma, Canva, Miro, etc) that's free and open source. 

It introduces a node-component-system for efficient rendering, while ensuring complete extensibility. It can support both native and web (via WebAssembly) platforms.

## Basic Example

Install rust and follow these steps:
1. `cd examples/basic`
2. `cargo run`

## Egui Integration Example

Install rust and follow these steps:
1. `cd examples/egui`
2. `cargo run`

## Web Examples

Install yarn and follow these steps:
1. `cd examples/web`
2. `yarn install`
3. `yarn build:wasm`
4. `yarn start`

## Tauri

Follow steps for web till #3. Then, run `yarn tauri dev`.

## Collaborative Editing

RadiantKit now supports real-time collaborative editing. Follow these steps to set it up:

Run the backend server:
1. `cd backend`
2. `cargo r`

Run egui app with a client id of 2:
1. `cd examples/egui`
2. `cargo r 2`

Run the whiteboard app (which runs with a default client id of 4):
1. `cd apps/whiteboard`
2. `yarn install`
3. `yarn start`
