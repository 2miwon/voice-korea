pub mod prelude {
    pub use crate::routes::*;
    pub use crate::utils::context::*;

    pub use crate::pages::not_found::NotFoundPage;
    pub use dioxus::document::eval;
}

pub mod config;
pub mod pages;

pub mod service {
    pub mod login_service;
    pub mod metadata_api;
    pub mod opinion_api;
    pub mod organization_api;
    pub mod popup_service;
    pub mod theme;
}

pub mod models {
    pub mod pi;
    pub mod question;
    pub mod role_field;
    pub mod survey;
    pub mod user;
}

pub mod utils {
    pub mod api;
    pub mod context;
    pub mod hash;
    pub mod metadata;
    pub mod time;
}

pub mod components {
    pub mod alert;
    pub mod bar_graph;
    pub mod block_header;
    pub mod bottom;
    pub mod button;
    pub mod calendar;
    pub mod checkbox;
    pub mod close_label;
    pub mod custom_checkbox;
    pub mod drop_zone;
    pub mod dropdown;
    pub mod expandable_card;
    pub mod file_list;
    pub mod form_field;
    pub mod icons;
    pub mod input;
    pub mod label;
    pub mod outside_hook;
    pub mod pagination;
    pub mod pi_graph;
    pub mod popup;
    pub mod radio;
    pub mod section;
    pub mod select;
    pub mod select_category;
    pub mod select_date;
    pub mod stepper;
    pub mod table_row;
    pub mod textarea;
    pub mod updatable_card;
    pub mod upload_button;
}

pub mod api;
pub mod routes;
