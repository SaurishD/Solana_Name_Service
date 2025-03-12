use anchor_lang::prelude::*;

declare_id!("AMHL7gijZFDF6VKQzbT3TkuPSHmP76HP7KxBpV9RiG3y");

#[program]
pub mod solana_name_service {

    use super::*;

    pub fn add_domain_address(ctx: Context<AddDomainAddress>,domain_name: String, pk: Pubkey) -> Result<()> {
        let store = &mut ctx.accounts.address_store;
        store.address = pk;
        store.owner = ctx.accounts.user.key();
        msg!("PDA initialized for {} with value {}",domain_name, pk);
        Ok(())
    }

    pub fn update_domain_address(ctx: Context<UpdateDomainAddress>, domain_name: String, pk: Pubkey) -> Result<Pubkey>{
        let store = &mut ctx.accounts.address_store;
        if store.owner != ctx.accounts.user.key(){
            return Err(ProgramError::IllegalOwner.into());
        }
        store.address = pk;
        msg!("Address updated for {} with value {}",domain_name, pk);
        Ok(pk)
    }
    
    #[allow(unused_variables)]
    pub fn get_address(ctx: Context<GetDomainAddress>, domain_name: String) -> Result<Pubkey>{
        let store = &ctx.accounts.address_store;
        Ok(store.address)
    }

    pub fn close_domain_mapping(ctx: Context<CloseDomainAddress>, domain_name: String) -> Result<()> {
        let store = &mut ctx.accounts.address_store;
        let user = &mut ctx.accounts.user;

        let lamports = store.to_account_info().lamports();
        **user.to_account_info().try_borrow_mut_lamports()? += lamports;
        **store.to_account_info().try_borrow_mut_lamports()? = 0;
        msg!("PDA closed for {}",domain_name);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(domain_name: String)]
pub struct AddDomainAddress<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(  
        init_if_needed,
        payer=user,
        space=DISCRIMINATOR + AddressStore::INIT_SPACE,
        seeds=[domain_name.as_ref()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
    pub system_program: Program<'info, System>,
}

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

#[account]
#[derive(InitSpace)]
pub struct AddressStore{
    pub owner: Pubkey,
    pub address: Pubkey 
}

const DISCRIMINATOR: usize = 8;
