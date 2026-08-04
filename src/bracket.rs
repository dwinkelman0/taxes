use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bracket {
    #[serde(default)]
    pub lower_bound: u32,
    pub upper_bound: u32,
    pub rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BracketSchedule(pub Vec<Bracket>);

impl BracketSchedule {
    pub fn get_amount_owed(&self, income: f64) -> f64 {
        let remaining = income;
        let mut total_tax = 0.0;
        for bracket in &self.0 {
            let lower = bracket.lower_bound as f64;
            let upper = bracket.upper_bound as f64;
            if remaining <= lower {
                break;
            }
            let taxable_in_bracket = (remaining.min(upper) - lower).max(0.0);
            total_tax += taxable_in_bracket * bracket.rate;
        }
        total_tax
    }
}
