use crate::components::icons::{
    chat::Chat, end_circle::EndCircle, mic_off::MicOff, mic_on::MicOn, share::Share,
    video_off::VideoOff, video_on::VideoOn,
};
use bdk::prelude::*;
use by_components::icons::user::UserGroup;

#[component]
pub fn Footer(
    onprev: EventHandler<MouseEvent>,
    mic: bool,
    video: bool,
    record: bool,
    onchange_mic: EventHandler<bool>,
    onchange_video: EventHandler<bool>,
    onchange_share: EventHandler<MouseEvent>,
    onchange_member: EventHandler<MouseEvent>,
    onchange_record: EventHandler<MouseEvent>,
    onchange_chat: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-40 py-10 bg-netural-9",
            div { class: "flex flex-row w-fit justify-start items-center gap-10",
                BottomLabel {
                    onclick: move |_| {
                        onchange_mic.call(!mic);
                    },
                    title: "Audio",
                    if mic {
                        MicOn {}
                    } else {
                        MicOff {}
                    }
                }

                BottomLabel {
                    onclick: move |_| {
                        onchange_video.call(!video);
                    },
                    title: "Video",
                    if video {
                        VideoOn {}
                    } else {
                        VideoOff {}
                    }
                }
            }

            div { class: "flex flex-row w-fit justify-start items-center gap-10",
                BottomLabel {
                    onclick: move |e| {
                        onchange_member.call(e);
                    },
                    title: "Participants",
                    UserGroup {
                        width: "24",
                        height: "24",
                        fill: "#ffffff",
                        class: "[&>path]:stroke-white",
                    }
                }
                BottomLabel {
                    onclick: move |e| {
                        onchange_chat.call(e);
                    },
                    title: "Chat",
                    Chat { width: "24", height: "24" }
                }
                BottomLabel {
                    onclick: move |e| {
                        onchange_share.call(e);
                    },
                    title: "Share",
                    Share {}
                }
                        // BottomLabel {
            //     onclick: move |e| {
            //         onchange_record.call(e);
            //     },
            //     title: "Record",
            //     Record {}
            // }
            }

            BottomLabel {
                onclick: move |e| {
                    onprev.call(e);
                },
                title: "End",
                EndCircle {}
            }
        }
    }
}

#[component]
pub fn BottomLabel(title: String, onclick: EventHandler<MouseEvent>, children: Element) -> Element {
    rsx! {
        button {
            class: "flex flex-col w-fit justify-center items-center px-10 py-4 gap-4",
            onclick,
            {children}
            div { class: "font-semibold text-xs/15 text-white", {title} }
        }
    }
}
