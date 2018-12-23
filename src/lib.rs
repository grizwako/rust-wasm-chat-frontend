use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window, WebSocket, console, FormData, HtmlFormElement, EventListener, EventTarget, Event};
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    init();
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    setup_form_handling(&document);
    Ok(())
}


fn setup_form_handling(document: &Document) {
    let form = document.get_element_by_id("chat-controls").expect("#chat-controls not found.");

    let handler = Box::new(move |event: Event| {
        event.prevent_default();
        let data = FormData::new_with_form(
            form.dyn_ref::<HtmlFormElement>().expect("#chat-controls is not HtmlFormElement")
        );

        let b = match data {
            Ok(form_data) => form_data.get("message"),
            Err(x) => x
        };
        console::log_1(&b);
    });
    let cb: Closure<Fn(Event)> = Closure::wrap(handler);

    document
        .get_element_by_id("chat-controls")
        .expect("should have #chat-controls on the page")
        .dyn_ref::<EventTarget>()
        .expect("#chat-controls must be an `EventTarget`")
        .add_event_listener_with_callback(
            "submit",
            cb.as_ref().unchecked_ref()
        ).expect("Could not add event listener");

    cb.forget();
}