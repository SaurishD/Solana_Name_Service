use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AddressStore {
    pub owner: Pubkey,
    pub address: Pubkey,
    pub expiration_time: i64
}