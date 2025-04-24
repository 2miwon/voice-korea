use dioxus::prelude::*;

#[component]
pub fn MicOn(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
) -> Element {
    rsx! {
        div {
            svg {
                class,
                fill,
                height,
                view_box: "0 0 24 24",
                width,
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M12 17V21M12 21H9M12 21H15",
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                }
                rect {
                    height: "10",
                    rx: "2",
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    width: "4",
                    x: "10",
                    y: "3",
                }
                path {
                    d: "M17.7378 12.7542C17.3674 13.9659 16.6228 15.0293 15.6109 15.7918C14.599 16.5544 13.3716 16.977 12.1047 16.9991C10.8378 17.0212 9.59647 16.6417 8.55854 15.9149C7.52061 15.1881 6.73941 14.1515 6.32689 12.9534",
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                }
            }
        }
    }
}
