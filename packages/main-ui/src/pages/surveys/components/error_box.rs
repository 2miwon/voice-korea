use bdk::prelude::*;
use by_components::icons::validations::Warning;

#[component]
pub fn ErrorBox(hidden: bool, title: String, description: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full justify-start items-center p-16 gap-20 border border-necessary bg-[#eb57570f] rounded-lg aria-active:hidden",
            "aria-active": hidden,
            Warning { class: "[&>path]:stroke-necessary [&>circle]:stroke-necessary" }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-bold text-[15px] text-necessary leading-26", {title} }
                div { class: "font-normal text-sm text-necessary leading-17", {description} }
            }
        }
    }
}
