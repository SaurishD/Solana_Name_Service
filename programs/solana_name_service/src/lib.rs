use anchor_lang::prelude::*;
pub mod state;

pub mod instructions;
use instructions::{add_domain::*, update_domain::*, close_domain::*, get_domain::*};
declare_id!("AMHL7gijZFDF6VKQzbT3TkuPSHmP76HP7KxBpV9RiG3y");

#[program]
pub mod solana_name_service {

    use crate::instructions::{add_domain, update_domain, close_domain, get_domain};

    use super::*;

    pub fn add_domain_address(ctx: Context<AddDomainAddress>,domain_name: String, pk: Pubkey) -> Result<()> {
        add_domain::add_domain_address(ctx, domain_name, pk)
    }

    pub fn update_domain_address(ctx: Context<UpdateDomainAddress>, domain_name: String, pk: Pubkey) -> Result<Pubkey>{
        update_domain::update_domain_address(ctx, domain_name, pk)
    }
    
    #[allow(unused_variables)]
    pub fn get_address(ctx: Context<GetDomainAddress>, domain_name: String) -> Result<Pubkey>{
        get_domain::get_address(ctx, domain_name)
    }

    pub fn close_domain_mapping(ctx: Context<CloseDomainAddress>, domain_name: String) -> Result<()> {
        close_domain::close_domain_mapping(ctx, domain_name)
    }
}





