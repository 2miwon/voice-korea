use std::collections::HashMap;
use std::str::FromStr;

use crate::pages::surveys::components::start_project_modal::{
    StartProjectModal, StartProjectModalTranslate,
};
use crate::pages::surveys::models::attribute_combination::AttributeCombination;
use crate::pages::surveys::models::attribute_group_info::AttributeGroupInfo;
use bdk::prelude::btracing;
use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::attribute_v2::AgeV2;
use models::response::AgeV3;
use models::{
    attribute_v2::{GenderV2, RegionV2, SalaryV2},
    PanelV2, PanelV2Summary, SurveyV2,
};
use models::{AttributeDistribute, AttributeQuota};

use crate::{
    pages::surveys::models::current_step::CurrentStep,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::{
    create_survey::CreateSurveyResponse,
    i18n::{SettingAttributeTranslate, SurveyNewTranslate},
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    nav: Navigator,
    user: LoginService,

    total_survey_types: Signal<Vec<String>>,
    current_step: Signal<CurrentStep>,

    survey_request: Signal<Option<CreateSurveyResponse>>,

    popup_service: PopupService,
    panels: Signal<Vec<PanelV2Summary>>,
    selected_panels: Signal<Vec<PanelV2>>,
    maximum_panel_count: Signal<Vec<u64>>,
    total_panel_members: Signal<u64>,

    estimate_time: Signal<i64>,
    point: Signal<i64>,

    survey_id: Signal<Option<i64>>,

    attribute_options: Signal<HashMap<String, Vec<AttributeGroupInfo>>>,
    selected_attributes: Signal<Vec<String>>,
    selected_tab: Signal<bool>,
    total_counts: Signal<i64>,

    combination_error: Signal<bool>,

    attribute_combinations: Signal<Vec<AttributeCombination>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, survey_id: Option<i64>) -> Self {
        let tr: SettingAttributeTranslate = translate(&lang);
        let mut attribute_options: HashMap<String, Vec<AttributeGroupInfo>> = HashMap::new();

        let gender = GenderV2::variants(&lang)
            .into_iter()
            .skip(1)
            .map(|v| AttributeGroupInfo {
                name: tr.gender.to_string(),
                attribute: v,
                rate: 0,
            })
            .collect::<Vec<_>>();
        let region = RegionV2::variants(&lang)
            .into_iter()
            .skip(1)
            .map(|v| AttributeGroupInfo {
                name: tr.region.to_string(),
                attribute: v,
                rate: 0,
            })
            .collect::<Vec<_>>();
        let salary = SalaryV2::variants(&lang)
            .into_iter()
            .skip(1)
            .map(|v| AttributeGroupInfo {
                name: tr.salary.to_string(),
                attribute: v,
                rate: 0,
            })
            .collect::<Vec<_>>();
        let age = vec![
            AgeV2::Teenager,
            AgeV2::Twenty,
            AgeV2::Thirty,
            AgeV2::Fourty,
            AgeV2::Fifty,
            AgeV2::Sixty,
            AgeV2::Over,
        ]
        .iter()
        .map(|v| AttributeGroupInfo {
            name: tr.age.to_string(),
            attribute: v.translate(&lang).to_string(),
            rate: 0,
        })
        .collect();

        attribute_options.insert(tr.gender.to_string(), gender);

        attribute_options.insert(tr.region.to_string(), region);

        attribute_options.insert(tr.salary.to_string(), salary);

        attribute_options.insert(tr.age.to_string(), age);

        let translates: SurveyNewTranslate = translate(&lang);

        let login_service: LoginService = use_context();
        let org_id = use_memo(move || login_service.get_selected_org().unwrap_or_default().id);

        let mut ctrl = Self {
            nav: use_navigator(),
            user: use_context(),

            survey_request: use_signal(|| None),

            total_survey_types: use_signal(|| {
                vec![
                    translates.dropdown.to_string(),
                    translates.checkbox.to_string(),
                    translates.subjective.to_string(),
                    translates.rating.to_string(),
                ]
            }),

            current_step: use_signal(|| CurrentStep::CreateSurvey),
            panels: use_signal(|| vec![]),

            popup_service: use_context(),

            selected_panels: use_signal(|| vec![]),
            maximum_panel_count: use_signal(|| vec![]),
            total_panel_members: use_signal(|| 0),

            survey_id: use_signal(|| survey_id),

            estimate_time: use_signal(|| 0),
            point: use_signal(|| 0),

            attribute_options: use_signal(|| attribute_options.clone()),
            selected_attributes: use_signal(|| vec![]),
            selected_tab: use_signal(|| true),

            attribute_combinations: use_signal(|| vec![]),
            total_counts: use_signal(|| 0),

            combination_error: use_signal(|| false),
        };

        let survey_resource: Resource<Option<SurveyV2>> = use_resource({
            let org_id = org_id();
            move || {
                let survey_client = SurveyV2::get_client(&crate::config::get().api_url);

                async move {
                    if survey_id.is_none() {
                        None
                    } else {
                        match survey_client.get(org_id, survey_id.unwrap()).await {
                            Ok(d) => Some(d),
                            Err(e) => {
                                tracing::error!("get survey failed: {e}");
                                None
                            }
                        }
                    }
                }
            }
        });

        use_effect({
            let mut attribute_options = attribute_options.clone();

            move || {
                if let Some(Some(survey)) = survey_resource.value()() {
                    let combinations = survey.attribute_quotas.clone();
                    let attributes = survey.attribute_distributes.clone();
                    let quotas = survey.quotas.clone();
                    let mut selected_groups = vec![];
                    let mut attribute_combination = vec![];

                    let mut push_group = |name: String, attr_label: String| {
                        if !selected_groups.contains(&name) {
                            selected_groups.push(name.clone());
                        }

                        AttributeGroupInfo {
                            name,
                            attribute: attr_label,
                            rate: 0,
                        }
                    };

                    for combination in combinations {
                        let group_items = combination
                            .attributes
                            .iter()
                            .filter_map(|attr| match attr {
                                models::response::Attribute::Age(age) => Some(push_group(
                                    tr.age.to_string(),
                                    age.translate(&lang).to_string(),
                                )),
                                models::response::Attribute::Gender(g) => Some(push_group(
                                    tr.gender.to_string(),
                                    g.translate(&lang).to_string(),
                                )),
                                models::response::Attribute::Region(r) => Some(push_group(
                                    tr.region.to_string(),
                                    r.translate(&lang).to_string(),
                                )),
                                models::response::Attribute::Salary(s) => Some(push_group(
                                    tr.salary.to_string(),
                                    s.translate(&lang).to_string(),
                                )),
                                models::response::Attribute::None => None,
                            })
                            .collect::<Vec<_>>();

                        attribute_combination.push(AttributeCombination {
                            group: group_items,
                            total_rate: ((combination.user_count as f64) * 100.0 / (quotas as f64))
                                .floor() as i64,
                            total_count: combination.user_count as usize,
                        });
                    }

                    let mut update_rate = |key: String, attr_label: String, rate: i64| {
                        if let Some(list) = attribute_options.get_mut(&key) {
                            if let Some(item) = list.iter_mut().find(|g| g.attribute == attr_label)
                            {
                                item.rate = rate;
                            }
                        }
                    };

                    for attr in attributes {
                        match attr.attribute {
                            models::response::Attribute::Age(age) => {
                                update_rate(
                                    tr.age.to_string(),
                                    age.translate(&lang).to_string(),
                                    attr.rate,
                                );
                            }
                            models::response::Attribute::Gender(gender) => {
                                update_rate(
                                    tr.gender.to_string(),
                                    gender.translate(&lang).to_string(),
                                    attr.rate,
                                );
                            }
                            models::response::Attribute::Region(region) => {
                                update_rate(
                                    tr.region.to_string(),
                                    region.translate(&lang).to_string(),
                                    attr.rate,
                                );
                            }
                            models::response::Attribute::Salary(salary) => {
                                update_rate(
                                    tr.salary.to_string(),
                                    salary.translate(&lang).to_string(),
                                    attr.rate,
                                );
                            }
                            models::response::Attribute::None => {}
                        }
                    }

                    for (key, list) in attribute_options.iter_mut() {
                        if selected_groups.contains(key) {
                            list.retain(|item| item.rate != 0);
                        }
                    }

                    ctrl.survey_request.set(Some(CreateSurveyResponse {
                        title: survey.name.clone(),
                        description: survey.description.clone(),
                        start_date: survey.started_at,
                        end_date: survey.ended_at,
                        area: survey.project_area,
                        questions: survey.questions.clone(),
                        point: survey.point,
                        estimate_time: survey.estimate_time,
                    }));

                    ctrl.estimate_time.set(survey.estimate_time);
                    ctrl.point.set(survey.point);
                    ctrl.total_counts.set(survey.quotas);
                    ctrl.selected_attributes.set(selected_groups);
                    ctrl.attribute_combinations.set(attribute_combination);
                    ctrl.attribute_options.set(attribute_options.clone());
                }
            }
        });

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn set_total_counts(&mut self, total_counts: i64) {
        self.total_counts.set(total_counts);
        self.generate_combinations_with_meta();
    }

    pub fn helper(
        &self,
        acc: Vec<Vec<AttributeGroupInfo>>,
        rest: &[Vec<AttributeGroupInfo>],
    ) -> Vec<Vec<AttributeGroupInfo>> {
        if rest.is_empty() {
            return acc;
        }

        let mut result = vec![];
        for a in &acc {
            for r in &rest[0] {
                let mut new_comb = a.clone();
                new_comb.push(r.clone());
                result.push(new_comb);
            }
        }

        self.helper(result, &rest[1..])
    }

    pub fn generate_combinations_with_meta(&mut self) {
        let selected_attributes = self.selected_attributes();
        let options = self.attribute_options();

        let selected_values: Vec<Vec<AttributeGroupInfo>> = selected_attributes
            .iter()
            .filter_map(|key| options.get(key))
            .cloned()
            .collect();

        if selected_values.is_empty() {
            return;
        }

        let initial: Vec<Vec<AttributeGroupInfo>> =
            selected_values[0].iter().map(|x| vec![x.clone()]).collect();

        let raw_combinations = self.helper(initial, &selected_values[1..]);
        let total_counts = self.total_counts();

        let mut sum_counts: i64 = 0;

        let mut v: Vec<AttributeCombination> = raw_combinations
            .into_iter()
            .map(|group| {
                let total_rate = (group.iter().map(|v| v.rate as f64 / 100.0).product::<f64>()
                    * 100.0)
                    .floor() as i64;

                let total_count: usize =
                    ((total_counts as f64) * (total_rate as f64 / 100.0)).floor() as usize;

                sum_counts += total_count as i64;

                AttributeCombination {
                    group,
                    total_rate,
                    total_count,
                }
            })
            .collect();

        if (total_counts > sum_counts) && !v.is_empty() {
            let last_index = v.len() - 1;
            let last = &mut v[last_index];
            last.total_count =
                (last.total_count as usize + (total_counts - sum_counts) as usize) as usize;
        }

        self.attribute_combinations.set(v);
    }

    pub fn change_selected_tab(&mut self, selected: bool) {
        self.selected_tab.set(selected);
        self.change_rate();
        self.generate_combinations_with_meta();
    }

    pub fn add_selected_attribute(&mut self, attribute: String) {
        self.selected_attributes.with_mut(|attributes| {
            attributes.push(attribute);
        });

        self.change_rate();
        self.generate_combinations_with_meta();
    }

    pub fn remove_selected_attribute(&mut self, index: usize) {
        self.selected_attributes.with_mut(|attributes| {
            attributes.remove(index);
        });

        self.change_rate();
        self.generate_combinations_with_meta();
    }

    pub fn clear_selected_attributes(&mut self) {
        self.selected_attributes.with_mut(|attributes| {
            attributes.clear();
        });

        self.change_rate();
        self.generate_combinations_with_meta();
    }

    pub fn update_attribute_manual_rate(&mut self) {
        let selected_attributes = self.selected_attributes();
        let mut attribute_options = self.attribute_options();

        for key in &selected_attributes {
            if let Some(groups) = attribute_options.get_mut(key) {
                let len = groups.len();
                if len == 0 {
                    continue;
                }

                for i in 0..len {
                    groups[i].rate = 0;
                }
            }
        }

        self.attribute_options.set(attribute_options);
    }

    pub fn update_attribute_equal_rate(&mut self) {
        let selected_attributes = self.selected_attributes();
        let mut attribute_options = self.attribute_options();

        for key in &selected_attributes {
            if let Some(groups) = attribute_options.get_mut(key) {
                let len = groups.len();
                if len == 0 {
                    continue;
                }

                let d = 100 / len;
                let m = 100 % len;

                for i in 0..len {
                    groups[i].rate = if i < len - m {
                        d as i64
                    } else {
                        (d + 1) as i64
                    };
                }
            }
        }

        self.attribute_options.set(attribute_options);
    }

    pub fn remove_attribute_option(&mut self, key: String, attribute_name: String) {
        let mut should_remove_key = false;

        self.attribute_options.with_mut(|options| {
            if let Some(list) = options.get_mut(&key) {
                list.retain(|item| item.attribute != attribute_name);

                if list.is_empty() {
                    should_remove_key = true;
                }
            }
        });

        if should_remove_key {
            self.selected_attributes.with_mut(|attributes| {
                attributes.retain(|k| k != &key);
            });

            self.attribute_options.with_mut(|options| {
                options.remove(&key);
            });
        }

        self.change_rate();
        self.generate_combinations_with_meta();
    }

    pub fn change_rate(&mut self) {
        let selected = self.selected_tab();
        if selected {
            //true: equal, false: manual
            self.update_attribute_equal_rate();
        } else {
            self.update_attribute_manual_rate();
        }
    }

    pub fn change_attribute_combination_value(&mut self, index: usize, total_count: usize) {
        self.attribute_combinations.with_mut(|combi| {
            combi[index].total_count = total_count;
        });
    }

    pub fn update_attribute_rate(&mut self, key: String, attribute_name: String, rate: i64) {
        self.attribute_options.with_mut(|option| {
            if let Some(list) = option.get_mut(&key) {
                for group in list.iter_mut() {
                    if group.attribute == attribute_name {
                        group.rate = rate;
                        break;
                    }
                }
            }
        });

        self.generate_combinations_with_meta();
    }

    pub fn get_survey_id(&self) -> Option<i64> {
        (self.survey_id)()
    }

    pub fn change_survey_request(&mut self, req: CreateSurveyResponse) {
        self.survey_request.set(Some(req));
    }

    pub fn get_survey_request(&self) -> CreateSurveyResponse {
        let req = (self.survey_request)();

        if req.is_none() {
            CreateSurveyResponse::default()
        } else {
            req.unwrap()
        }
    }

    pub fn handle_survey_request(&mut self, survey: CreateSurveyResponse) {
        tracing::debug!("handle_survey_request: {:?}", survey);
        self.survey_request.set(Some(survey));
        self.current_step.set(CurrentStep::SettingPanel);
    }

    pub fn change_step(&mut self, step: CurrentStep) {
        self.current_step.set(step);
    }

    pub fn get_current_step(&self) -> CurrentStep {
        (self.current_step)()
    }

    pub fn get_total_survey_types(&self) -> Vec<String> {
        (self.total_survey_types)()
    }

    pub fn total_panels(&self) -> Vec<PanelV2Summary> {
        (self.panels)()
    }

    pub fn maximum_counts(&mut self) -> Vec<u64> {
        (self.maximum_panel_count)()
    }

    pub fn change_total_panel_members(&mut self, members: u64) {
        self.total_panel_members.set(members);
    }

    pub fn get_total_panel_members(&self) -> u64 {
        (self.total_panel_members)()
    }

    pub fn remove_all_selected_panel(&mut self) {
        self.selected_panels.set(vec![]);
        self.maximum_panel_count.set(vec![]);
        self.total_panel_members.set(0);
    }

    pub async fn open_start_project_modal(&mut self, lang: Language) {
        let tr: StartProjectModalTranslate = translate(&lang);
        let ctrl = self.clone();
        let mut popup_service = self.popup_service;

        let combi = self.attribute_combinations();
        let totals = self.total_counts();

        let mut sum = 0;
        for c in combi.clone() {
            sum += c.total_count as i64;
        }
        if totals != sum {
            self.combination_error.set(true);
            return;
        }
        self.combination_error.set(false);

        let org = self.user.get_selected_org();
        if org.is_none() {
            tracing::error!("Organization is not selected");
            return;
        }

        let org_id = org.unwrap().id;
        let survey_request = self.survey_request();
        let total_panels = self.total_counts();

        let survey_id = self.survey_id();

        popup_service
            .open(rsx! {
                StartProjectModal {
                    lang,
                    onsend: {
                        move |_| {
                            let survey_request = survey_request.clone();
                            async move {
                                if survey_id.is_none() {
                                    ctrl.create_survey(org_id, survey_request, total_panels).await;
                                } else {
                                    ctrl.update_survey(
                                            survey_id.unwrap_or_default(),
                                            org_id,
                                            survey_request,
                                            total_panels,
                                        )
                                        .await;
                                }
                                popup_service.close();
                            }
                        }
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("start project")
            .with_title(tr.title);
    }

    pub async fn update_survey(
        &self,
        survey_id: i64,
        org_id: i64,
        survey_request: Option<CreateSurveyResponse>,
        total_panels: i64,
    ) {
        let cli = SurveyV2::get_client(crate::config::get().api_url);
        let mut attribute_options = self.attribute_options();

        let mut attribute_distributes = vec![];

        for key in self.selected_attributes() {
            if let Some(groups) = attribute_options.get_mut(&key) {
                for group in groups {
                    let name = group.attribute.clone();

                    let attribute = if AgeV3::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Age(
                            AgeV3::from_str(&name).unwrap_or_default(),
                        )
                    } else if GenderV2::from_str(&name).is_some() {
                        models::prelude::response::Attribute::Gender(
                            GenderV2::from_str(&name).unwrap_or_default(),
                        )
                    } else if RegionV2::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Region(
                            RegionV2::from_str(&name).unwrap_or_default(),
                        )
                    } else {
                        models::prelude::response::Attribute::Salary(
                            SalaryV2::from_str(&name).unwrap_or_default(),
                        )
                    };

                    attribute_distributes.push(AttributeDistribute {
                        attribute,
                        rate: group.rate,
                    });
                }
            }
        }

        let mut attribute_quotas = vec![];

        let combinations = self.attribute_combinations();
        for attr in combinations {
            let attributes = attr
                .group
                .iter()
                .map(|v| {
                    let name = v.attribute.clone();

                    let attr = if AgeV3::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Age(
                            AgeV3::from_str(&name).unwrap_or_default(),
                        )
                    } else if GenderV2::from_str(&name).is_some() {
                        models::prelude::response::Attribute::Gender(
                            GenderV2::from_str(&name).unwrap_or_default(),
                        )
                    } else if RegionV2::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Region(
                            RegionV2::from_str(&name).unwrap_or_default(),
                        )
                    } else {
                        models::prelude::response::Attribute::Salary(
                            SalaryV2::from_str(&name).unwrap_or_default(),
                        )
                    };

                    attr
                })
                .collect();
            attribute_quotas.push(AttributeQuota {
                user_count: attr.total_count as i64,
                attributes,
            });
        }

        let CreateSurveyResponse {
            title,
            description,
            start_date,
            end_date,
            area,
            questions,

            point,
            estimate_time,
        } = survey_request.unwrap();

        match cli
            .update(
                org_id,
                survey_id,
                title,
                models::ProjectType::Survey,
                area,
                models::ProjectStatus::Ready,
                start_date,
                end_date,
                description,
                total_panels,
                questions,
                attribute_quotas,
                attribute_distributes,
                vec![],
                estimate_time,
                point,
                vec![],
            )
            .await
        {
            Ok(_) => {
                btracing::debug!("success to update survey");
                self.nav.go_back();
            }
            Err(e) => {
                btracing::error!("Failed to update survey with error: {:?}", e);
            }
        }
    }

    pub async fn create_survey(
        &self,
        org_id: i64,
        survey_request: Option<CreateSurveyResponse>,
        total_panels: i64,
    ) {
        let cli = SurveyV2::get_client(crate::config::get().api_url);
        let mut attribute_options = self.attribute_options();

        let mut attribute_distributes = vec![];

        for key in self.selected_attributes() {
            if let Some(groups) = attribute_options.get_mut(&key) {
                for group in groups {
                    let name = group.attribute.clone();

                    let attribute = if AgeV3::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Age(
                            AgeV3::from_str(&name).unwrap_or_default(),
                        )
                    } else if GenderV2::from_str(&name).is_some() {
                        models::prelude::response::Attribute::Gender(
                            GenderV2::from_str(&name).unwrap_or_default(),
                        )
                    } else if RegionV2::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Region(
                            RegionV2::from_str(&name).unwrap_or_default(),
                        )
                    } else {
                        models::prelude::response::Attribute::Salary(
                            SalaryV2::from_str(&name).unwrap_or_default(),
                        )
                    };

                    attribute_distributes.push(AttributeDistribute {
                        attribute,
                        rate: group.rate,
                    });
                }
            }
        }

        let mut attribute_quotas = vec![];

        let combinations = self.attribute_combinations();
        for attr in combinations {
            let attributes = attr
                .group
                .iter()
                .map(|v| {
                    let name = v.attribute.clone();

                    let attr = if AgeV3::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Age(
                            AgeV3::from_str(&name).unwrap_or_default(),
                        )
                    } else if GenderV2::from_str(&name).is_some() {
                        models::prelude::response::Attribute::Gender(
                            GenderV2::from_str(&name).unwrap_or_default(),
                        )
                    } else if RegionV2::from_str(&name).is_ok() {
                        models::prelude::response::Attribute::Region(
                            RegionV2::from_str(&name).unwrap_or_default(),
                        )
                    } else {
                        models::prelude::response::Attribute::Salary(
                            SalaryV2::from_str(&name).unwrap_or_default(),
                        )
                    };

                    attr
                })
                .collect();
            attribute_quotas.push(AttributeQuota {
                user_count: attr.total_count as i64,
                attributes,
            });
        }

        let CreateSurveyResponse {
            title,
            description,
            start_date,
            end_date,
            area,
            questions,

            point,
            estimate_time,
        } = survey_request.unwrap();

        match cli
            .create(
                org_id,
                title,
                area,
                start_date,
                end_date,
                description,
                total_panels,
                questions,
                attribute_quotas,
                attribute_distributes,
                vec![],
                vec![],
                estimate_time,
                point,
            )
            .await
        {
            Ok(_) => {
                btracing::debug!("success to create survey");
                self.nav.go_back();
            }
            Err(e) => {
                btracing::error!("Failed to create survey with error: {:?}", e);
            }
        };
    }

    pub fn back(&self) {
        self.nav.go_back();
    }
}
