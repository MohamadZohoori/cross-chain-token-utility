use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Insufficient staked balance")]
    InsufficientStake,
}
