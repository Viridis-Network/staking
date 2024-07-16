use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub nft_collection: Pubkey,
    pub base_lock_days: u16,
    pub base_apy: u16,
}

impl Config {
    pub fn len() -> usize {
        8 + 32 + 32 + 2 + 2
    }
}

#[account]
pub struct StakeInfo {
    pub stakes: Vec<StakeEntry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct StakeEntry {
    pub amount: u64,
    pub start_time: i64,
    pub stake_lock_days: u16,
    pub base_apy: u16,
    pub nft_lock_time: Option<i64>,
    pub nft_lock_days: Option<u16>,
    pub nft_apy: Option<u16>,
    pub nft_unlock_time: Option<i64>,
    pub is_destaked: bool,
    pub paid_amount: u64,
}

impl StakeEntry {
    pub fn new(amount: u64, start_time: i64, stake_lock_days: u16, base_apy: u16) -> Self {
        Self {
            amount,
            start_time,
            stake_lock_days,
            base_apy,
            nft_lock_time: None,
            nft_lock_days: None,
            nft_apy: None,
            nft_unlock_time: None,
            is_destaked: false,
            paid_amount: 0,
        }
    }

    pub fn add_nft_info(&mut self, lock_time: i64, lock_days: u16, apy: u16) {
        self.nft_lock_time = Some(lock_time);
        self.nft_lock_days = Some(lock_days);
        self.nft_apy = Some(apy);
    }

    pub fn add_payment(&mut self, payment: u64) {
        self.paid_amount = self.paid_amount.saturating_add(payment);
    }

    pub fn is_nft_locked(&self) -> bool {
        self.nft_lock_time.is_some() && self.nft_unlock_time.is_none()
    }
}
