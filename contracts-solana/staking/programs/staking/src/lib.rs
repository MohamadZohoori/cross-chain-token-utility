use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("StaK1ng111111111111111111111111111111111");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        reward_rate: u64,
    ) -> Result<()> {
        initialize::handler(ctx, reward_rate)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        unstake::handler(ctx, amount)
    }
}
