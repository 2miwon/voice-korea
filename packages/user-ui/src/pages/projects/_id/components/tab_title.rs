use bdk::prelude::*;

#[component]
pub fn TabTitle(
    title: String,
    children: Element,
    #[props(default = "".to_string())] class: String,
) -> Element {
    rsx! {
        div { class: "w-full flex flex-row mt-28 mb-10 justify-between items-center {class} ",
            span { class: "font-semibold text-[20px]", {title} }
            {children}
        }
    }
}
