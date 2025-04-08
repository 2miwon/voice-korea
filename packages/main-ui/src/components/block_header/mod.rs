use bdk::prelude::*;

#[component]
pub fn BlockHeader(
    required: bool,
    header: String,
    description: String,
    onopen: Option<EventHandler<bool>>,
) -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        div { class: "flex flex-row w-full justify-between",
            div { class: "flex flex-col w-full gap-2",
                div { class: "flex flex-row w-full justify-start items-start",
                    if required {
                        div { class: "text-base font-bold text-necessary mb-5 mr-2",
                            "*"
                        }
                    }
                    div { class: "text-lg font-bold text-text-black", "{header}" }
                }
                div { class: "text-sm font-medium text-text-gray", "{description}" }
            }
            if let Some(onclick) = onopen {
                div {
                    class: "flex flex-row justify-center items-center",
                    onclick: move |_| {
                        open.set(!open());
                        onclick.call(open());
                    },
                    if open() {
                        div { class: "text-sm font-medium text-text-gray", "▼" }
                    } else {
                        div { class: "text-sm font-medium text-text-gray", "▲" }
                    }
                }
            }
        }
    }
}
