use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window, WebSocket, console, FormData, HtmlFormElement, EventListener, EventTarget, Event};
use js_sys::Reflect;
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

    let ws = setup_ws_connection();
    setup_ws_msg_recv(ws.clone());
    setup_form_handling(&document, ws);
    Ok(())
}


fn setup_form_handling(document: &Document, ws: WebSocket) -> () {
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
    let cbx: Closure<Fn(Event)> = Closure::wrap(handler);

    document
        .get_element_by_id("chat-controls")
        .expect("should have #chat-controls on the page")
        .dyn_ref::<EventTarget>()
        .expect("#chat-controls must be an `EventTarget`")
        .add_event_listener_with_callback(
            "submit",
            cbx.as_ref().unchecked_ref()
        ).expect("Could not add event listener");

    cbx.forget();
}

fn setup_ws_connection() -> WebSocket {
    let ws = WebSocket::new_with_str("ws://localhost:2794", "rust-websocket")
        .expect("WebSocket failed to connect 'ws://localhost:2794'");


    let ws_c = ws.clone();
    let open_handler = Box::new(move || {
        console::log_1(&"Connection opened, sending 'test' to server".into());
        ws_c.send_with_str("test").expect("Failed to send 'test' to server");
    });
    let cb_oh: Closure<Fn()> = Closure::wrap(open_handler);
    ws.set_onopen(
        Some(cb_oh.as_ref().unchecked_ref() )
    );
    cb_oh.forget();
    ws
}

fn setup_ws_msg_recv(ws: WebSocket) -> () {
    let msg_recv_handler = Box::new(move |msg:JsValue| {
        console::log_1(
            &Reflect::get(&msg, &"data".into())
            .expect("ws msg has no 'data'")
        );
    });
    let cb_mrh: Closure<Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    ws.set_onmessage(
        Some(cb_mrh.as_ref().unchecked_ref() )
    );

    cb_mrh.forget();
}