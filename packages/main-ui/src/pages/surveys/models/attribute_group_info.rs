use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeGroupInfo {
    pub name: String,
    pub attribute: String,
    pub rate: i64,
}
