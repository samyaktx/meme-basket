use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MemeBasketIndex {
    pub id: u64,
    pub maker: Pubkey,
    #[max_len(10)]
    pub meme_index: Vec<Pubkey>,
    pub bump: u8,
}
