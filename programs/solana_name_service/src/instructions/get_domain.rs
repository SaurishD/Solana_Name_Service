
use anchor_lang::prelude::*;
use crate::state::AddressStore;

#[derive(Accounts)]
#[instruction(domain_name: String)]
pub struct GetDomainAddress<'info> {
    #[account(
        seeds=[domain_name.as_ref()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
    pub user: Signer<'info>
}


#[allow(unused_variables)]
    pub fn get_address(ctx: Context<GetDomainAddress>, domain_name: String) -> Result<Pubkey>{
        let store = &ctx.accounts.address_store;
        Ok(store.address)
    }