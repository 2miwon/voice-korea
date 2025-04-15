use bdk::prelude::*;

#[component]
pub fn RadioButton(
    name: String,
    value: String,
    checked: bool,
    onchange: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        label { class: "flex items-center space-x-2 cursor-pointer relative",

            input {
                r#type: "radio",
                name,
                value,
                class: "peer hidden",
                checked,
                onchange: move |event| onchange.call(event),
            }

            span { class: "
                    w-20 h-20 rounded-full border-2
                    border-gray-400 peer-checked:border-blue-600
                    block transition-colors
                " }


            span { class: "
                    absolute w-12 h-12 rounded-full bg-white
                    left-4 top-6
                    opacity-0 peer-checked:opacity-100
                    peer-checked:bg-blue-600
                    transition-opacity
                " }

            span { "{value}" }
        }
    }
}
