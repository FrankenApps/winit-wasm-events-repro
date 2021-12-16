# Winit WASM Mouse repro

Mouse and Key input seems to be broken on the web except for Mouse Movement.

## Run natively
You should be able to run the project on **Linux**, **MacOS** and **Windows** using `cargo run --release`.

## Run on web
In order to build for `wasm32-unknown-unknown`:
1. Run `wasm-pack build --target web` ( you need a recent version of [`wasm-pack`](https://rustwasm.github.io/wasm-pack/))
2. Serve the root directory that contains the `Cargo.toml` with a static web server (for example with [`live-server`](https://www.npmjs.com/package/live-server))
3. Open `index.html` in your browser

Live version can be found here: https://frankenapps.github.io/winit-wasm-events-repro/index.html
