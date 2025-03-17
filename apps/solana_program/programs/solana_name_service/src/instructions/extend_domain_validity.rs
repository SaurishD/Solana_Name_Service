use anchor_lang::{prelude::*, solana_program::system_instruction};

use crate::{state::AddressStore, types::plan::Plan};

#[derive(Accounts)]
#[instruction(domain_name: String)]
pub struct ExtendDomainValidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[domain_name.as_ref()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
    pub system_program: Program<'info, System>,
}

pub fn extend_domain_validity(
    ctx: Context<ExtendDomainValidity>,
    domain_name: String,
    extension_plan: Plan,
) -> Result<()> {
    let store = &mut ctx.accounts.address_store;
    if store.owner != ctx.accounts.user.key() {
        return Err(ProgramError::IllegalOwner.into());
    }
    let cost = extension_plan.cost();
    let duration = extension_plan.duration();

    let transfer_instruction =
        system_instruction::transfer(&ctx.accounts.user.key(), &store.key(), cost);

    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[ctx.accounts.user.to_account_info(), store.to_account_info()],
    )?;

    store.expiration_time += duration;
    msg!("Expiration time extended for domain {}", domain_name);

    Ok(())
}
