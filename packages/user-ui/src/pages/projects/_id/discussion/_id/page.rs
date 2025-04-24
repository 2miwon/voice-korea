use bdk::prelude::*;

use crate::pages::projects::_id::discussion::_id::components::{Footer, Header, Video};
use crate::pages::projects::_id::discussion::_id::controller::Controller;

#[component]
pub fn DiscussionVideoPage(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    discussion_id: ReadOnlySignal<i64>,
) -> Element {
    let ctrl = Controller::init(lang, project_id, discussion_id)?;
    rsx! {
        div { class: "flex flex-col w-full h-lvh justify-start items-start",
            Header {
                title: "Debate topic",
                onprev: move |_| {
                    ctrl.back();
                },
            }

            Video {}

            Footer {
                onprev: move |_| {
                    ctrl.back();
                },
            }
        }
    }
}
