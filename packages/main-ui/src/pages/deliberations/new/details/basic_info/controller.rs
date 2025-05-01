use bdk::prelude::*;
use models::*;

use super::*;
use crate::{
    config,
    routes::Route,
    service::login_service::LoginService,
    utils::time::{current_timestamp_with_time, parsed_timestamp_with_time},
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    pub metadatas: Resource<Vec<ResourceFileSummary>>,
    pub surveys: Resource<Vec<SurveyV2Summary>>,
    basic_info: Signal<DeliberationBasicInfoCreateRequest>,
    pub committee_members: Signal<Vec<String>>,

    #[allow(dead_code)]
    pub search_keyword: Signal<String>,
    #[allow(dead_code)]
    pub documents: Signal<Vec<ResourceFile>>,
    pub parent: DeliberationNewController,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let basic_info = use_signal(|| DeliberationBasicInfoCreateRequest::default());
        let search_keyword = use_signal(|| "".to_string());

        let metadatas = use_server_future(move || {
            let page = 1;
            let size = 300;
            let keyword = search_keyword().clone();
            async move {
                let client = ResourceFile::get_client(&config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }

                if keyword.is_empty() {
                    let query = ResourceFileQuery::new(size).with_page(page);
                    client
                        .query(org_id.unwrap().id, query)
                        .await
                        .unwrap_or_default()
                        .items
                } else {
                    client
                        .search_by(size, Some(page.to_string()), org_id.unwrap().id, keyword)
                        .await
                        .unwrap_or_default()
                        .items
                }
            }
        })?;

        let surveys = use_server_future(move || {
            let page = 1;
            let size = 100;

            async move {
                let client = SurveyV2::get_client(&crate::config::get().api_url);
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }

                match client
                    .query(org_id.unwrap().id, SurveyV2Query::new(size).with_page(page))
                    .await
                {
                    Ok(res) => res.items,
                    Err(e) => {
                        tracing::error!("Failed to list surveys: {:?}", e);
                        vec![]
                    }
                }
            }
        })?;

        let mut ctrl = Self {
            lang,
            basic_info,
            metadatas,
            surveys,
            parent: use_context(),
            nav: use_navigator(),

            search_keyword,

            documents: use_signal(|| vec![]),
            committee_members: use_signal(|| vec![]),
        };

        use_effect({
            let req = ctrl.parent.deliberation_requests();
            let mut basic_info = req
                .basic_infos
                .get(0)
                .unwrap_or(&DeliberationBasicInfoCreateRequest::default())
                .clone();

            move || {
                let committees = req.roles.iter().map(|v| v.email.clone()).collect();
                let started_at = basic_info.clone().started_at;
                let ended_at = basic_info.clone().ended_at;

                if started_at == 0 {
                    basic_info.started_at = current_timestamp_with_time(0, 0, 0);
                }

                if ended_at == 0 {
                    basic_info.ended_at = current_timestamp_with_time(23, 59, 59);
                }

                ctrl.basic_info.set(basic_info.clone());
                ctrl.committee_members.set(committees);
            }
        });

        Ok(ctrl)
    }

    pub fn set_title(&mut self, title: String) {
        self.basic_info.with_mut(|req| {
            req.title = title;
        });
    }

    pub fn set_description(&mut self, description: String) {
        self.basic_info.with_mut(|req| {
            req.description = description;
        });
    }

    pub fn set_start_date(&mut self, started_at: i64) {
        self.basic_info.with_mut(|req| {
            req.started_at = parsed_timestamp_with_time(started_at, 0, 0, 0);
        });
    }

    pub fn set_end_date(&mut self, ended_at: i64) {
        self.basic_info.with_mut(|req| {
            req.ended_at = parsed_timestamp_with_time(ended_at, 23, 59, 59);
        });
    }

    pub fn add_survey(&mut self, survey_id: i64) {
        self.basic_info.with_mut(|req| {
            req.surveys.push(survey_id);
        });
    }

    pub fn remove_survey(&mut self, survey_id: i64) {
        self.basic_info.with_mut(|req| {
            req.surveys.retain(|id| !(id.clone() == survey_id));
        });
    }

    pub fn clear_survey(&mut self) {
        self.basic_info.with_mut(|req| req.surveys = vec![]);
    }

    pub fn add_committee(&mut self, email: String) {
        self.basic_info.with_mut(|req| {
            req.users.push(email);
        });
    }

    pub fn remove_committee(&mut self, email: String) {
        self.basic_info.with_mut(|req| {
            req.users.retain(|e| !(e.clone() == email));
        })
    }

    pub fn clear_committee(&mut self) {
        self.basic_info.with_mut(|req| req.users = vec![]);
    }

    pub fn get_basic_info(&self) -> DeliberationBasicInfoCreateRequest {
        (self.basic_info)()
    }

    pub async fn create_resource(&mut self, file: File) -> Result<()> {
        let metadata = self.create_metadata(file).await;

        match metadata {
            Ok(v) => {
                let mut basic_info = self.basic_info();

                basic_info.resources.push(v.id);
                self.basic_info.set(basic_info);
                self.metadatas.restart();
                Ok(())
            }
            Err(e) => {
                tracing::error!("Create Failed Reason: {:?}", e);
                Err(models::ApiError::ReqwestFailed(e.to_string()))
            }
        }
    }

    pub fn get_selected_surveys(&self) -> Vec<SurveyV2Summary> {
        let total_surveys = self.surveys().unwrap_or_default();
        let basic_info = self.get_basic_info();
        let surveys = basic_info.clone().surveys;

        total_surveys
            .clone()
            .into_iter()
            .filter(|survey| surveys.iter().any(|id| id.clone() == survey.id))
            .collect()
    }

    pub fn get_selected_committee(&self) -> Vec<String> {
        let basic_info = self.get_basic_info();
        let roles = basic_info.clone().users;

        roles
    }

    pub fn get_selected_resources(&self) -> Vec<ResourceFile> {
        let metadatas = self.metadatas().unwrap_or_default();
        let resources = self.get_basic_info().resources;

        metadatas
            .clone()
            .into_iter()
            .filter(|resource| resources.iter().any(|id| id.clone() == resource.id))
            .map(|v| v.into())
            .collect()
    }

    pub fn add_resource(&mut self, resource: ResourceFile) {
        let mut basic_info = self.basic_info();
        basic_info.resources.push(resource.id);
        self.basic_info.set(basic_info);
    }

    pub fn delete_resource(&mut self, id: i64) {
        let mut basic_info = self.basic_info();
        basic_info.resources.retain(|doc| doc.clone() != id);
        self.basic_info.set(basic_info);
    }

    pub async fn create_metadata(&self, file: File) -> Result<ResourceFile> {
        let user: LoginService = use_context();
        let org = user.get_selected_org();
        if org.is_none() {
            return Err(models::ApiError::OrganizationNotFound);
        }
        let org_id = org.unwrap().id;
        let client = models::ResourceFile::get_client(&config::get().api_url);

        client
            .create(
                org_id,
                file.name.clone(),
                None,
                None,
                None,
                None,
                None,
                vec![file],
            )
            .await
    }

    pub fn back(&mut self) {
        self.parent.save_basic_info(self.basic_info());
        self.nav
            .replace(Route::CompositionPanel { lang: self.lang });
    }

    pub async fn temp_save(&mut self) {
        self.parent.save_basic_info(self.basic_info());
        self.parent.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.parent.save_basic_info(self.basic_info());
            self.nav
                .push(Route::DeliberationSampleSurveySettingPage { lang: self.lang });
        }
    }

    pub fn is_valid(&self) -> bool {
        let basic_info = self.basic_info();

        let title = basic_info.title;
        let description = basic_info.description;
        let started_at = basic_info.started_at;
        let ended_at = basic_info.ended_at;

        let members = basic_info.users;

        !(title.is_empty()
            || started_at >= ended_at
            || description.is_empty()
            || members.is_empty())
    }

    pub fn validation_check(&self) -> bool {
        let basic_info = self.basic_info();

        let title = basic_info.title;
        let description = basic_info.description;
        let started_at = basic_info.started_at;
        let ended_at = basic_info.ended_at;

        let members = basic_info.users;

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

        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "기본 정보 제목을 입력해주세요.",
        en = "Please enter the basic info title."
    )]
    TitleRequired,
    #[translate(
        ko = "기본 정보 설명을 입력해주세요.",
        en = "Please enter the basic info description."
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
}
