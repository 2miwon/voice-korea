use bdk::prelude::*;
use models::deliberation_project::{DeliberationProjectSummary, ProjectSorter};

use crate::pages::{
    components::project_card::ProjectCard,
    projects::{controller::Controller, i18n::ProjectListTranslate},
    search_project::SearchProject,
    sorter::Sorter,
};

#[component]
pub fn ProjectListPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;

    let projects = ctrl.projects()?.items;

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center",
            div { class: "max-w-desktop flex flex-col w-full justify-start items-start gap-20",
                div { class: "flex flex-row w-full justify-start items-start gap-15",
                    SearchProject {
                        lang,
                        onsearch: move |title: String| {
                            ctrl.search_keyword.set(title);
                        },
                    }

                    div { class: " w-full flex flex-row justify-end items-center",
                        Sorter {
                            id: "project_sorter_dropdown",
                            lang,
                            sorter: ctrl.sorter(),
                            on_sorter_changed: move |sorter: ProjectSorter| {
                                ctrl.sorter.set(sorter);
                            },
                        }
                    }
                }
                DeliberationList { lang, projects }
            }
        }
    }
}

#[component]
pub fn DeliberationList(lang: Language, projects: Vec<DeliberationProjectSummary>) -> Element {
    let tr: ProjectListTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-10",
            div { class: "flex flex-row w-full justify-start items-start font-semibold text-lg text-black",
                "{tr.project}"
            }

            div { class: "grid grid-cols-1 tablet:grid-cols-2 desktop:grid-cols-3 gap-20 w-full mt-30 [&>:nth-child(n+3)]:hidden tablet:[&>:nth-child(n+3)]:block tablet:[&>:nth-child(n+5)]:hidden desktop:[&>*]:!block",
                for deliberation in projects {
                    ProjectCard { lang, deliberation: deliberation.into() }
                }
            }
        }
    }
}
