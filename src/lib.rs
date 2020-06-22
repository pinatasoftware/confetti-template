extern crate cfg_if;
extern crate confetti;
extern crate wasm_bindgen;

mod middleware;
mod router;
mod utils;

use cfg_if::cfg_if;
use router::routes;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn confetti(request: JsValue) -> Result<JsValue, JsValue> {
    let res = confetti::run(request, routes()).await;
    Ok(res)
}
