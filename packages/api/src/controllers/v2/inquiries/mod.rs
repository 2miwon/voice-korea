use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{extract::State, routing::post, Extension, Json},
};
use models::{
    inquiry::{Inquiry, InquiryAction, InquiryCreateRequest, InquiryRepository},
    *,
};

#[derive(Clone, Debug)]
pub struct InquiryController {
    repo: InquiryRepository,
}

impl InquiryController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Inquiry::get_repository(pool.clone());

        Self { repo }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", post(Self::act_inquiry))
            .with_state(self.clone())
    }

    pub async fn act_inquiry(
        State(ctrl): State<InquiryController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<InquiryAction>,
    ) -> Result<Json<Inquiry>> {
        tracing::debug!("act_inquiry: {:?}", body);
        match body {
            InquiryAction::Create(param) => {
                let res: Inquiry = ctrl.create(param).await?;
                Ok(Json(res))
            }
        }
    }

    async fn create(&self, param: InquiryCreateRequest) -> Result<Inquiry> {
        let _ = self
            .notify_slack(
                param.name.clone(),
                param.email.clone(),
                param.message.clone(),
            )
            .await;

        let res = self
            .repo
            .insert(param.name, param.email, param.message)
            .await?;

        Ok(res)
    }

    async fn notify_slack(&self, name: String, email: String, message: String) -> Result<()> {
        let config = crate::config::get();

        let msg = format!(
            "Voice Korea\nname: {}\nemail: {}\nmessage: {}",
            name, email, message
        );

        tracing::debug!(
            "notify_slack: {} {}",
            msg,
            config.slack_channel_inquiry.to_string()
        );

        btracing::notify!(config.slack_channel_inquiry.to_string(), &msg);

        Ok(())
    }
}
