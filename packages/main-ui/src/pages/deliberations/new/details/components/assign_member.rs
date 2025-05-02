use bdk::prelude::*;

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::{
        components::email_dropdown::EmailDropdown, details::votes::i18n::FinalSurveyMemberTranslate,
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
    let tr: FinalSurveyMemberTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            EmailDropdown {
                lang,
                id: "final-committee",
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
