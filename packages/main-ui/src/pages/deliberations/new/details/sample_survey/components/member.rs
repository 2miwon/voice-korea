use bdk::prelude::*;
use models::{DeliberationSampleSurveyCreateRequest, OrganizationMemberSummary};

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::committee_dropdown::CommitteeDropdown,
        details::sample_survey::i18n::SampleSurveyMemberTranslate,
    },
};

#[component]
pub fn SampleSurveyMember(
    lang: Language,

    sample_survey: DeliberationSampleSurveyCreateRequest,
    set_sample_survey: EventHandler<DeliberationSampleSurveyCreateRequest>,

    total_committees: Vec<OrganizationMemberSummary>,
    selected_committees: Vec<OrganizationMemberSummary>,
) -> Element {
    let tr: SampleSurveyMemberTranslate = translate(&lang);

    let select_ids: Vec<i64> = selected_committees
        .clone()
        .iter()
        .map(|v| v.user_id)
        .collect();
    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            CommitteeDropdown {
                id: "sample-committee",
                hint: tr.search_committee,

                selected_committees,
                committees: total_committees,

                add_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut sample = sample_survey.clone();
                    move |member: OrganizationMemberSummary| {
                        select_ids.push(member.user_id);
                        sample.users = select_ids.clone();
                        set_sample_survey.call(sample.clone());
                    }
                },
                remove_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut sample = sample_survey.clone();
                    move |id: i64| {
                        select_ids.retain(|committee_id| !(committee_id.clone() == id));
                        sample.users = select_ids.clone();
                        set_sample_survey.call(sample.clone());
                    }
                },
                clear_committee: {
                    let mut sample = sample_survey.clone();
                    move |_| {
                        let select_ids = vec![];
                        sample.users = select_ids.clone();
                        set_sample_survey.call(sample.clone());
                    }
                },
            }
        }
    }
}
