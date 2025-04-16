use dioxus::prelude::*;
use dioxus_logger::tracing;

#[component]
pub fn CustomCheckbox(
    #[props(default = false)] disabled: bool,
    mut checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        label { class: "flex items-center cursor-pointer disabled:cursor-not-allowed",
            input {
                disabled,
                r#type: "checkbox",
                class: "hidden peer",
                checked: "{checked}",
                onchange: move |_| {
                    tracing::debug!("Checkbox changed to {}", ! checked);
                    onchange.call(!checked);
                },
            }
            div { class: "w-[24px] h-[24px] flex items-center justify-center rounded-md transition-all border bg-white border-gray-400 peer-disabled:bg-color-disabled peer-checked:bg-primary peer-checked:border-none",
                div { class: "text-white text-lg",
                    if checked {
                        div { "✔" }
                    }
                }
            }
        }
    }
}
