use bdk::prelude::*;

use crate::components::form_field::InputEnterField;

#[component]
pub fn RoleEmailInputForm(
    #[props(default = 54)] height: i64,
    #[props(default = "inputfield".to_string())] name: String,
    label: String,
    placeholder: String,
    value: String,
    oninput: EventHandler<FormEvent>,
    onenter: EventHandler<KeyboardEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
            div { class: "w-[180px] mr-[50px] text-[#222222] font-medium text-[15px]",
                {label}
            }
            InputEnterField {
                height,
                placeholder,
                value,
                oninput,
                onenter,
            }
        }
    }
}
