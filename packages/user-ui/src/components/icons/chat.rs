use dioxus::prelude::*;

#[component]
pub fn Chat(
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
                view_box: "0 0 25 24",
                width,
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M3.62891 6.83203C3.62891 4.62289 5.41977 2.83203 7.62891 2.83203H17.6289C19.838 2.83203 21.6289 4.62289 21.6289 6.83203V14.832C21.6289 17.0412 19.838 18.832 17.6289 18.832H14.8789L13.4289 20.7654C13.0289 21.2987 12.2289 21.2987 11.8289 20.7654L10.3789 18.832H7.62891C5.41977 18.832 3.62891 17.0412 3.62891 14.832V6.83203Z",
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                }
                line {
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_width: "2",
                    x1: "9.12891",
                    x2: "16.1289",
                    y1: "7.5625",
                    y2: "7.5625",
                }
                path {
                    d: "M8.87891 11.0664L13.8789 11.0664",
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_width: "2",
                }
                path {
                    d: "M8.87891 14.3828H10.8789",
                    stroke: "white",
                    stroke_linecap: "round",
                    stroke_width: "2",
                }
            }
        }
    }
}
