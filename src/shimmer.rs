#[cfg(target_arch = "wasm32")]
use web_sys::console::log_1;

#[cfg(not(target_arch = "wasm32"))]
pub fn log(s: String) {
    println!("{}", s);
}

#[cfg(target_arch = "wasm32")]
pub fn log(s: String) {
    log_1(&s.into());
}
