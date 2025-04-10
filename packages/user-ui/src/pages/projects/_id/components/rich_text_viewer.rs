use bdk::prelude::*;
use dioxus::document::Style;

#[component]
pub fn RichTextViewer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    class: String,
    html: String,
    #[props(default = false)] contenteditable: bool,
) -> Element {
    rsx! {

        Style { href: "https://cdn.jsdelivr.net/npm/quill@2.0.0-dev.4/dist/quill.snow.css" }
        div {
            class: "{class}",
            contenteditable,
            dangerous_inner_html: format!("{}", html.replace("\n", "<br>")),
            ..attributes,
        
        }
    }
}
