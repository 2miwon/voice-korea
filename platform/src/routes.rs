use dioxus::prelude::*;

use crate::pages::attributes::AttributePage;
use crate::pages::create::CreatePage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::find_email::FindEmailPage;
use crate::pages::groups::_id::page::GroupDetailPage;
use crate::pages::groups::page::GroupPage;
use crate::pages::id::response_report::ResponseReportPage;
use crate::pages::id::select_response::response_type::SelectResponseDetailPage;
use crate::pages::id::select_response::SelectResponsePage;
use crate::pages::id::survey_summary::SurveySummaryPage;
use crate::pages::id::write_question::WriteQuestionPage;
use crate::pages::id::write_title::WriteTitlePage;
use crate::pages::login::LoginPage;
use crate::pages::members::_id::page::MemberDetailPage;
use crate::pages::members::page::MemberPage;
use crate::pages::opinions::new::page::OpinionCreatePage;
use crate::pages::opinions::page::OpinionPage;
use crate::pages::reset_password::ResetPasswordPage;
use crate::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/dashboard")]
            DashboardPage { lang: Language },
            #[route("/groups")]
            GroupPage { lang: Language },
            #[route("/groups/:group_id")]
            GroupDetailPage { lang: Language, group_id: String },
            #[route("/opinions")]
            OpinionPage { lang: Language },
            #[route("/opinions/new")]
            OpinionCreatePage { lang: Language },
            #[route("/members")]
            MemberPage { lang: Language },
            #[route("/members/:member_id")]
            MemberDetailPage { lang: Language, member_id: String },
            #[route("/id/:survey_id/write-title")]
            WriteTitlePage { lang: Language, survey_id: String },
            #[route("/id/:survey_id/write-question")]
            WriteQuestionPage { lang: Language, survey_id: String },
            #[route("/id/:survey_id/select-response")]
            SelectResponsePage { lang: Language, survey_id: String },
            #[route("/id/:survey_id/select-response/type/:select_type")]
            SelectResponseDetailPage { lang: Language, survey_id: String, select_type: String },
            #[route("/id/:survey_id/survey-summary")]
            SurveySummaryPage { lang: Language, survey_id: String },
            #[route("/id/:survey_id/response-report")]
            ResponseReportPage { lang: Language, survey_id: String },
            #[route("/attributes")]
            AttributePage { lang: Language },
        #[end_layout]

        #[route("/")]
        LoginPage { lang: Language },
        #[route("/create")]
        CreatePage { lang: Language },
        #[route("/find-email")]
        FindEmailPage { lang: Language },
        #[route("/reset-password")]
        ResetPasswordPage { lang: Language },
    #[end_nest]

    #[redirect("/", || Route::LoginPage { lang: Language::default() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
