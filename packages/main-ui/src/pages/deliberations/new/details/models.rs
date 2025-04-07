use bdk::prelude::*;

use crate::routes::Route;

#[derive(Debug, Clone, PartialEq, Copy, Translate)]
pub enum DeliberationDetailSettingStep {
    #[translate(ko = "기본 정보")]
    BasicInfo,
    #[translate(ko = "표본 조사")]
    SampleSurvey,
    #[translate(ko = "숙의")]
    Deliberation,
    #[translate(ko = "토론")]
    Discussion,
    #[translate(ko = "투표")]
    Vote,
}

impl DeliberationDetailSettingStep {
    pub fn to_route(&self, lang: Language) -> Route {
        match self {
            DeliberationDetailSettingStep::BasicInfo => {
                Route::DeliberationBasicInfoSettingPage { lang }
            }
            DeliberationDetailSettingStep::SampleSurvey => {
                Route::DeliberationSampleSurveySettingPage { lang }
            }
            DeliberationDetailSettingStep::Deliberation => Route::DeliberationSettingPage { lang },
            DeliberationDetailSettingStep::Discussion => {
                Route::DeliberationDiscussionSettingPage { lang }
            }
            DeliberationDetailSettingStep::Vote => Route::DeliberationVoteSettingPage { lang },
        }
    }
}

impl From<Route> for DeliberationDetailSettingStep {
    fn from(route: Route) -> Self {
        match route {
            Route::DeliberationBasicInfoSettingPage { .. } => Self::BasicInfo,
            Route::DeliberationSampleSurveySettingPage { .. } => Self::SampleSurvey,
            Route::DeliberationSettingPage { .. } => Self::Deliberation,
            Route::DeliberationDiscussionSettingPage { .. } => Self::Discussion,
            Route::DeliberationVoteSettingPage { .. } => Self::Vote,
            _ => Self::BasicInfo,
        }
    }
}
