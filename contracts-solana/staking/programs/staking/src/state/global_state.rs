use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    pub authority: Pubkey,
    pub reward_rate: u64,
}
