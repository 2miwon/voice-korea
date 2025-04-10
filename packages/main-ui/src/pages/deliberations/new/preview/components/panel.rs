use crate::routes::Route;
use crate::{
    components::updatable_card::UpdatableCard,
    pages::deliberations::new::preview::i18n::PreviewTranslate,
};
use bdk::prelude::*;
use models::PanelV2Summary;

#[component]
pub fn Panel(lang: Language, selected_panels: Vec<PanelV2Summary>) -> Element {
    let tr: PreviewTranslate = translate(&lang);
    rsx! {
        UpdatableCard {
            lang,
            enable_line: true,
            title: tr.composition_panel,
            route: Route::CompositionPanel { lang },
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-start items-center min-h-55 gap-50",
                    div { class: "flex flex-row w-180 h-fit font-medium text-[15px] text-text-black",
                        {tr.total_panels}
                    }
                    div { class: "flex flex-wrap w-full flex-1 justify-start iems-center gap-10",
                        for panel in selected_panels.clone() {
                            BlackLabel { text: panel.name }
                        }
                    }
                }

                for panel in selected_panels {
                    div { class: "flex flex-row w-full justify-start items-center min-h-55 gap-50",
                        div { class: "flex flex-row w-180 h-fit font-medium text-[15px] text-text-black",
                            {format!("{} {}", panel.name, tr.attribute)}
                        }
                        div { class: "flex flex-wrap w-full flex-1 justify-start iems-center gap-10",
                            for attribute in panel.attributes {
                                div {
                                    match attribute {
                                        models::response::Attribute::Age(age_v3) => {
                                            rsx! {
                                                BlackLabel { text: age_v3.translate(&lang) }
                                            }
                                        }
                                        models::response::Attribute::Gender(gender_v2) => {
                                            rsx! {
                                                BlackLabel { text: gender_v2.translate(&lang) }
                                            }
                                        }
                                        models::response::Attribute::Region(region_v2) => {
                                            rsx! {
                                                BlackLabel { text: region_v2.translate(&lang) }
                                            }
                                        }
                                        models::response::Attribute::Salary(salary_v2) => {
                                            rsx! {
                                                BlackLabel { text: salary_v2.translate(&lang) }
                                            }
                                        }
                                        models::response::Attribute::None => {
                                            rsx! {}
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
}

#[component]
pub fn BlackLabel(text: String) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit h-fit px-8 py-3 bg-label-black rounded-sm font-semibold text-sm text-white leading-18",
            {text}
        }
    }
}
