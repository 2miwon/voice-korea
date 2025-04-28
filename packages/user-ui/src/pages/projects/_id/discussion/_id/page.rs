use bdk::prelude::*;

use crate::pages::projects::_id::discussion::_id::components::{Footer, Header, Video};
use crate::pages::projects::_id::discussion::_id::controller::Controller;
use crate::pages::{ConversationSidebar, ParticipantSidebar};

#[component]
pub fn DiscussionVideoPage(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    discussion_id: ReadOnlySignal<i64>,
) -> Element {
    let mut show_side_member = use_signal(|| false);
    let mut show_side_conversation = use_signal(|| false);
    let mut ctrl = Controller::init(lang, project_id, discussion_id)?;
    let mut mic = use_signal(|| true);
    let mut video = use_signal(|| true);

    let participants = ctrl.participants()?;

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

                        onchange_mic: move |m: bool| {
                            mic.set(m);
                        },
                        onchange_video: move |v: bool| {
                            video.set(v);
                        },
                        onchange_chat: move |_| {
                            if show_side_conversation() {
                                show_side_conversation.set(false);
                            } else {
                                show_side_conversation.set(true);
                                show_side_member.set(false);
                            }
                        },
                        onchange_member: move |_| {
                            if show_side_member() {
                                show_side_member.set(false);
                            } else {
                                show_side_member.set(true);
                                show_side_conversation.set(false);
                            }
                        },
                        onprev: move |_| async move {
                            ctrl.back().await;
                        },
                    }
                }

                ConversationSidebar {
                    messages: ctrl.chat_messages(),
                    show_conversation: show_side_conversation(),

                    hide_conversation: move |_| {
                        show_side_conversation.set(false);
                    },
                    onsend: move |text: String| {
                        tracing::debug!("message text: {:?}", text);
                        ctrl.send_message(text);
                    },
                }

                ParticipantSidebar {
                    show_member: show_side_member(),
                    hide_member: move |_| {
                        show_side_member.set(false);
                    },
                    onrefresh: move |_| {
                        ctrl.handle_refresh();
                    },

                    onselect: move |attendee_id: String| {
                        ctrl.handle_selecting_attendee(attendee_id);
                    },
                    participants: participants.participants,
                    users: participants.users,
                }
            }
        }
    }
}
