use bdk::prelude::*;

#[component]
pub fn InputField(
    #[props(default = 54)] height: i64,
    #[props(default = "inputfield".to_string())] name: String,
    placeholder: String,
    value: String,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full focus:outline-none justify-start items-center bg-background-gray rounded-[4px] h-{height}",
            div { class: "flex px-15 w-full",
                input {
                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                    r#type: "text",
                    name,
                    placeholder,
                    value,
                    oninput,
                }
            }
        }
    }
}
