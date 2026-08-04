use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inputs {
    pub income: u32,
    pub contribution_401k: Option<u32>,
    pub contribution_hsa: Option<u32>,
}