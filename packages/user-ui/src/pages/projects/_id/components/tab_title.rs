use bdk::prelude::*;
use by_components::icons::arrows::ChevronLeft;

#[component]
fn Wrapper(children: Element, #[props(default = "".to_string())] class: String) -> Element {
    rsx! {
        div { class: "w-full flex flex-row min-h-40 justify-between items-center {class} ",
            {children}
        }
    }
}
#[component]
pub fn TabTitle(
    #[props(default = "".to_string())] class: String,
    title: String,
    children: Element,
) -> Element {
    rsx! {
        Wrapper { class,
            span { class: "font-semibold text-[20px]", {title} }
            {children}
        }
    }
}

#[component]
pub fn TabTitleWithPrev(
    #[props(default = "".to_string())] class: String,
    title: String,
    children: Element,
    onprev: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        Wrapper { class,
            div { class: "flex flex-row justify-start items-center gap-8",
                button {
                    class: "w-[24px] h-[24px]",
                    onclick: move |e: Event<MouseData>| {
                        onprev.call(e);
                    },
                    ChevronLeft { class: "[&>stroke]:stroke-black" }
                }
                div { class: "font-semibold text-text-black text-[20px]", {title} }
            }
            {children}
        }
    }
}
