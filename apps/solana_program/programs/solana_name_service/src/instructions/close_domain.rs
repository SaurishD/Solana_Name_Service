use crate::state::AddressStore;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(domain_name: String)]
pub struct CloseDomainAddress<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[domain_name.as_ref()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
}

pub fn close_domain_mapping(ctx: Context<CloseDomainAddress>, domain_name: String) -> Result<()> {
    let store = &mut ctx.accounts.address_store;
    let user = &mut ctx.accounts.user;

    let lamports = store.to_account_info().lamports();
    **user.to_account_info().try_borrow_mut_lamports()? += lamports;
    **store.to_account_info().try_borrow_mut_lamports()? = 0;
    msg!("PDA closed for {}", domain_name);
    Ok(())
}
