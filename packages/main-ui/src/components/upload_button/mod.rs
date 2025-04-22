#![allow(unused_variables)]
#![allow(unused_mut)]
use dioxus::prelude::*;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[component]
pub fn UploadButton(
    #[props(default = "file-upload".to_string())] id: String,
    class: String,
    text: String,
    onuploaded: EventHandler<FormEvent>,
    #[props(default = "image/*".to_string())] accept: String,
    #[props(default = false)] multiple: bool,
) -> Element {
    rsx! {
        input {
            id: id.clone(),
            class: "hidden",
            r#type: "file",
            accept,
            multiple,
            onchange: {
                let id = id.clone();
                move |ev| {
                    onuploaded.call(ev);
                    #[cfg(feature = "web")]
                    {
                        let id = id.clone();
                        spawn(async move {
                            use gloo_timers::future::TimeoutFuture;
                            TimeoutFuture::new(0).await;
                            let input = web_sys::window()
                                .unwrap()
                                .document()
                                .unwrap()
                                .get_element_by_id(&id.clone())
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap();
                            input.set_value("");
                        });
                    }
                }
            },
        }
        button {
            class,
            onclick: {
                let id = id.clone();
                move |_| {
                    #[cfg(feature = "web")]
                    {
                        let input = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .get_element_by_id(&id.clone())
                            .unwrap();
                        input.dyn_ref::<web_sys::HtmlInputElement>().unwrap().click();
                    }
                }
            },
            {text}
        }
    }
}
