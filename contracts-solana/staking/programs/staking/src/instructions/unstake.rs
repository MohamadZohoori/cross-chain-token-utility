use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{
    constants::VAULT_SEED,
    errors::StakingError,
    state::user_stake::UserStake,
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut, has_one = owner)]
    pub user_stake: Account<'info, UserStake>,

    pub owner: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let stake = &mut ctx.accounts.user_stake;
    require!(stake.amount >= amount, StakingError::InsufficientStake);

    // PDA signer logic intentionally here
    token::transfer(
        ctx.accounts.into(),
        amount,
    )?;

    stake.amount -= amount;
    Ok(())
}
