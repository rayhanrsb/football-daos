// This is the function for creating a new realm
use anchor_lang::prelude::*;
use spl_governance::state::*;

pub fn create_realm(ctx: Context<CreateRealm>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the new realm account that has not yet been created
    pub realm_account: UncheckedAccount<'info>,
    // This is the current program
    #[account(address = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")]
    pub realm_authority_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub governance_token_mint_account: Account<'info, Mint>,
    #[account(mut)]
    pub governance_token_holding_account: Account<'info, Token>,
    #[account(mut)]
    pub payer_account: Account<'info, t>,
    #[account(mut)]
    pub system_account: Account<'info, t>,
    #[account(mut)]
    pub spl_token_account: Account<'info, t>,
    #[account(mut)]
    pub rent_sysvar_account: Account<'info, t>,
    #[account(mut)]
    pub council_token_mint_info: Account<'info, t>,
    #[account(mut)]
    pub council_token_holding_info: Account<'info, t>,
    #[account(mut)]
    pub realm_config_info: Account<'info, t>,
    #[account(mut)]
    pub community_voter_weight_addin_info: Account<'info, t>,
    #[account(mut)]
    pub community_max_voter_weight_addin_info: Account<'info, t>,
    #[account(mut)]
    pub council_voter_weight_addin_info: Account<'info, t>,
    #[account(mut)]
    pub council_max_voter_weight_addin_info: Account<'info, t>,
}
