use bdk::prelude::*;
use models::{DeliberationDiscussionCreateRequest, OrganizationMemberSummary};

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::committee_dropdown::CommitteeDropdown,
        details::discussions::i18n::DiscussionMemberTranslate,
    },
};

#[component]
pub fn DiscussionMember(
    lang: Language,
    discussion: DeliberationDiscussionCreateRequest,
    set_discussion: EventHandler<DeliberationDiscussionCreateRequest>,

    total_committees: Vec<OrganizationMemberSummary>,
    selected_committees: Vec<OrganizationMemberSummary>,
) -> Element {
    let tr: DiscussionMemberTranslate = translate(&lang);

    let select_ids: Vec<i64> = selected_committees.clone().iter().map(|v| v.id).collect();

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            CommitteeDropdown {
                id: "discussion-committee",
                hint: tr.search_committee,
                selected_committees,
                committees: total_committees,

                add_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut discussion = discussion.clone();
                    move |member: OrganizationMemberSummary| {
                        select_ids.push(member.id);
                        discussion.users = select_ids.clone();
                        set_discussion.call(discussion.clone());
                    }
                },
                remove_committee: {
                    let mut select_ids = select_ids.clone();
                    let mut discussion = discussion.clone();
                    move |id: i64| {
                        select_ids.retain(|committee_id| !(committee_id.clone() == id));
                        discussion.users = select_ids.clone();
                        set_discussion.call(discussion.clone());
                    }
                },
                clear_committee: {
                    let mut discussion = discussion.clone();
                    move |_| {
                        let select_ids = vec![];
                        discussion.users = select_ids.clone();
                        set_discussion.call(discussion.clone());
                    }
                },
            }
        }
    }
}
