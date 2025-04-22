use bdk::prelude::*;
use by_components::icons::upload_download::Download2;

use crate::{
    components::AvatarLabel,
    pages::projects::_id::components::{
        accordion::Accordion, rich_text_viewer::RichTextViewer, section::Section,
        tab_title::TabTitle,
    },
    utils::time::formatted_timestamp,
};
use models::{deliberation_role::DeliberationRole, tab::Tab};

use super::controllers::Controller;
use super::i18n::BasicInfoTranslate;

#[component]
pub fn BasicInfo(lang: Language, project_id: ReadOnlySignal<i64>, children: Element) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let basic_info = ctrl.basic_info()?;

    let tr: BasicInfoTranslate = translate(&lang);
    let tab_title: &str = Tab::BasicInfo.translate(&lang);

    rsx! {
        Section { id: "basic-info",
            TabTitle { title: tab_title,
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
                    div { class: "w-full flex flex-col gap-20",
                        div { class: "font-bold text-lg", {basic_info.title} }
                        RichTextViewer {
                            contenteditable: false,
                            html: basic_info.description,
                        }
                        div { class: "w-full flex flex-row justify-start gap-20",
                            for member in basic_info.members {
                                AvatarLabel {
                                    label: member.nickname.unwrap_or(member.email),
                                    //FIXME: use organization name
                                    sub: "DAO",
                                }
                            }
                        }
                    }
                }

                Accordion { title: tr.committee_title,
                    div { class: "w-full flex flex-col gap-12 [&>:last-child]:hidden",
                        for (title , members) in ctrl.commitees() {
                            Committee { lang, title, members }
                        }
                    }
                }
                //Related Data
                div { class: "w-full flex flex-col rounded-lg mb-40 bg-white justify-start items-center py-14 px-20",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-13",
                        div { class: "w-180 flex flex-row items-center text-base font-bold",
                            span { {tr.related_materials_title} }
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
                                    div { class: "font-medium text-sm text-white line-clamp-1",
                                        {resource.title}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Committee(lang: Language, title: String, members: Vec<DeliberationRole>) -> Element {
    rsx! {
        div { class: "w-full flex flex-col gap-10",
            h1 { class: "text-18 font-bold w-full", {title} }
            div { class: "grid grid-cols-5 gap-20 w-full",
                for member in members {

                    AvatarLabel { label: member.email, sub: "DAO" }
                }
            }
        }
        hr { class: "w-full border-b-1 border-line-gray" }
    }
}
