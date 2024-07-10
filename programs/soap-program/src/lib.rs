use {anchor_lang::prelude::*, instructions::*};

// pub mod errors;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("AvpSYxeEiRdkFUYoKXCAn9jHfZVR51uQFSc18qV2u1Dj");

#[program]
pub mod soap_program {
    use super::*;

    // pub fn mint_to() -> Result<()> {
    //     mint::handler(ctx);
    // }

    pub fn create(
        ctx: Context<Create>,
        soap_title: String,
        soap_symbol: String,
        soap_uri: String,
    ) -> Result<()> {
        create::handler(ctx, soap_title, soap_symbol, soap_uri)
    }

    pub fn fund_pot(ctx: Context<FundPot>, sol_lamports: u64) -> Result<()> {
        fund_pot::handler(ctx, sol_lamports)
    }

    pub fn withdraw_pot(ctx: Context<WithdrawPot>, sol_lamports: u64) -> Result<()> {
        withdraw_pot::handler(ctx, sol_lamports)
    }

    pub fn mint_soap(ctx: Context<MintSoap>) -> Result<()> {
        mint::handler(ctx)
    }
}
