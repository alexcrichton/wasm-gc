#![cfg_attr(feature = "wasm-bindgen", feature(proc_macro, wasm_custom_section, wasm_import_module))]
#[macro_use]
extern crate cfg_if;
extern crate wasm_gc;

pub const ERR_INVALID_MODULE: u32 = 1;
pub const ERR_OUTPUT_TOO_SMALL: u32 = 2;

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
pub struct WasmGcOptions {
    demangle: bool,
}

#[cfg_attr(feature = "wasm-bindgen", wasm_bindgen)]
impl WasmGcOptions {
    pub fn new() -> WasmGcOptions {
        WasmGcOptions { demangle: true }
    }

    pub fn demangle(&mut self, demangle: bool) {
        self.demangle = demangle;
    }

    pub fn gc(&self, input: &[u8], output: &mut [u8]) -> u32 {
        gc(input, output, self)
    }
}

cfg_if! {
    if #[cfg(feature = "wasm-bindgen")] {
        extern crate wasm_bindgen;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn wasm_gc(input: &[u8], output: &mut [u8]) -> u32 {
            gc(input, output, &WasmGcOptions::new())
        }
    } else {
        use std::slice;

        #[no_mangle]
        pub unsafe extern fn wasm_gc(
            input_ptr: *const u8,
            input_len: usize,
            output_ptr: *mut u8,
            output_len: usize,
        ) -> u32 {
            let opts = WasmGcOptions::new();
            wasm_gc_options(input_ptr, input_len, output_ptr, output_len, &opts)
        }

        #[no_mangle]
        pub unsafe extern fn wasm_gc_options(
            input_ptr: *const u8,
            input_len: usize,
            output_ptr: *mut u8,
            output_len: usize,
            options: *const WasmGcOptions,
        ) -> u32 {
            let input = slice::from_raw_parts(input_ptr, input_len);
            let output = slice::from_raw_parts_mut(output_ptr, output_len);
            gc(input, output, &*options)
        }

        #[no_mangle]
        pub extern fn wasm_gc_options_new() -> *mut WasmGcOptions {
            Box::into_raw(Box::new(WasmGcOptions::new()))
        }

        #[no_mangle]
        pub unsafe extern fn wasm_gc_options_demangle(
            opts: *mut WasmGcOptions,
            demangle: u32,
        ) {
            (*opts).demangle = demangle != 0;
        }

        #[no_mangle]
        pub unsafe extern fn wasm_gc_options_free(opts: *mut WasmGcOptions) {
            drop(Box::from_raw(opts));
        }
    }
}

fn gc(input: &[u8], output: &mut [u8], opts: &WasmGcOptions) -> u32 {
    let result = wasm_gc::Config::new()
        .demangle(opts.demangle)
        .gc(input);
    let result = match result {
        Ok(result) => result,
        Err(_) => return ERR_INVALID_MODULE,
    };
    match output.get_mut(..result.len()) {
        Some(buf) => buf.copy_from_slice(&result),
        None => return ERR_OUTPUT_TOO_SMALL,
    }
    return 0
}
