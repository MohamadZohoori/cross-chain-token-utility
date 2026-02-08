use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::{
    constants::STAKE_SEED,
    errors::StakingError,
    state::user_stake::UserStake,
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 48,
        seeds = [STAKE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    require!(amount > 0, StakingError::InvalidAmount);

    let stake = &mut ctx.accounts.user_stake;
    let clock = Clock::get()?;

    token::transfer(
        ctx.accounts.into(),
        amount,
    )?;

    stake.owner = ctx.accounts.user.key();
    stake.amount += amount;
    stake.last_stake_ts = clock.unix_timestamp;

    Ok(())
}
