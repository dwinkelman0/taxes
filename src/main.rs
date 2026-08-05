mod bracket;
mod inputs;
mod limits;

use bracket::BracketSchedule;
use clap::Parser;
use inputs::{FilingStatus, Inputs};
use limits::Limits;
use std::fs;

fn load_json<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let data = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&data).expect("Failed to parse JSON")
}

#[derive(Parser, Debug)]
#[command(name = "taxes")]
#[command(about = "Calculate 2026 federal and California tax burden")]
struct CliArgs {
    #[arg(long, default_value_t = 150_000)]
    income: u32,

    #[arg(long)]
    contribution_401k: Option<u32>,

    #[arg(long)]
    contribution_hsa: Option<u32>,

    #[arg(long, default_value = "single")]
    filing_status: FilingStatus,
}

fn main() {
    let args = CliArgs::parse();

    let federal_schedule = load_json::<BracketSchedule>("data/federal_2026.json");
    let ca_schedule = load_json::<BracketSchedule>("data/california_2026.json");
    let limits = load_json::<Limits>("data/limits_2026.json");

    let inputs = Inputs {
        income: args.income,
        contribution_401k: args.contribution_401k,
        contribution_hsa: args.contribution_hsa,
        filing_status: args.filing_status,
    };

    let contribution_401k = inputs.contribution_401k.unwrap_or(limits.max_401k);
    let contribution_hsa = inputs.contribution_hsa.unwrap_or(limits.max_hsa);

    // Calculate CA tax first (SALT depends on this)
    let ca_deduction = *limits
        .ca_standard_deduction
        .get(&format!("{:?}", inputs.filing_status))
        .expect("Invalid filing status for CA standard deduction") as f64;
    let ca_taxable = inputs.income as f64 - contribution_401k as f64 - ca_deduction;
    let ca_tax = ca_schedule.get_amount_owed(ca_taxable);

    let fica_ss = limits.social_security.get_amount_owed(inputs.income as f64);
    let fica_medicare = limits.medicare.get_amount_owed(inputs.income as f64);
    let salt_deduction = (limits.max_salt_deduction as f64).min(ca_tax);
    let federal_taxable =
        inputs.income as f64 - contribution_401k as f64 - contribution_hsa as f64 - salt_deduction;
    let federal_tax = federal_schedule.get_amount_owed(federal_taxable);

    let total_tax = federal_tax + ca_tax + fica_ss + fica_medicare;
    let net_income =
        inputs.income as f64 - contribution_401k as f64 - contribution_hsa as f64 - total_tax;

    println!("--- Tax Calculation ---");
    println!("Gross income: ${:.2}", inputs.income as f64);
    println!("401k contribution: ${:.2}", contribution_401k as f64);
    println!("HSA contribution: ${:.2}", contribution_hsa as f64);
    println!("CA standard deduction: ${:.2}", ca_deduction);
    println!("Social Security tax: ${:.2}", fica_ss);
    println!("Medicare tax: ${:.2}", fica_medicare);
    println!("SALT deduction: ${:.2}", salt_deduction);
    println!("Federal taxable income: ${:.2}", federal_taxable);
    println!("CA taxable income: ${:.2}", ca_taxable);
    println!("Federal tax: ${:.2}", federal_tax);
    println!("CA tax: ${:.2}", ca_tax);
    println!("Total tax burden: ${:.2}", total_tax);
    println!("Net income: ${:.2}", net_income);
}
