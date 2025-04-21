use bdk::prelude::*;
use models::{deliberation_role::DeliberationRoleCreateRequest, Role};

use crate::{
    components::updatable_card::UpdatableCard,
    pages::deliberations::new::preview::i18n::PreviewTranslate, routes::Route,
};

#[component]
pub fn Committee(
    lang: Language,
    roles: Vec<Role>,
    committees: Vec<DeliberationRoleCreateRequest>,
) -> Element {
    let tr: PreviewTranslate = translate(&lang);
    rsx! {
        UpdatableCard {
            lang,
            enable_line: true,
            title: tr.composition_committee,
            route: Route::CompositionCommitee { lang },
            div { class: "flex flex-col w-full justify-start items-start",
                for role in roles.iter() {
                    div { class: "flex flex-row w-full justify-start items-center min-h-55 gap-50",
                        div { class: "flex flex-row w-180 h-fit font-medium text-[15px] text-text-black",
                            {role.translate(&lang)}
                        }
                        div { class: "flex flex-wrap flex-row w-full justify-start items-center gap-20",
                            for c in committees.iter().filter(|c| c.role == *role) {
                                div { class: "flex flex-row w-fit gap-4",
                                    div { class: "w-24 h-24 rounded-full bg-[#9baae4]" }
                                    div { class: "font-medium text-sm text-text-black leading-17",
                                        {c.email.clone()}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
