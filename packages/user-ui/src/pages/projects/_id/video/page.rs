use bdk::prelude::*;

use crate::pages::projects::_id::video::components::{Footer, Header, Video};
use crate::pages::projects::_id::video::controller::Controller;

#[component]
pub fn ProjectVideoPage(lang: Language, project_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = Controller::init(lang, project_id)?;
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
