use anchor_lang::prelude::*;

declare_id!("AMHL7gijZFDF6VKQzbT3TkuPSHmP76HP7KxBpV9RiG3y");

#[program]
pub mod solana_name_service {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, pk: Pubkey) -> Result<()> {
        let store = &mut ctx.accounts.address_store;
        store.address = pk;
        store.owner = ctx.accounts.user.key();
        msg!("PDA initialized with value {}", pk);
        Ok(())
    }

    pub fn update_address(ctx: Context<Update>, pk: Pubkey) -> Result<Pubkey>{
        let store = &mut ctx.accounts.address_store;
        if store.owner != ctx.accounts.user.key(){
            return Err(ProgramError::IllegalOwner.into());
        }
        store.address = pk;
        Ok(pk)
    }

    pub fn get_address(ctx: Context<Get>) -> Result<Pubkey>{
        let store = &ctx.accounts.address_store;
        Ok(store.address)
    }

    pub fn close_domain_mapping(ctx: Context<Close>) -> Result<()> {
        let store = &mut ctx.accounts.address_store;
        let user = &mut ctx.accounts.user;

        let lamports = store.to_account_info().lamports();
        **user.to_account_info().try_borrow_mut_lamports()? += lamports;
        **store.to_account_info().try_borrow_mut_lamports()? = 0;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(  
        init_if_needed,
        payer=user,
        space=20,
        seeds=[seed.as_bytes()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Get<'info> {
    #[account(
        seeds=[name.as_bytes()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
    pub user: Signer<'info>
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[seed.as_bytes()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[seed.as_bytes()],
        bump
    )]
    pub address_store: Account<'info, AddressStore>,
}
#[account]
pub struct AddressStore{
    pub owner: Pubkey,
    pub address: Pubkey 
}
