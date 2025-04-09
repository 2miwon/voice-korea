use bdk::prelude::*;

use crate::pages::deliberations::new::DeliberationNewPage;

#[component]
pub fn DeliberationEditPage(lang: Language, deliberation_id: i64) -> Element {
    rsx! {
        DeliberationNewPage { lang, deliberation_id }
    }
}
