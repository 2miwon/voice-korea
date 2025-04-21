use bdk::prelude::*;

#[component]
pub fn Checkbox(#[props(default = false)] disabled: bool, checked: bool) -> Element {
    rsx! {
        div {
            class: "cursor-pointer aria-disabled:cursor-not-allowed size-24 flex items-center justify-center rounded-md transition-all border border-gray-400 aria-checked:border-none",
            "aria-disabled": disabled,
            "aria-checked": checked,
            background_color: if disabled { "var(--color-disabled)".to_string() } else if checked { "var(--color-primary)".to_string() } else { "white".to_string() },
            div { class: "text-white text-lg",
                if checked {
                    div { "✔" }
                }
            }
        }
    }
}
