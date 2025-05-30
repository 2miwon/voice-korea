use dioxus::prelude::*;

#[component]
pub fn VideoOff(
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
            path {
                clip_rule: "evenodd",
                d: "M7.28578 6H6C4.34315 6 3 7.34315 3 9V15C3 16.6569 4.34315 18 6 18H13C14.6302 18 15.9567 16.6998 15.999 15.0798L17.7506 16.4811C17.7737 16.4996 17.797 16.5175 17.8205 16.5347L14 12.7142V15C14 15.5523 13.5523 16 13 16H6C5.44772 16 5 15.5523 5 15V9C5 8.44772 5.44772 8 6 8H9.28578L7.28578 6ZM14 9.88579V9C14 8.44772 13.5523 8 13 8H12.1142L10.1142 6H13C14.6302 6 15.9567 7.30024 15.999 8.92021L17.7506 7.51889C19.0601 6.47127 21 7.40361 21 9.08062V14.9194C21 15.4871 20.7777 15.9695 20.4328 16.3186L19 14.8858V9.08063L16 11.4806V11.8858L14 9.88579Z",
                fill: "#7C8292",
                fill_rule: "evenodd",
            }
            path {
                d: "M4 4L20 20",
                stroke: "#EB5757",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
            }
        }
    }
}
