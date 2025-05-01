use bdk::prelude::*;
use models::{DeliberationFinalSurveyCreateRequest, Question};

use crate::{
    routes::Route,
    utils::time::{current_timestamp_with_time, parsed_timestamp_with_time},
};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    final_survey: Signal<DeliberationFinalSurveyCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub committee_members: Signal<Vec<String>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let final_survey = use_signal(|| DeliberationFinalSurveyCreateRequest::default());

        let mut ctrl = Self {
            lang,
            final_survey,
            parent: use_context(),
            nav: use_navigator(),

            committee_members: use_signal(|| vec![]),
        };

        use_effect({
            let req = ctrl.parent.deliberation_requests();
            let mut final_survey = req
                .final_surveys
                .get(0)
                .unwrap_or(&DeliberationFinalSurveyCreateRequest::default())
                .clone();

            move || {
                let committees = req.roles.iter().map(|v| v.email.clone()).collect();
                let started_at = final_survey.clone().started_at;
                let ended_at = final_survey.clone().ended_at;

                if started_at == 0 {
                    final_survey.started_at = current_timestamp_with_time(0, 0, 0);
                }

                if ended_at == 0 {
                    final_survey.ended_at = current_timestamp_with_time(23, 59, 59);
                }

                ctrl.final_survey.set(final_survey.clone());
                ctrl.committee_members.set(committees);
            }
        });
        Ok(ctrl)
    }

    pub fn set_title(&mut self, title: String) {
        self.final_survey.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.final_survey.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.final_survey.with_mut(|req| {
            req.started_at = parsed_timestamp_with_time(started_at, 0, 0, 0);
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.final_survey.with_mut(|req| {
            req.ended_at = parsed_timestamp_with_time(ended_at, 23, 59, 59);
        });
    }

    pub fn set_estimate_time(&mut self, estimate_time: i64) {
        self.final_survey.with_mut(|req| {
            req.estimate_time = estimate_time;
        });
    }

    pub fn set_point(&mut self, point: i64) {
        self.final_survey.with_mut(|req| {
            req.point = point;
        });
    }

    pub fn add_committee(&mut self, email: String) {
        self.final_survey.with_mut(|req| {
            req.users.push(email);
        });
    }

    pub fn remove_committee(&mut self, email: String) {
        self.final_survey.with_mut(|req| {
            req.users.retain(|e| !(e.clone() == email));
        })
    }

    pub fn clear_committee(&mut self) {
        self.final_survey.with_mut(|req| req.users = vec![]);
    }

    pub fn add_question(&mut self) {
        self.final_survey.with_mut(|req| {
            req.surveys.push(Question::default());
        });
    }

    pub fn remove_question(&mut self, index: usize) {
        self.final_survey.with_mut(|req| {
            req.surveys.remove(index);
        });
    }

    pub fn update_question(&mut self, index: usize, question: Question) {
        self.final_survey.with_mut(|req| {
            req.surveys[index] = question;
        });
    }

    pub fn get_final_survey(&self) -> DeliberationFinalSurveyCreateRequest {
        (self.final_survey)()
    }

    pub fn get_selected_committee(&self) -> Vec<String> {
        let final_survey = self.get_final_survey();
        let roles = final_survey.clone().users;

        roles
    }

    pub fn back(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.nav
            .replace(Route::DeliberationDiscussionSettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_final_survey(self.final_survey());
        self.parent.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.parent.save_final_survey(self.final_survey());
            self.nav.push(Route::Preview { lang: self.lang });
        }
    }

    pub fn is_valid(&self) -> bool {
        let final_survey = self.final_survey();

        let title = final_survey.title;
        let description = final_survey.description;
        let started_at = final_survey.started_at;
        let ended_at = final_survey.ended_at;

        let members = final_survey.users;
        let surveys = final_survey.surveys;

        !(title.is_empty()
            || description.is_empty()
            || started_at >= ended_at
            || members.is_empty()
            || surveys.is_empty())
    }

    pub fn validation_check(&self) -> bool {
        let final_survey = self.final_survey();

        let title = final_survey.title;
        let description = final_survey.description;
        let started_at = final_survey.started_at;
        let ended_at = final_survey.ended_at;

        let members = final_survey.users;
        let surveys = final_survey.surveys;

        if title.is_empty() {
            btracing::e!(self.lang, ValidationError::TitleRequired);
            return false;
        }
        if description.is_empty() {
            btracing::e!(self.lang, ValidationError::DescriptionRequired);
            return false;
        }
        if started_at >= ended_at {
            btracing::e!(self.lang, ValidationError::TimeValidationFailed);
            return false;
        }
        if members.is_empty() {
            btracing::e!(self.lang, ValidationError::MemberRequired);
            return false;
        }
        if surveys.is_empty() {
            btracing::e!(self.lang, ValidationError::SurveyRequired);
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "최종 설문 제목을 입력해주세요.",
        en = "Please enter the final survey title."
    )]
    TitleRequired,
    #[translate(
        ko = "최종 설문 설명을 입력해주세요.",
        en = "Please enter the final survey description."
    )]
    DescriptionRequired,
    #[translate(
        ko = "시작 날짜는 종료 날짜보다 작아야합니다.",
        en = "The start date must be less than the end date."
    )]
    TimeValidationFailed,
    #[translate(
        ko = "1명 이상의 담당자를 선택해주세요.",
        en = "Please select one or more contact persons."
    )]
    MemberRequired,
    #[translate(
        ko = "한 문항 이상의 설문을 입력해주세요.",
        en = "Please enter one or more questions in the survey."
    )]
    SurveyRequired,
}
