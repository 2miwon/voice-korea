use bdk::prelude::*;
use by_components::icons::validations::Clear;

use crate::components::icons::Logo;

#[component]
pub fn Header(title: String, onprev: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-20 py-10 bg-netural-9",
            div { class: "flex flex-row w-fit h-fit justify-start items-center gap-5",
                Logo { width: "30", height: "20", class: "fill-third" }
                div { class: "font-extrabold text-xs/15 text-third", "VOICE KOREA" }
            }
            div { class: "font-semibold text-lg/22 text-white", {title} }
            button {
                onclick: move |e: Event<MouseData>| {
                    onprev.call(e);
                },
                Clear {
                    width: "18",
                    height: "18",
                    fill: "#7c8292",
                    class: "[&>path]:stroke-third",
                }
            }
        }
    }
}
