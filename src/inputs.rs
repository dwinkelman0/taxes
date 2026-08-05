use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum FilingStatus {
    Single,
    MarriedJointly,
    MarriedSeparately,
    HeadOfHousehold,
    SurvivingSpouse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inputs {
    pub income: u32,
    pub contribution_401k: Option<u32>,
    pub contribution_hsa: Option<u32>,
    pub filing_status: FilingStatus,
}
