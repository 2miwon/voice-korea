use bdk::prelude::*;
use dioxus_logger::tracing;
use models::{
    deliberation::{Deliberation, DeliberationQuery, DeliberationSummary},
    prelude::PanelInfo,
    QueryResponse,
};
use serde::{Deserialize, Serialize};

use crate::{
    pages::deliberations::components::remove_deliberation::RemoveDeliberationModal,
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::components::remove_deliberation::RemoveDeliberationModalTranslate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Opinion {
    pub project_id: String,
    pub opinion_type: String,
    pub project_name: String,
    pub total_response_count: u64,
    pub response_count: u64,
    pub panels: Vec<PanelInfo>,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub deliberations: Resource<QueryResponse<DeliberationSummary>>,
    pub popup_service: PopupService,
    page: Signal<usize>,
    pub size: usize,
    pub search_keyword: Signal<String>,
    pub selected_id: Signal<i64>,
    pub org_id: Signal<i64>,
    pub context_menu: Signal<bool>,
    pub mouse_pos: Signal<(f64, f64)>,
    pub nav: Navigator,
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();
        let page = use_signal(|| 1);
        let size = 10;
        let search_keyword = use_signal(|| "".to_string());

        let deliberations = use_server_future(move || {
            let page = page();
            let keyword = search_keyword().clone();

            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return QueryResponse {
                        items: vec![],
                        total_count: 0,
                    };
                }
                let client = Deliberation::get_client(&crate::config::get().api_url);

                let query = DeliberationQuery::new(size).with_page(page);

                if keyword.is_empty() {
                    match client.query(org_id.unwrap().id, query).await {
                        Ok(res) => res,
                        Err(e) => {
                            tracing::error!("Failed to list deliberations: {:?}", e);
                            return QueryResponse {
                                items: vec![],
                                total_count: 0,
                            };
                        }
                    }
                } else {
                    match client
                        .search_by(size, Some(page.to_string()), org_id.unwrap().id, keyword)
                        .await
                    {
                        Ok(res) => res,
                        Err(e) => {
                            tracing::error!("Failed to list deliberations: {:?}", e);
                            return QueryResponse {
                                items: vec![],
                                total_count: 0,
                            };
                        }
                    }
                }
            }
        })?;

        let ctrl = Self {
            lang,
            popup_service: use_context(),
            deliberations,
            page,
            size,
            search_keyword,
            org_id: use_signal(|| user.get_selected_org().unwrap_or_default().id),
            selected_id: use_signal(|| 0),
            context_menu: use_signal(|| false),
            mouse_pos: use_signal(|| (0.0, 0.0)),
            nav: use_navigator(),
        };

        Ok(ctrl)
    }

    pub fn set_page(&mut self, page: usize) {
        self.page.set(page);
    }

    pub fn get_x(&self) -> f64 {
        self.mouse_pos.with(|v| v.0 - 239.0)
    }

    pub fn get_y(&self) -> f64 {
        self.mouse_pos.with(|v| v.1 + 20.0)
    }

    pub fn move_user_page(&self, project_id: i64) {
        use web_sys::window;

        let lang = self.lang;
        let url = crate::config::get().user_url;

        let user_domain = format!(
            "{}/{}/projects/{}",
            url,
            if lang == Language::Ko { "ko" } else { "en" },
            project_id
        );

        if let Some(w) = window() {
            let _ = w.open_with_url(&user_domain);
        }
    }

    pub fn total_pages(&self) -> usize {
        let size = self.size;
        self.deliberations.with(|v| {
            if let Some(v) = v {
                if v.total_count != 0 {
                    (v.total_count as usize - 1) / size + 1
                } else {
                    0
                }
            } else {
                0
            }
        }) as usize
    }

    pub fn handle_click_menu(&mut self, id: i64, e: MouseEvent) {
        e.prevent_default();
        e.stop_propagation();

        let should_open = !self.context_menu() || self.selected_id() != id;

        self.context_menu.set(should_open);
        self.selected_id.set(id);
        let rect = e.page_coordinates();
        self.mouse_pos.set((rect.x, rect.y));
        tracing::debug!("opened: {} Mouse position: {:?}", should_open, rect);
    }

    pub async fn handle_remove(&mut self) {
        let mut popup_service = self.popup_service;
        let lang = self.lang;
        let org_id = self.org_id();
        let id = self.selected_id();

        let tr: RemoveDeliberationModalTranslate = translate(&lang);

        let mut ctrl = self.clone();

        popup_service
            .open(rsx! {
                RemoveDeliberationModal {
                    lang: self.lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    onremove: move |_e: MouseEvent| async move {
                        let client = Deliberation::get_client(&crate::config::get().api_url);
                        match client.remove_deliberation(org_id, id).await {
                            Ok(_) => {
                                ctrl.deliberations.restart();
                                ctrl.context_menu.set(false);
                                popup_service.close();
                            }
                            Err(e) => {
                                btracing::error!("failed to remove deliberation with error: {:?}", e);
                                ctrl.context_menu.set(false);
                                popup_service.close();
                            }
                        }
                    },
                }
            })
            .with_id("remove_deliberation")
            .with_title(tr.title);
    }

    pub fn handle_edit(&mut self) {
        self.context_menu.set(false);

        self.nav.push(Route::DeliberationEditPage {
            lang: self.lang,
            deliberation_id: self.selected_id(),
        });
    }
}
