use crate::state::AddressStore;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(domain_name: String)]
pub struct UpdateDomainAddress<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[domain_name.as_ref()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
}

pub fn update_domain_address(
    ctx: Context<UpdateDomainAddress>,
    domain_name: String,
    pk: Pubkey,
) -> Result<Pubkey> {
    let store = &mut ctx.accounts.address_store;
    if store.owner != ctx.accounts.user.key() {
        return Err(ProgramError::IllegalOwner.into());
    }
    store.address = pk;
    msg!("Address updated for {} with value {}", domain_name, pk);
    Ok(pk)
}
