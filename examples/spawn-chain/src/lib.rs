#![deny(warnings)]
// `Alloc`
#![feature(allocator_api)]
#![feature(type_ascription)]

mod elixir;
mod start;

use lumen_rt_full as runtime;
use lumen_rt_full::process::spawn::options::Options;

use liblumen_web::wait;

use wasm_bindgen::prelude::*;

use crate::elixir::chain::{console_1, dom_1, none_1};
use crate::start::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
    initialize_dispatch_table();
    liblumen_web::start();
}

#[wasm_bindgen]
pub fn run(count: usize) -> js_sys::Promise {
    run_with_output(count, Output::None)
}

#[wasm_bindgen]
pub fn log_to_console(count: usize) -> js_sys::Promise {
    run_with_output(count, Output::Console)
}

#[wasm_bindgen]
pub fn log_to_dom(count: usize) -> js_sys::Promise {
    run_with_output(count, Output::Dom)
}

enum Output {
    None,
    Console,
    Dom,
}

fn run_with_output(count: usize, output: Output) -> js_sys::Promise {
    let mut options: Options = Default::default();
    options.min_heap_size = Some(79 + count * 10);

    wait::with_return_0::spawn(
        options,
    |child_process| {
        let count_term = child_process.integer(count)?;

        // if this fails use a bigger sized heap
        let frame = match output {
            Output::None => none_1::frame(),
            Output::Console => console_1::frame(),
            Output::Dom => dom_1::frame()
        };

        Ok(vec![frame.with_arguments(false, &[count_term])])
    })
    // if this fails use a bigger sized heap
    .unwrap()
}
