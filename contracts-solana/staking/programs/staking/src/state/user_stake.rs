use anchor_lang::prelude::*;

#[account]
pub struct UserStake {
    pub owner: Pubkey,
    pub amount: u64,
    pub last_stake_ts: i64,
}
