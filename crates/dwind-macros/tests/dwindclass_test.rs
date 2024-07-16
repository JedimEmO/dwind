use dominator::html;
use dominator_css_bindgen_test::*;
use dwind_macros::dwclass;
use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test::wasm_bindgen_test]
fn basic_dwind_macro_test() {
    let _ = html!("div", {
        .dwclass!("foo")
    });
}
