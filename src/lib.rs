use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, Window, WebSocket, console, FormData, HtmlFormElement, EventListener, EventTarget, Event};
use js_sys::Reflect;
extern crate console_error_panic_hook;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub user: String,
    pub text: String 
}


#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    init();
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let parent = document.get_element_by_id("chat-display")
        .expect("No #chat-display");
    let ws = setup_ws_connection();
    let template = document.create_element("div")?;
    setup_ws_msg_recv(ws.clone(), parent, template);
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

        if let Some(msg) = message_from_form(data) {
            ws.send_with_str(
                &serde_json::to_string(&msg).expect("Serde could not serialize struct")
            ).expect("Could not send message");
        }
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

    fn message_from_form(form_data: Result<FormData, JsValue>) -> Option<Message> {
        match form_data {
            Ok(form_data) => Some (
                Message {
                    user: form_data.get("username").as_string().expect("could not read username from form"),
                    text: form_data.get("message").as_string().expect("could not read message from form")
                }
            ),
            Err(_) => None
        }
    }
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

fn setup_ws_msg_recv(ws: WebSocket, msgContainer : Element, templateNode: Element) -> () {
    let msg_recv_handler = Box::new(move |msg:JsValue| {
        let data: JsValue = Reflect::get(&msg, &"data".into())
            .expect("No 'data' field in websocket message!");

        let message: Message = serde_json::from_str(
                &data.as_string().expect("Field 'data' is not string")
            ).expect("Serde could not decode Message");

        let val = templateNode.clone_node().expect("Could not clone template node");
        let text = format!("{} says: {}", message.user, message.text);
        val.set_text_content(Some(&text));
        msgContainer.append_child(&val).expect("Could not append message node to container");
    });
    let cb_mrh: Closure<Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    ws.set_onmessage(
        Some(cb_mrh.as_ref().unchecked_ref() )
    );

    cb_mrh.forget();
}