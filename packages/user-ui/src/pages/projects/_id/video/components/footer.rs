use crate::components::icons::{
    end_circle::EndCircle, mic_off::MicOff, mic_on::MicOn, video_off::VideoOff, video_on::VideoOn,
};
use bdk::prelude::*;

#[component]
pub fn Footer(onprev: EventHandler<MouseEvent>) -> Element {
    let mut mic = use_signal(|| false);
    let mut video = use_signal(|| false);
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-40 py-10 bg-netural-9",
            div { class: "flex flex-row w-fit justify-start items-center gap-10",
                button {
                    class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
                    onclick: move |_| {
                        mic.set(!mic());
                    },
                    if mic() {
                        MicOn {}
                    } else {
                        MicOff {}
                    }
                    div { class: "font-semibold text-xs/15 text-white", "Audio" }
                }

                button {
                    class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
                    onclick: move |_| {
                        video.set(!video());
                    },
                    if video() {
                        VideoOn {}
                    } else {
                        VideoOff {}
                    }
                    div { class: "font-semibold text-xs/15 text-white", "Video" }
                }
            }

            button {
                class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
                onclick: move |e| {
                    onprev.call(e);
                },
                EndCircle {}
                div { class: "font-semibold text-xs/15 text-white", "End" }
            }
        }
    }
}
