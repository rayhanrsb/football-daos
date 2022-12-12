use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod football_daos {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


// Account structure
// These are the accounts involved in creating a DAO
// The realm account ties everything together. It commands the community mint account and council mint account
#[account]
pub struct realm_account {

}

// The community mint account mints tokens that allow the community to vote on proposals that require community votes
#[account]
pub struct community_mint_account {

}

// The council mint account mints tokens that allow the council to vote on proposals that require council votes
#[account]
pub struct council_mint_account {

}

// The governed account is the account that the DAO exists to control. A Solana DAO can have many of these, but for simplicity lets stick with one
#[account]
pub struct governed_account {

}

// The governance account is a program account which has authority over the governed account. It is derived from the realm account. A Solana DAO can have many of these but we are sticking with 1 for now
#[account]
pub struct governance_account {

}

// Every proposal takes the form of an account. It is derived from the governance account
#[account]
pub struct proposal_account {

}
