use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Clone, Copy, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum Plan {
    Day30,
    Day60,
    Day90,
    Months6,
    Year1,
}

impl Plan {
    pub fn duration(&self) -> i64 {
        match self {
            Plan::Day30 => 30 * 24 * 60 * 60,
            Plan::Day60 => 60 * 24 * 60 * 60,
            Plan::Day90 => 90 * 24 * 60 * 60,
            Plan::Months6 => 180 * 24 * 60 * 60,
            Plan::Year1 => 365 * 24 * 60 * 60,
        }
    }

    pub fn cost(&self) -> u64 {
        match self {
            Plan::Day30 => (30.0 * 1_000_000_000.0 * 0.001) as u64,
            Plan::Day60 => (60.0 * 1_000_000_000.0 * 0.001) as u64,
            Plan::Day90 => (90.0 * 1_000_000_000.0 * 0.001) as u64,
            Plan::Months6 => (180.0 * 1_000_000_000.0 * 0.001) as u64,
            Plan::Year1 => (365.0 * 1_000_000_000.0 * 0.001) as u64,
        }
    }
}
