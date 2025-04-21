use bdk::prelude::*;
use models::*;

use super::*;
use crate::{routes::Route, service::login_service::LoginService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    #[allow(dead_code)]
    pub parent_ctrl: ParentController,

    panels: Resource<Vec<PanelV2Summary>>,
    pub selected_panels: Signal<Vec<PanelV2Summary>>,
    nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();

        let panels = use_server_future(move || {
            let page = 1;
            let size = 20;
            let org_id = user.get_selected_org();

            async move {
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                let endpoint = crate::config::get().api_url;
                let res = PanelV2::get_client(endpoint)
                    .query(org_id.unwrap().id, PanelV2Query::new(size).with_page(page))
                    .await;

                res.unwrap_or_default().items
            }
        })?;

        let mut ctrl = Self {
            lang,
            panels,
            nav: use_navigator(),

            parent_ctrl: use_context(),
            selected_panels: use_signal(|| vec![]),
        };

        use_effect({
            let req = ctrl.parent_ctrl.deliberation_requests();
            let panels = panels().unwrap_or_default();
            move || {
                let selected_panels: Vec<PanelV2Summary> = panels
                    .iter()
                    .filter(|panel| req.panel_ids.contains(&panel.id))
                    .cloned()
                    .collect();

                ctrl.selected_panels.set(selected_panels);
            }
        });

        Ok(ctrl)
    }

    pub fn back(&mut self) {
        self.save_deliberation();
        self.nav.go_back();
    }

    pub async fn temp_save(&mut self) {
        self.save_deliberation();
        self.parent_ctrl.temporary_save(false).await;
    }

    pub fn next(&mut self) {
        if self.validation_check() {
            self.save_deliberation();
            self.nav
                .push(Route::DeliberationBasicInfoSettingPage { lang: self.lang });
        }
    }

    pub fn save_deliberation(&mut self) {
        let mut parent_ctrl = self.parent_ctrl;
        let panel_ids = self.selected_panels().iter().map(|v| v.id).collect();
        parent_ctrl.save_panels(panel_ids);
    }

    pub fn add_selected_panel(&mut self, panel: PanelV2Summary) {
        self.selected_panels.push(panel);
    }

    pub fn remove_selected_panel(&mut self, panel_id: i64) {
        self.selected_panels.retain(|panel| !(panel.id == panel_id));
    }

    pub fn clear_selected_panel(&mut self) {
        self.selected_panels.set(vec![]);
    }

    pub fn change_selected_panel_by_index(&mut self, index: usize, value: u64) {
        self.selected_panels.with_mut(|panels| {
            panels[index].user_count = value;
        });
    }

    pub fn is_valid(&self) -> bool {
        !(self.selected_panels().is_empty())
    }

    pub fn validation_check(&self) -> bool {
        if self.selected_panels().is_empty() {
            btracing::e!(self.lang, ValidationError::PanelRequired);
            return false;
        }

        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(
        ko = "공론에 참여할 패널을 1개 이상 선택해주세요.",
        en = "Please select at least one panelist to participate in the discussion."
    )]
    PanelRequired,
}
