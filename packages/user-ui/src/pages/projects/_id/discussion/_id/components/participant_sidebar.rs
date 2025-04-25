use bdk::prelude::*;
use by_components::icons::validations::Clear;
use models::{discussion_participants::DiscussionParticipant, UserSummary};

use crate::components::icons::{refresh::Refresh, Logo};

#[component]
pub fn ParticipantSidebar(
    show_member: bool,
    participants: Vec<DiscussionParticipant>,
    users: Vec<UserSummary>,

    hide_member: EventHandler<MouseEvent>,
    refresh: EventHandler<MouseEvent>,
    clicked_attendee: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "fixed top-0 right-0 h-full w-[320px] bg-white shadow-lg z-50 transition-transform duration-500 transform aria-active:translate-x-0 translate-x-full rounded-l-lg",
            "aria-active": show_member,
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center px-20 py-18 bg-netural-9",
                    div { class: "flex flex-row w-fit justify-start items-center gap-10",
                        Logo { width: "30", height: "20", class: "fill-third" }
                        div { class: "font-semibold text-white text-sm/17", "Participants" }
                        button {
                            onclick: move |e: Event<MouseData>| {
                                refresh.call(e);
                            },
                            Refresh {
                                width: "12",
                                height: "12",
                                fill: "#bfc8d9",
                                class: "[&>path]:stroke-discussion-border-gray",
                            }
                        }
                    }
                    button {
                        onclick: move |e: Event<MouseData>| {
                            hide_member.call(e);
                        },
                        Clear {
                            width: "24",
                            height: "24",
                            fill: "#bfc8d9",
                            class: "[&>path]:stroke-discussion-border-gray",
                        }
                    }
                }
                div { class: "flex flex-col w-full h-lvh justify-start items-start px-10 py-20 bg-key-gray gap-20",
                    for (i , user) in users.iter().enumerate() {
                        button {
                            class: "flex flex-row w-full justify-start items-center gap-4",
                            onclick: {
                                let participant_id = participants[i].participant_id.clone();
                                move |_| {
                                    clicked_attendee.call(participant_id.clone());
                                }
                            },
                            div { class: "flex flex-row w-30 h-30 justify-center items-center rounded-full bg-text-gray",
                                Logo {
                                    width: "21",
                                    height: "12",
                                    class: "fill-white",
                                }
                            }

                            div { class: "font-medium text-white text-sm/18", {user.email.clone()} }
                        }
                    }
                }
            }
        }
    }
}
