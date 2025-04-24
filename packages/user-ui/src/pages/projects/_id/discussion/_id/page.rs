use bdk::prelude::*;

use crate::pages::projects::_id::discussion::_id::components::{Footer, Header, Video};
use crate::pages::projects::_id::discussion::_id::controller::Controller;
use crate::pages::ParticipantSidebar;

#[component]
pub fn DiscussionVideoPage(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    discussion_id: ReadOnlySignal<i64>,
) -> Element {
    let mut show_side_member = use_signal(|| false);
    let ctrl = Controller::init(lang, project_id, discussion_id)?;
    let mut mic = use_signal(|| true);
    let mut video = use_signal(|| true);

    rsx! {
        div { class: "relative flex flex-col w-full h-lvh justify-start items-start",
            Header {
                title: "Debate topic", //FIXME: fix to real title
                onprev: move |_| async move {
                    ctrl.back().await;
                },
            }

            div { class: "flex flex-row w-full h-full justify-start items-start",
                div { class: "flex flex-1 flex-col w-full h-full justify-start items-start",
                    Video { video: video() }

                    Footer {
                        mic: mic(),
                        video: video(),

                        change_mic: move |m: bool| {
                            mic.set(m);
                        },
                        change_video: move |v: bool| {
                            video.set(v);
                        },
                        change_show_member: move |_| {
                            show_side_member.set(!show_side_member());
                        },
                        onprev: move |_| async move {
                            ctrl.back().await;
                        },
                    }
                }

                //FIXME: fix to real email
                ParticipantSidebar {
                    show_member: show_side_member(),
                    hide_member: move |_| {
                        show_side_member.set(false);
                    },
                    emails: vec![
                        "aaa@bbb.ccc".to_string(),
                        "aaa1@bbb.ccc".to_string(),
                        "aaa2@bbb.ccc".to_string(),
                        "aaa3@bbb.ccc".to_string(),
                        "aaa4@bbb.ccc".to_string(),
                    ],
                }
            }
        }
    }
}
