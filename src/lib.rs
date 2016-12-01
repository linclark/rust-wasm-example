#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_rust_function']"]
#[link_args = "-s DEMANGLE_SUPPORT=1"]
extern {}

#[no_mangle]
pub fn rust_function(n:i32) -> i32 {
    // Without this line, WASM files will not show up in Chrome's debugger.
    println!("logging from Rust!");

    n + 42
}

fn main() {
    /* Intentionally left blank */
}