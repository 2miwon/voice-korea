use serde::{Deserialize, Serialize};

use super::attribute_group_info::AttributeGroupInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeCombination {
    pub group: Vec<AttributeGroupInfo>,
    pub total_rate: f64,
    pub total_count: usize,
}
