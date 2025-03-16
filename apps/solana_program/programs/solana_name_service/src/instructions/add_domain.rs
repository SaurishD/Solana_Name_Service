
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use crate::state::AddressStore;
use crate::types::plan::*;


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


pub fn add_domain_address(ctx: Context<AddDomainAddress>,domain_name: String, pk: Pubkey, registration_plan: Plan) -> Result<()> {
    let store = &mut ctx.accounts.address_store;
    let cost =  registration_plan.cost();
    let duration = registration_plan.duration();
    let clock = Clock::get()?;

    let transfer_transction = system_instruction::transfer(&ctx.accounts.user.key(), &store.key(), cost);

    anchor_lang::solana_program::program::invoke(&transfer_transction, &[ctx.accounts.user.to_account_info(), store.to_account_info()])?;

    store.address = pk;
    store.owner = ctx.accounts.user.key();

    store.expiration_time = clock.unix_timestamp + duration;

    msg!("PDA initialized for {} with value {}",domain_name, pk);
    Ok(())
}