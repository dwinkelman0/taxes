use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limits {
    pub max_401k: u32,
    pub max_hsa: u32,
    pub max_salt_deduction: u32,
}