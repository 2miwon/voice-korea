use bdk::prelude::*;
use models::{elearning::ElearningCreateRequest, *};

use crate::{
    config,
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
    utils::time::current_timestamp,
};

use super::{components::load_data_modal::LoadDataModal, DeliberationNewController};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    org_id: i64,
    pub e_learning_tab: Signal<bool>,

    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    pub committee_members: Signal<Vec<String>>,

    pub deliberation: Signal<DeliberationContentCreateRequest>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,
    pub popup_service: PopupService,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let deliberation = use_signal(|| DeliberationContentCreateRequest::default());
        let popup_service: PopupService = use_context();

        let metadatas = use_server_future(move || {
            let page = 1;
            let size = 300;
            async move {
                let client = ResourceFile::get_client(&config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                client
                    .query(
                        org_id.unwrap().id,
                        ResourceFileQuery::new(size).with_page(page),
                    )
                    .await
                    .unwrap_or_default()
                    .items
            }
        })?;

        let mut ctrl = Self {
            lang,
            org_id: user.get_selected_org().unwrap_or_default().id,
            e_learning_tab: use_signal(|| true),

            metadatas,
            committee_members: use_signal(|| vec![]),

            parent: use_context(),
            nav: use_navigator(),
            deliberation,
            // elearnings: use_signal(|| vec![]),
            // questions: use_signal(|| vec![]),
            popup_service,
        };

        let req = ctrl.parent.deliberation_requests();

        let current_timestamp = current_timestamp();

        use_effect(move || {
            let committees = req.roles.iter().map(|v| v.email.clone()).collect();
            let mut deliberation = req
                .contents
                .get(0)
                .unwrap_or(&DeliberationContentCreateRequest::default())
                .clone();
            let started_at = deliberation.started_at;
            let ended_at = deliberation.ended_at;
            if started_at == 0 {
                deliberation.started_at = current_timestamp;
            }
            if ended_at == 0 {
                deliberation.ended_at = current_timestamp;
            }
            ctrl.deliberation.set(deliberation.clone());
            ctrl.committee_members.set(committees);
        });

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn set_title(&mut self, title: String) {
        self.deliberation.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.deliberation.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.deliberation.with_mut(|req| {
            req.started_at = started_at;
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.deliberation.with_mut(|req| {
            req.ended_at = ended_at;
        });
    }

    pub fn add_committee(&mut self, email: String) {
        self.deliberation.with_mut(|req| {
            req.users.push(email);
        });
    }

    pub fn remove_committee(&mut self, email: String) {
        self.deliberation.with_mut(|req| {
            req.users.retain(|e| !(e.clone() == email));
        })
    }

    pub fn clear_committee(&mut self) {
        self.deliberation.with_mut(|req| req.users = vec![]);
    }

    pub fn get_selected_committee(&self) -> Vec<String> {
        let deliberation = self.deliberation();
        let roles = deliberation.clone().users;

        roles
    }

    pub fn remove_elearning(&mut self, index: usize) {
        self.deliberation.with_mut(|req| {
            req.elearnings.remove(index);
        });
    }

    pub fn add_elearning(&mut self) {
        self.deliberation.with_mut(|req| {
            let mut elearning = ElearningCreateRequest::default();
            elearning.resources.push(ResourceFile::default());
            req.elearnings.push(elearning);
            tracing::debug!("elearnings: {:?}", req.elearnings);
        });
    }

    pub fn set_elearning_necessary(&mut self, index: usize, necessary: bool) {
        if index >= self.deliberation().elearnings.len() {
            tracing::error!("Index out of bounds: {}", index);
            return;
        }
        self.deliberation.with_mut(|req| {
            req.elearnings[index].necessary = necessary;
        });
    }

    pub fn set_elearning_title(&mut self, index: usize, title: String) {
        if index >= self.deliberation().elearnings.len() {
            tracing::error!("Index out of bounds: {}", index);
            return;
        }
        self.deliberation.with_mut(|req| {
            req.elearnings[index].title = title;
        });
    }

    pub async fn set_elearning_metadata(&mut self, index: usize, file: File) {
        let client = models::ResourceFile::get_client(&config::get().api_url);

        let file = client
            .create(
                self.org_id,
                file.name.clone(),
                None,
                None,
                None,
                None,
                None,
                vec![file],
            )
            .await
            .unwrap_or_default();

        self.deliberation.with_mut(|req| {
            tracing::debug!("elearnings: {:?} index: {:?}", req.elearnings, index);
            if req.elearnings[index].resources.is_empty() {
                req.elearnings[index].resources.push(file.clone());
            } else {
                req.elearnings[index].resources[0] = file.clone();
            }
        });
    }

    pub fn back(&mut self) {
        self.parent.save_content(self.deliberation());
        self.nav
            .replace(Route::DeliberationSampleSurveySettingPage { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        tracing::debug!("deliberations: {:?}", self.deliberation());
        self.parent.save_content(self.deliberation());
        self.parent.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.parent.save_content(self.deliberation());
            self.nav
                .push(Route::DeliberationDiscussionSettingPage { lang: self.lang });
        }
    }

    pub fn set_selected_field(&mut self, index: usize, field: String) {
        if index >= self.deliberation().questions.len() {
            tracing::error!("Index out of bounds: {}", index);
            return;
        }
        self.deliberation.with_mut(|req| {
            let question = req.questions[index].clone();
            let mut question_field = Question::new(&field);
            question_field.set_title(&question.title());
            question_field.set_description(&question.description());
            req.questions[index] = question_field;
        });
    }

    pub fn set_question_title(&mut self, index: usize, title: String) {
        if index >= self.deliberation().questions.len() {
            tracing::error!("Index out of bounds: {}", index);
            return;
        }
        self.deliberation.with_mut(|req| {
            req.questions[index].set_title(&title);
        });
    }

    pub fn set_question_description(&mut self, index: usize, content: String) {
        if index >= self.deliberation().questions.len() {
            tracing::error!("Index out of bounds: {}", index);
            return;
        }
        self.deliberation.with_mut(|req| {
            req.questions[index].set_description(&content);
        });
    }

    pub fn get_metadatas(&self) -> Vec<ResourceFileSummary> {
        match self.metadatas() {
            Ok(v) => v,
            Err(_) => vec![],
        }
    }

    pub fn open_load_from_data_modal(&mut self, index: usize) {
        let mut ctrl = *self;
        self.popup_service
            .open(rsx! {
                LoadDataModal {
                    lang: self.lang,
                    metadatas: ctrl.get_metadatas(),

                    onclose: move |_| {
                        ctrl.popup_service.close();
                    },
                    onupload: move |file: ResourceFile| {
                        ctrl.set_resource(index, file);
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("load_from_data");
    }

    pub fn add_question(&mut self) {
        self.deliberation.with_mut(|req| {
            req.questions.push(Question::default());
        });
    }

    pub fn remove_question(&mut self, index: usize) {
        self.deliberation.with_mut(|req| {
            req.questions.remove(index);
        });
    }

    pub fn change_option(&mut self, question_index: usize, option_index: usize, option: String) {
        self.deliberation.with_mut(|req| {
            req.questions[question_index].change_option(option_index, &option);
        });
    }

    pub fn remove_option(&mut self, question_index: usize, option_index: usize) {
        self.deliberation.with_mut(|req| {
            req.questions[question_index].remove_option(option_index);
        });
    }

    pub fn add_option(&mut self, question_index: usize) {
        self.deliberation.with_mut(|req| {
            req.questions[question_index].add_option("");
        });
    }

    pub fn set_resource(&mut self, index: usize, resource: ResourceFile) {
        if index >= self.deliberation().elearnings.len() {
            tracing::error!("Index out of bounds: {}", index);
            return;
        }
        self.deliberation
            .with_mut(|req: &mut DeliberationContentCreateRequest| {
                if req.elearnings[index].resources.is_empty() {
                    req.elearnings[index].resources.push(resource.clone());
                } else {
                    req.elearnings[index].resources[0] = resource.clone();
                }
            });
    }

    pub fn is_valid(&self) -> bool {
        let deliberation = self.deliberation();

        let title = deliberation.title;
        let description = deliberation.description;
        let started_at = deliberation.started_at;
        let ended_at = deliberation.ended_at;

        let members = deliberation.users;
        let elearnings = deliberation.elearnings;
        let questions = deliberation.questions;

        !(title.is_empty()
            || description.is_empty()
            || started_at >= ended_at
            || members.is_empty()
            || elearnings.is_empty()
            || questions.is_empty())
    }

    pub fn validation_check(&self) -> bool {
        let deliberation = self.deliberation();

        let title = deliberation.title;
        let description = deliberation.description;
        let started_at = deliberation.started_at;
        let ended_at = deliberation.ended_at;

        let members = deliberation.users;
        let elearnings = deliberation.elearnings;
        let questions = deliberation.questions;

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
        if elearnings.is_empty() {
            btracing::e!(self.lang, ValidationError::ElearningRequired);
            return false;
        }
        if questions.is_empty() {
            btracing::e!(self.lang, ValidationError::QuestionRequired);
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "숙의 제목을 입력해주세요.",
        en = "Please enter the deliberation title."
    )]
    TitleRequired,
    #[translate(
        ko = "숙의 설명을 입력해주세요.",
        en = "Please enter the deliberation description."
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
        ko = "하나 이상의 이러닝 자료를 생성해주세요.",
        en = "Please create one or more eLearning materials."
    )]
    ElearningRequired,
    #[translate(
        ko = "한 문항 이상의 설문을 입력해주세요.",
        en = "Please enter one or more questions in the survey."
    )]
    QuestionRequired,
}
