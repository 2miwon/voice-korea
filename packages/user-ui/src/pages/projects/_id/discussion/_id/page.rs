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
    let mut mic = use_signal(|| false);
    let mut video = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full h-lvh justify-start items-start",
            Header {
                title: "Debate topic", //FIXME: fix to real title
                onprev: move |_| async move {
                    ctrl.back().await;
                },
            }

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
                onprev: move |_| async move {
                    ctrl.back().await;
                },
            }
        }
    }
}
