use crate::routes::Route;

use super::*;
use bdk::prelude::*;
use by_components::loaders::cube_loader::CubeLoader;

#[component]
pub fn DeliberationDetailSettingLayout(lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;

    rsx! {
        div {
            id: "deliberation-detail-setting-layout",
            class: "flex flex-col w-full gap-20",

            div { class: "w-full flex flex-row",
                for n in DeliberationDetailSettingStep::VARIANTS {
                    button {
                        class: "border-b-[1px] !px-20 !py-10 min-w-150 aria-active:!border-b-[5px] aria-active:!border-primary hover:!border-b-[5px] hover:!border-primary/10",
                        "aria-active": ctrl.current_step == *n,
                        //FIXME: remove to this comment when save issue is resolved.
                        onclick: move |_| {},
                        {n.translate(&lang)}
                    }
                }
            }

            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "w-full h-full flex flex-col justify-center items-center", CubeLoader {} }
                },
                Outlet::<Route> {}
            }
        } // end of this page
    }
}
