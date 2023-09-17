# Radiant

Radiant is an in-development framework to build graphics applications (like Figma, Canva, Miro, etc) that's free and open source. 

It introduces a node-component-system for efficient rendering, while ensuring complete extensibility. It can support both native and web (via WebAssembly) platforms.

## POC Desktop App

Install rust and follow these steps:
1. `cd app`
2. `cargo run`

## Web

Install yarn and follow these steps:
1. `cd web`
2. `yarn install`
3. `yarn build:wasm`
4. `yarn start`

## Tauri

Follow steps for web till #3. Then, run `yarn tauri dev`.
