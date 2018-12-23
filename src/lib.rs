use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window, WebSocket, console, FormData, HtmlFormElement};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");


    setup_form_handling(&document);

    // And now that our demo is ready to go let's switch things up so
    // everything is displayed and our loading prompt is hidden.
    document
        .get_element_by_id("loading")
        .expect("should have #loading on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#loading should be an `HtmlElement`")
        .style()
        .set_property("display", "none")?;
    document
        .get_element_by_id("script")
        .expect("should have #script on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#script should be an `HtmlElement`")
        .style()
        .set_property("display", "block")?;

    Ok(())
}


// We also want to count the number of times that our green square has been
// clicked. Our callback will update the `#num-clicks` div.
//
// This is pretty similar above, but showing how closures can also implement
// `FnMut()`.
fn setup_form_handling(document: &Document) {

    let form = document.get_element_by_id("chat-controls").expect("#chat-controls not found.");
    let username = document.get_element_by_id("username").expect("#username not found.");
    let message = document.get_element_by_id("message").expect("#message not found.");
    let f2 = form.dyn_ref::<HtmlFormElement>().expect("#chat-controls is not HtmlFormElement");
    let data = FormData::new_with_form(f2);
    let b = match data {
        Ok(form_data) => form_data.get("message"),
        Err(x) => x
    };
    console::log_1(&b);


    let handler = Closure::wrap(Box::new(move || {
        
        console::log_1(&"adsfsadf".into());

    }) as Box<dyn FnMut()>);

    let num_clicks = document
        .get_element_by_id("num-clicks")
        .expect("should have #num-clicks on the page");
    let mut clicks = 0;
    let a = Closure::wrap(Box::new(move || {
        clicks += 1;
        num_clicks.set_inner_html(&clicks.to_string());
    }) as Box<dyn FnMut()>);
    document
        .get_element_by_id("green-square")
        .expect("should have #green-square on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#green-square be an `HtmlElement`")
        .set_onclick(Some(a.as_ref().unchecked_ref()));
    a.forget();
}