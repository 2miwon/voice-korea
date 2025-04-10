#![allow(unused)]
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
                    TriangleDown {}
                } else {
                    TriangleUp {}
                }
            }

            if expand() {
                {children}
            }
        
        }
    }
}
