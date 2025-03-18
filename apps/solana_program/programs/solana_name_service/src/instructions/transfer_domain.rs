use anchor_lang::{account, prelude::*};

use crate::{state::AddressStore, types::errors::SnsError};

#[derive(Accounts)]
#[instruction(domain_name: String)]
pub struct TransferDomainOwnership<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[domain_name.as_ref()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
}

pub fn transfer_domain_ownership(
    ctx: Context<TransferDomainOwnership>,
    domain_name: String,
    new_owner: Pubkey,
) -> Result<()> {
    let store = &mut ctx.accounts.address_store;

    if ctx.accounts.user.key() != store.owner {
        return Err(SnsError::Unauthorised.into());
    }

    store.owner = new_owner;
    msg!("Ownership of {} transferred to {}", domain_name, new_owner);

    Ok(())
}
