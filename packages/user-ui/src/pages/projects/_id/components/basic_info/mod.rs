mod controllers;
mod i18n;

use bdk::prelude::*;
use by_components::icons::upload_download::Download2;

use crate::{
    pages::projects::_id::components::{accordion::Accordion, rich_text_viewer::RichTextViewer},
    utils::time::formatted_timestamp,
};
use models::tab::Tab;

use controllers::Controller;
use i18n::BasicInfoTranslate;

#[component]
pub fn BasicInfo(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let basic_info = ctrl.basic_info()?;

    let tr: BasicInfoTranslate = translate(&lang);
    let tab_title: &str = Tab::BasicInfo.translate(&lang);

    rsx! {
        div {
            id: "basic-info",
            class: "max-[1000px]:px-30 flex flex-col w-full h-fit bg-box-gray gap-20",
            ..attributes,
            // header
            div { class: "w-full flex max-[500px]:flex-col max-[500px]:items-start max-[500px]:justify-start max-[500px]:gap-5 flex-row justify-between items-center mt-28",
                div { class: " font-semibold text-xl", "{tab_title}" }
                div { class: "font-medium text-[15px] text-black",
                    {
                        format!(
                            "{} ~ {}",
                            formatted_timestamp(lang, basic_info.started_at),
                            formatted_timestamp(lang, basic_info.ended_at),
                        )
                    }
                }
            }
            // information section
            div { class: "flex flex-col gap-10",
                Accordion { title: tr.main_title, default_open: true,
                    div { class: "w-full justify-start mt-15 mb-20 font-bold text-lg",
                        "{basic_info.title}"
                    }
                    RichTextViewer {
                        class: "w-full flex justify-start text-[15px]",
                        contenteditable: false,
                        html: basic_info.description,
                    }

                // FIXME: fix to query by members field
                // div { class: "w-full mt-20 flex flex-row justify-start gap-40",
                //     for member in basic_info.members {
                //         div { class: "flex flex-row justify-start gap-8",
                //             img { class: "w-40 h-40 bg-profile-gray rounded-full" }
                //             div { class: "flex flex-col justify-center",
                //                 p { class: "font-semibold text-[15px] justify-start",
                //                     {member.role.translate(&lang)}
                //                 }
                //             }
                //         }
                //     }
                // }
                }

                //Related Data
                div { class: "w-full flex flex-col rounded-lg mb-40 bg-white justify-start items-center py-14 px-20",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-13",
                        div { class: "w-180 flex flex-row items-center text-base font-bold",
                            span { "{tr.related_materials_title}" }
                        }
                        //file
                        div { class: "flex flex-wrap flex-1 justify-start items-center gap-8",
                            for resource in basic_info.resources {
                                div {
                                    class: "cursor-pointer flex flex-row justify-start items-center rounded-[100px] bg-light-gray gap-4 px-12 py-4",
                                    onclick: {
                                        let files = resource.files.clone();
                                        move |_| {
                                            let files = files.clone();
                                            async move {
                                                for file in files.clone() {
                                                    let name = file.name;
                                                    let link = file.url;
                                                    ctrl.download_file(name, link).await;
                                                }
                                            }
                                        }
                                    },
                                    Download2 {
                                        width: "18",
                                        height: "18",
                                        class: " [&>path]:fill-white",
                                    }
                                    div { class: "font-medium text-sm text-white", {resource.title} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
