use bdk::prelude::*;
use dioxus::document::Style;

#[component]
pub fn RichTextViewer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = "".to_string())] class: String,
    html: String,
    #[props(default = false)] contenteditable: bool,
) -> Element {
    rsx! {

        Style { href: "https://cdn.jsdelivr.net/npm/quill@2.0.0-dev.4/dist/quill.snow.css" }
        div { class: "flex flex-row w-full justify-start items-start ql-snow",
            div {
                class: format!("ql-editor {}", class),
                contenteditable,
                dangerous_inner_html: html,
                ..attributes,
            }
        }
    }
}
