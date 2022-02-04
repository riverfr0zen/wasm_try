extern crate wasm_try;


fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    wasm_try::rect_eg_app();
}

