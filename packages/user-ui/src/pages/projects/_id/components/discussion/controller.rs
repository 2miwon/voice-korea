use bdk::prelude::*;
use models::{
    DeliberationDiscussion, DeliberationDiscussionQuery, DeliberationDiscussionSummary, Discussion,
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,

    discussion: Resource<DeliberationDiscussionSummary>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let discussion = use_server_future(move || async move {
            let res = DeliberationDiscussion::get_client(&crate::config::get().api_url)
                .query(project_id(), DeliberationDiscussionQuery::new(1))
                .await
                .unwrap_or_default();
            if res.items.is_empty() {
                DeliberationDiscussionSummary::default()
            } else {
                res.items[0].clone()
            }
        })?;

        let ctrl = Self {
            lang,
            project_id,
            discussion,
        };

        Ok(ctrl)
    }

    pub async fn start_meeting(&self, discussion_id: i64) {
        let project_id = self.project_id();
        match Discussion::get_client(&crate::config::get().api_url)
            .start_meeting(project_id, discussion_id)
            .await
        {
            Ok(_) => btracing::info!("Meeting started successfully"),
            Err(err) => tracing::error!("Failed to start meeting: {:?}", err),
        }
    }
}
