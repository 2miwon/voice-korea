use super::InputField;
use crate::components::select_category::SelectCategory;
use bdk::prelude::*;

#[component]
pub fn SelectInputField(
    #[props(default = 183)] width: i64,
    #[props(default = 54)] height: i64,
    #[props(default = "inputfield".to_string())] name: String,
    selected_field: Option<String>,
    select_placeholder: String,
    placeholder: String,
    text_value: String,
    onchange: EventHandler<Event<FormData>>,
    oninput: EventHandler<FormEvent>,
    options: Vec<String>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center gap-20",
            SelectCategory {
                width,
                height,
                selected_field,
                placeholder: select_placeholder,
                onchange,
                options,
            }
            InputField {
                height,
                name,
                placeholder,
                value: text_value,
                oninput: move |e: FormEvent| {
                    oninput.call(e);
                },
            }
        }
    }
}
