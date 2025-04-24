use crate::components::icons::{
    end_circle::EndCircle, mic_off::MicOff, mic_on::MicOn, share::Share, video_off::VideoOff,
    video_on::VideoOn,
};
use bdk::prelude::*;
use web_sys::js_sys::eval;

#[component]
pub fn Footer(
    onprev: EventHandler<MouseEvent>,
    mic: bool,
    video: bool,
    change_mic: EventHandler<bool>,
    change_video: EventHandler<bool>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-40 py-10 bg-netural-9",
            div { class: "flex flex-row w-fit justify-start items-center gap-10",
                button {
                    class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
                    onclick: move |_| {
                        let _ = eval(
                            r#"
                                                if (window._toggleAudio) {
                                                    window._toggleAudio();
                                                }
                                            "#,
                        );
                        change_mic.call(!mic);
                    },
                    if mic {
                        MicOn {}
                    } else {
                        MicOff {}
                    }
                    div { class: "font-semibold text-xs/15 text-white", "Audio" }
                }

                button {
                    class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
                    onclick: move |_| {
                        let _ = eval(
                            r#"
                                                if (window._toggleVideo) {
                                                    window._toggleVideo();
                                                }
                                            "#,
                        );
                        change_video.call(!video);
                    },
                    if video {
                        VideoOn {}
                    } else {
                        VideoOff {}
                    }
                    div { class: "font-semibold text-xs/15 text-white", "Video" }
                }
            }

            div { class: "flex flex-row w-fit justify-start items-center",
                button {
                    class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
                    onclick: move |_| {
                        let _ = eval(
                            r#"
                                                if (window._toggleShared) {
                                                    window._toggleShared();
                                                }
                                            "#,
                        );
                    },
                    Share {
                    }
                    div { class: "font-semibold text-xs/15 text-white", "Share" }
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
