use bdk::prelude::*;

#[component]
pub fn Section(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = "".to_string())] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full h-fit bg-box-gray mt-28 mb-40 gap-20 {class}",
            {children}
        }
    }
}
