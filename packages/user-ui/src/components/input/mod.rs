use dioxus::prelude::*;

#[component]
pub fn InputBox(
    #[props(default = None)] id: Option<String>,
    #[props(default = "flex flex-row w-full rounded-[10px] px-[15px] py-[10px] placeholder-[#b4b4b4] bg-[#f7f7f7] text-[#222222] outline-[#8095ea]".to_string())]
    class: String,
    width: Option<i64>,
    height: Option<i64>,
    placeholder: String,
    value: String,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        input {
            r#type: "text",
            id,
            class,
            width,
            height,
            placeholder,
            value,
            onchange: move |e| {
                onchange.call(e.value());
            },
        }
    }
}

#[component]
pub fn Input(
    #[props(default = None)] id: Option<String>,
    #[props(default = "".to_string())] class: String,
    #[props(default = "".to_string())] placeholder: String,
    value: String,
    onchange: EventHandler<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        input {
            r#type: "text",
            id,
            class: "px-27 py-17 bg-box-gray rounded-lg text-[18px]/24 border-none focus:outline-primary {class}",
            value,
            placeholder,
            onchange: move |e| {
                onchange.call(e.value());
            },
            ..attributes,
        }
    }
}
