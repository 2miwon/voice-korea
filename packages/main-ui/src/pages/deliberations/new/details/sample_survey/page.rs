use super::super::components::{AssignMember, IntroductionCard, Reward};
use super::*;
use crate::pages::deliberations::new::{
    components::footer_buttons::FooterButtons,
    details::sample_survey::components::question::QuestionList,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
use models::Question;

#[component]
pub fn DeliberationSampleSurveySettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: SampleSurveyTranslate = translate(&lang);
    let sample_survey = ctrl.get_sample_survey();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col w-full justify-start items-start gap-20",
                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-medium text-base text-text-black", {tr.input_introduction} }
                    div { class: "flex flex-col w-full justify-start items-start gap-20",
                        IntroductionCard {
                            lang,
                            start_date_id: "sample_survey_start_date",
                            end_date_id: "sample_survey_end_date",
                            description: tr.introduction_description.to_string(),
                            text_value: sample_survey.clone().title,
                            started_at: sample_survey.clone().started_at,
                            ended_at: sample_survey.clone().ended_at,
                            content: sample_survey.clone().description,
                            set_title: move |title: String| {
                                ctrl.set_title(title);
                            },
                            set_description: move |description: String| {
                                ctrl.set_description(description);
                            },
                            set_start_date: move |timestamp: i64| {
                                ctrl.set_start_date(timestamp);
                            },
                            set_end_date: move |timestamp: i64| {
                                ctrl.set_end_date(timestamp);
                            },
                        }
                        Reward {
                            lang,
                            point: sample_survey.clone().point,
                            estimate_time: sample_survey.clone().estimate_time,
                            set_estimate_time: move |estimate_time: i64| {
                                ctrl.set_estimate_time(estimate_time);
                            },
                            set_point: move |point: i64| {
                                ctrl.set_point(point);
                            },
                        }

                        AssignMember {
                            lang,
                            committees: ctrl.get_committees(),
                            selected_committees: ctrl.get_selected_committee(),
                            add_committee: move |id: i64| {
                                ctrl.add_committee(id);
                            },
                            remove_committee: move |id: i64| {
                                ctrl.remove_committee(id);
                            },
                            clear_committee: move |_| {
                                ctrl.clear_committee();
                            },
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-medium text-base text-text-black", {tr.voting_items} }
                    QuestionList {
                        lang,

                        sample_survey: ctrl.get_sample_survey(),

                        add_question: move |_| {
                            ctrl.add_question();
                        },
                        remove_question: move |index: usize| {
                            ctrl.remove_question(index);
                        },
                        update_question: move |(index, question): (usize, Question)| {
                            ctrl.update_question(index, question);
                        },
                    }
                }
            }
            FooterButtons {
                lang,
                on_backward: move |_| {
                    ctrl.back();
                },
                on_temp_save: move |_| async move { ctrl.temp_save().await },
                on_next: move |_| {
                    ctrl.next();
                },
                on_save: None,
                next_valid: true,
            }
        }
    }
}
