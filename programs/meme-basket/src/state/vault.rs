use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub owner: Pubkey,
    #[max_len(10)]
    pub token_accounts: Vec<Pubkey>, // Holds up to 10 token accounts
    pub bump: u8, // Bump seed for PDA
}