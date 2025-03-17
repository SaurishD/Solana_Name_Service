use anchor_lang::prelude::*;
pub mod state;
pub mod types;
use types::plan::*;

pub mod instructions;
use instructions::{add_domain::*, close_domain::*, get_domain::*, update_domain::*, extend_domain_validity::*};
declare_id!("AMHL7gijZFDF6VKQzbT3TkuPSHmP76HP7KxBpV9RiG3y");

#[program]
pub mod solana_name_service {

    use instructions::extend_domain_validity::ExtendDomainValidity;

    use crate::instructions::{add_domain, close_domain, extend_domain_validity, get_domain, update_domain};

    use super::*;

    pub fn add_domain_address(
        ctx: Context<AddDomainAddress>,
        domain_name: String,
        pk: Pubkey,
        registration_plan: Plan,
    ) -> Result<()> {
        add_domain::add_domain_address(ctx, domain_name, pk, registration_plan)
    }

    pub fn update_domain_address(
        ctx: Context<UpdateDomainAddress>,
        domain_name: String,
        pk: Pubkey,
    ) -> Result<Pubkey> {
        update_domain::update_domain_address(ctx, domain_name, pk)
    }

    pub fn extend_domain_validity(ctx: Context<ExtendDomainValidity>, domain_name: String, extention_plan: Plan) -> Result<()> {
        extend_domain_validity::extend_domain_validity(ctx,domain_name,extention_plan)
    }

    #[allow(unused_variables)]
    pub fn get_address(ctx: Context<GetDomainAddress>, domain_name: String) -> Result<Pubkey> {
        get_domain::get_address(ctx, domain_name)
    }

    pub fn close_domain_mapping(
        ctx: Context<CloseDomainAddress>,
        domain_name: String,
    ) -> Result<()> {
        close_domain::close_domain_mapping(ctx, domain_name)
    }
}
