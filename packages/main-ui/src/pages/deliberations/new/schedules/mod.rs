mod controller;
mod i18n;
mod layout;
mod models;
// mod page;

pub use layout::*;
pub(self) use models::*;
// pub use page::*;

// children pages
mod basic_info;
// mod deliberation;
// mod discussion;
// mod recommendation;
mod sample_survey;
// mod vote;

pub use basic_info::*;
// pub use deliberation::*;
// pub use discussion::*;
// pub use recommendation::*;
pub use sample_survey::*;
// pub use vote::*;
