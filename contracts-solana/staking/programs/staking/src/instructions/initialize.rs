use anchor_lang::prelude::*;
use crate::state::global_state::GlobalState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 40)]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    reward_rate: u64,
) -> Result<()> {
    let global = &mut ctx.accounts.global_state;
    global.authority = ctx.accounts.authority.key();
    global.reward_rate = reward_rate;
    Ok(())
}
