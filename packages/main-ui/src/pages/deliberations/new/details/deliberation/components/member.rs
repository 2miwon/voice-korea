use bdk::prelude::*;
use models::{DeliberationContentCreateRequest, OrganizationMemberSummary};

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::committee_dropdown::CommitteeDropdown,
        details::deliberation::i18n::DeliberationMmeberTranslate,
    },
};

#[component]
pub fn DeliberationMember(
    lang: Language,

    deliberation: DeliberationContentCreateRequest,

    total_committees: Vec<OrganizationMemberSummary>,
    selected_committees: Vec<OrganizationMemberSummary>,

    add_committee: EventHandler<i64>,
    remove_committee: EventHandler<i64>,
    clear_committee: EventHandler<MouseEvent>,
) -> Element {
    let tr: DeliberationMmeberTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            CommitteeDropdown {
                id: "deliberation-committee",
                hint: tr.search_committee,

                selected_committees,
                committees: total_committees,

                add_committee: move |member: OrganizationMemberSummary| {
                    add_committee.call(member.user_id);
                },
                remove_committee: move |id: i64| {
                    remove_committee.call(id);
                },
                clear_committee: move |e| {
                    clear_committee.call(e);
                },
            }
        }
    }
}
