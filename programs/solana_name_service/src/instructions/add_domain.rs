
use anchor_lang::prelude::*;
use crate::state::AddressStore;


#[derive(Accounts)]
#[instruction(domain_name: String)]
pub struct AddDomainAddress<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(  
        init_if_needed,
        payer=user,
        space=8 + AddressStore::INIT_SPACE,
        seeds=[domain_name.as_ref()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
    pub system_program: Program<'info, System>,
}


pub fn add_domain_address(ctx: Context<AddDomainAddress>,domain_name: String, pk: Pubkey) -> Result<()> {
    let store = &mut ctx.accounts.address_store;
    store.address = pk;
    store.owner = ctx.accounts.user.key();
    msg!("PDA initialized for {} with value {}",domain_name, pk);
    Ok(())
}