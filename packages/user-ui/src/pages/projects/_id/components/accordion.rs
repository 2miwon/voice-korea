use bdk::prelude::*;

use crate::components::icons::triangle::{TriangleDown, TriangleUp};

#[component]
pub fn Accordion(
    title: String,
    #[props(default = String::default())] class: String,
    #[props(default = VNode::empty())] children: Element,
    #[props(default = false)] default_open: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut expand = use_signal(|| default_open);
    rsx! {
        div {
            class: "w-full flex flex-col rounded-lg bg-white justify-start items-center py-14 px-20 {class}",
            ..attributes,


            div {
                class: "w-full flex justify-between items-center text-base font-bold cursor-pointer",
                onclick: move |_| {
                    let prev = expand();
                    expand.set(!prev);
                },
                span { {title} }
                if expand() {
                    TriangleUp {}
                } else {
                    TriangleDown {}
                }
            }

            if expand() {
                hr { class: "w-full h-1 mt-12 mb-12 border-line-gray" }
                {children}
            }
        
        }
    }
}
