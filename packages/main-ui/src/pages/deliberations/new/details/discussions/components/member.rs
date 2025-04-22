use bdk::prelude::*;

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::email_dropdown::EmailDropdown,
        details::discussions::i18n::DiscussionMemberTranslate,
    },
};

#[component]
pub fn Member(
    lang: Language,

    total_committees: Vec<String>,
    selected_committees: Vec<String>,

    add_committee: EventHandler<String>,
    remove_committee: EventHandler<String>,
    clear_committee: EventHandler<MouseEvent>,
) -> Element {
    let tr: DiscussionMemberTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            EmailDropdown {
                lang,
                id: "discussion-committee",
                hint: tr.search_committee,

                selected_committees,
                committees: total_committees,

                add_committee: move |email: String| {
                    add_committee.call(email);
                },
                remove_committee: move |email: String| {
                    remove_committee.call(email);
                },
                clear_committee: move |e| {
                    clear_committee.call(e);
                },
            }
        }
    }
}
