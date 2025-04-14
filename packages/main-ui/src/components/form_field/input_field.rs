use bdk::prelude::*;

#[component]
pub fn InputField(
    height: Option<i64>,
    name: Option<String>,
    placeholder: String,
    value: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let h = height.unwrap_or(54);
    rsx! {
        div { class: "flex flex-row w-full focus:outline-none justify-start items-center bg-background-gray rounded-[4px] h-{h}",
            div { class: "flex px-15 w-full",
                input {
                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                    r#type: "text",
                    name: name.unwrap_or("input".to_string()),
                    placeholder,
                    value,
                    oninput,
                }
            }
        }
    }
}

#[component]
pub fn TextAreaField(
    height: Option<i64>,
    name: Option<String>,
    placeholder: String,
    value: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    let h = height.unwrap_or(248);
    rsx! {
        div { class: "flex flex-row w-full focus:outline-none justify-start items-start bg-background-gray rounded-[4px] h-{h}",
            div { class: "flex px-15 py-10 w-full h-full justify-start items-start",
                textarea {
                    class: "flex w-full h-full justify-start items-start bg-transparent focus:outline-none my-10 break-words whitespace-normal",
                    placeholder,
                    name: name.unwrap_or("textarea".to_string()),
                    value,
                    oninput,
                }
            }
        }
    }
}
