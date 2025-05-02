use dioxus::prelude::*;

#[component]
pub fn Record(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            class,
            fill,
            height,
            view_box: "0 0 24 24",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                fill: "#8095EA",
                height: "7",
                rx: "3.5",
                stroke: "#8095EA",
                stroke_width: "2",
                width: "7",
                x: "8.5",
                y: "8.5",
            }
            rect {
                height: "16",
                rx: "8",
                stroke: "#8095EA",
                stroke_width: "2",
                width: "16",
                x: "4",
                y: "4",
            }
        }
    }
}
