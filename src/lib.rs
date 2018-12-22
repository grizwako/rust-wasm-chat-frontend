use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Global 'window' does not exist");
    let document = window.document().expect("window.document does not exist");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust :)");
    body.append_child(&val);

    Ok(())
}