use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::bracket::BracketSchedule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limits {
    pub max_401k: u32,
    pub max_hsa: u32,
    pub max_salt_deduction: u32,
    pub ca_standard_deduction: HashMap<String, u32>,
    pub social_security: BracketSchedule,
    pub medicare: BracketSchedule,
}
