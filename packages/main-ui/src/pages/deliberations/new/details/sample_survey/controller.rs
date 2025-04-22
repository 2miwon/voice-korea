use bdk::prelude::*;

use models::{
    deliberation_sample_surveys::deliberation_sample_survey::DeliberationSampleSurveyCreateRequest,
    *,
};

use crate::{routes::Route, utils::time::current_timestamp};

use super::DeliberationNewController;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    sample_survey: Signal<DeliberationSampleSurveyCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,

    pub committee_members: Signal<Vec<String>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let sample_survey = use_signal(|| DeliberationSampleSurveyCreateRequest::default());

        let mut ctrl = Self {
            lang,
            sample_survey,
            parent: use_context(),
            nav: use_navigator(),

            committee_members: use_signal(|| vec![]),
        };

        use_effect({
            let req = ctrl.parent.deliberation_requests();
            let mut sample_surveys = req
                .sample_surveys
                .get(0)
                .unwrap_or(&DeliberationSampleSurveyCreateRequest::default())
                .clone();
            let current_timestamp = current_timestamp();

            move || {
                let committees = req.roles.iter().map(|v| v.email.clone()).collect();
                let started_at = sample_surveys.clone().started_at;
                let ended_at = sample_surveys.clone().ended_at;

                if started_at == 0 {
                    sample_surveys.started_at = current_timestamp;
                }

                if ended_at == 0 {
                    sample_surveys.ended_at = current_timestamp;
                }

                ctrl.sample_survey.set(sample_surveys.clone());
                ctrl.committee_members.set(committees);
            }
        });

        Ok(ctrl)
    }

    pub fn set_title(&mut self, title: String) {
        self.sample_survey.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.sample_survey.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.sample_survey.with_mut(|req| {
            req.started_at = started_at;
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.sample_survey.with_mut(|req| {
            req.ended_at = ended_at;
        });
    }

    pub fn set_estimate_time(&mut self, estimate_time: i64) {
        self.sample_survey.with_mut(|req| {
            req.estimate_time = estimate_time;
        });
    }

    pub fn set_point(&mut self, point: i64) {
        self.sample_survey.with_mut(|req| {
            req.point = point;
        });
    }

    pub fn add_committee(&mut self, email: String) {
        self.sample_survey.with_mut(|req| {
            req.users.push(email);
        });
    }

    pub fn remove_committee(&mut self, email: String) {
        self.sample_survey.with_mut(|req| {
            req.users.retain(|e| !(e.clone() == email));
        })
    }

    pub fn clear_committee(&mut self) {
        self.sample_survey.with_mut(|req| req.users = vec![]);
    }

    pub fn add_question(&mut self) {
        self.sample_survey.with_mut(|req| {
            req.surveys.push(Question::default());
        });
    }

    pub fn remove_question(&mut self, index: usize) {
        self.sample_survey.with_mut(|req| {
            req.surveys.remove(index);
        });
    }

    pub fn update_question(&mut self, index: usize, question: Question) {
        self.sample_survey.with_mut(|req| {
            req.surveys[index] = question;
        });
    }

    pub fn get_sample_survey(&self) -> DeliberationSampleSurveyCreateRequest {
        (self.sample_survey)()
    }

    pub fn get_selected_committee(&self) -> Vec<String> {
        let sample_survey = self.get_sample_survey();
        let roles = sample_survey.clone().users;

        roles
    }

    pub fn back(&mut self) {
        self.parent.save_sample_survey(self.sample_survey());
        self.nav
            .replace(Route::DeliberationBasicInfoSettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_sample_survey(self.sample_survey());
        self.parent.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.parent.save_sample_survey(self.sample_survey());
            self.nav
                .push(Route::DeliberationSettingPage { lang: self.lang });
        }
    }

    pub fn is_valid(&self) -> bool {
        let sample_survey = self.sample_survey();

        let title = sample_survey.title;
        let description = sample_survey.description;
        let started_at = sample_survey.started_at;
        let ended_at = sample_survey.ended_at;

        let members = sample_survey.users;
        let surveys = sample_survey.surveys;

        !(title.is_empty()
            || description.is_empty()
            || started_at >= ended_at
            || members.is_empty()
            || surveys.is_empty())
    }

    pub fn validation_check(&self) -> bool {
        let sample_survey = self.sample_survey();

        let title = sample_survey.title;
        let description = sample_survey.description;
        let started_at = sample_survey.started_at;
        let ended_at = sample_survey.ended_at;

        let members = sample_survey.users;
        let surveys = sample_survey.surveys;

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
        ko = "표본 조사 제목을 입력해주세요.",
        en = "Please enter the sample survey title."
    )]
    TitleRequired,
    #[translate(
        ko = "표본 조사 설명을 입력해주세요.",
        en = "Please enter the sample survey description."
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
