use crate::{
    components::updatable_card::UpdatableCard,
    pages::deliberations::new::preview::i18n::PreviewTranslate, routes::Route,
};
pub use bdk::prelude::*;
use by_components::icons::edit::Edit1;
use models::{
    DeliberationBasicInfoCreateRequest, DeliberationContentCreateRequest,
    DeliberationDiscussionCreateRequest, DeliberationFinalSurveyCreateRequest,
    DeliberationSampleSurveyCreateRequest, OrganizationMemberSummary,
};

use crate::utils::time::format_range_from_timestamp;

#[component]
pub fn Procedure(
    lang: Language,
    basic_info: DeliberationBasicInfoCreateRequest,
    sample_survey: DeliberationSampleSurveyCreateRequest,
    deliberation: DeliberationContentCreateRequest,
    discussion: DeliberationDiscussionCreateRequest,
    final_survey: DeliberationFinalSurveyCreateRequest,

    basic_info_members: Vec<OrganizationMemberSummary>,
    sample_survey_members: Vec<OrganizationMemberSummary>,
    deliberation_members: Vec<OrganizationMemberSummary>,
    discussion_members: Vec<OrganizationMemberSummary>,
    final_survey_members: Vec<OrganizationMemberSummary>,
) -> Element {
    let tr: PreviewTranslate = translate(&lang);
    rsx! {
        UpdatableCard {
            lang,
            enable_line: false,
            title: tr.setting_deliberation_procedure,
            route: Route::DeliberationBasicInfoSettingPage {
                lang,
            },
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                DeliberationDetailComponent {
                    lang,
                    title: tr.basic_info,
                    start_date: basic_info.started_at,
                    end_date: basic_info.ended_at,
                    members: basic_info_members,
                    route: Route::DeliberationBasicInfoSettingPage {
                        lang,
                    },
                }
                DeliberationDetailComponent {
                    lang,
                    title: tr.sample_survey,
                    start_date: sample_survey.started_at,
                    end_date: sample_survey.ended_at,
                    members: sample_survey_members,
                    route: Route::DeliberationSampleSurveySettingPage {
                        lang,
                    },
                }
                DeliberationDetailComponent {
                    lang,
                    title: tr.deliberation,
                    start_date: deliberation.started_at,
                    end_date: deliberation.ended_at,
                    members: deliberation_members,
                    route: Route::DeliberationSettingPage {
                        lang,
                    },
                }
                DeliberationDetailComponent {
                    lang,
                    title: tr.discussion,
                    start_date: discussion.started_at,
                    end_date: discussion.ended_at,
                    members: discussion_members,
                    route: Route::DeliberationDiscussionSettingPage {
                        lang,
                    },
                }
                DeliberationDetailComponent {
                    lang,
                    title: tr.vote,
                    start_date: final_survey.started_at,
                    end_date: final_survey.ended_at,
                    members: final_survey_members,
                    route: Route::DeliberationVoteSettingPage {
                        lang,
                    },
                }
            }
        }
    }
}

#[component]
pub fn DeliberationDetailComponent(
    lang: Language,
    title: String,
    start_date: i64,
    end_date: i64,
    members: Vec<OrganizationMemberSummary>,
    route: Route,
) -> Element {
    let tr: PreviewTranslate = translate(&lang);
    let nav = use_navigator();
    rsx! {
        div { class: "flex flex-col w-full h-fit justify-start items-start p-16 bg-white rounded-sm border border-[#ededed] gap-4",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "font-bold text-[15px] text-text-black leading-26", {title} }
                div {
                    class: "cursor-pointer",
                    onclick: move |_| {
                        nav.push(route.clone());
                    },
                    Edit1 {}
                }
            }
            div { class: "font-normal text-xs text-[#6d6d6d] leading-16",
                {format_range_from_timestamp(start_date, end_date)}
            }
            div { class: "flex flex-row w-full justify-start items-center gap-10",
                div { class: "font-normal text-xs text-[#6d6d6d] leading-16", {tr.manager} }
                div { class: "flex flex-wrap flex-1 w-full justify-start items-center gap-10",
                    for member in members {
                        div { class: "flex flex-row w-fit justify-start items-center gap-4",
                            div { class: "w-15 h-15 rounded-full bg-[#d9d9d9]" }
                            div { class: "font-semibold text-xs text-[#6d6d6d] leading-16",
                                {member.name}
                            }
                        }
                    }
                }
            }
        }
    }
}
