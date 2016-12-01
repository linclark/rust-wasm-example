# Rust/WASM example

This is a simple Rust function that can be run in the browser. It takes an integer
argument from JavaScript and returns an integer.

## Running this example

1. Turn on WASM in your browser.
2. Run:

    ```
    cd examples
    python -m SimpleHTTPServer
    ```

3. Open `http://localhost:8000/` and then open the console. You should see something like:

    ```
    trying binaryen method: native-wasm
    binaryen method succeeded.
    pre-main prep time: 3 ms
    Emscripten boilerplate loaded.
    result: 55
    ```

## Building

1. Follow the instructions for installing [Rust and Emscripten](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627)
2. Configure your PATH as Emscripten suggests.
3. Run:

    ```
    cd examples
    cargo build --target=wasm32-unknown-emscripten
    rustc --target=wasm32-unknown-emscripten ../src/lib.rs -L ../target/asmjs-unknown-emscripten/debug/deps/
    ```
