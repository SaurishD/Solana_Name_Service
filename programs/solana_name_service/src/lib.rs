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
