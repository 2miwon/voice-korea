use bdk::prelude::*;
use models::{DeliberationFinalSurveyCreateRequest, OrganizationMemberSummary};

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::committee_dropdown::CommitteeDropdown,
        details::votes::i18n::FinalSurveyMemberTranslate,
    },
};

#[component]
pub fn FinalSurveyMember(
    lang: Language,

    final_survey: DeliberationFinalSurveyCreateRequest,
    set_final_survey: EventHandler<DeliberationFinalSurveyCreateRequest>,

    total_committees: Vec<OrganizationMemberSummary>,
    selected_committees: Vec<OrganizationMemberSummary>,
) -> Element {
    let tr: FinalSurveyMemberTranslate = translate(&lang);

    let select_ids: Vec<i64> = selected_committees
        .clone()
        .iter()
        .map(|v| v.user_id)
        .collect();

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            CommitteeDropdown {
                id: "final-committee",
                hint: tr.search_committee,

                selected_committees,
                committees: total_committees,

                add_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut final_survey = final_survey.clone();
                    move |member: OrganizationMemberSummary| {
                        select_ids.push(member.user_id);
                        final_survey.users = select_ids.clone();
                        set_final_survey.call(final_survey.clone());
                    }
                },
                remove_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut final_survey = final_survey.clone();
                    move |id: i64| {
                        select_ids.retain(|committee_id| !(committee_id.clone() == id));
                        final_survey.users = select_ids.clone();
                        set_final_survey.call(final_survey.clone());
                    }
                },
                clear_committee: {
                    let mut final_survey = final_survey.clone();
                    move |_| {
                        let select_ids = vec![];
                        final_survey.users = select_ids.clone();
                        set_final_survey.call(final_survey.clone());
                    }
                },
            }
        }
    }
}
