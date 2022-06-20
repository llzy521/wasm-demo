mod utils;

use wasm_bindgen::prelude::*;
use std::io::Write;
use std::fs;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, webassembly-rust!");
    let mut file = fs::File::create(
        "/Users/leolu/project/demo/webassembly-demo/webassembly-web/helloworld.txt",
    ).unwrap();
    file.write_all(b"Hello, world!").unwrap();
}

#[test]
fn a(){

}