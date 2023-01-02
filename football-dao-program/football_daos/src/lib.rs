use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token};
use std::str::FromStr;
// use anchor_lang::solana_program::entrypoint::ProgramResult;
use spl_governance::instruction as spl_instruction;
use spl_governance::state::enums::MintMaxVoteWeightSource;

declare_id!("3Sz5VQ2VnxZsgTsGrJqUSbpfM4H4efMm8QCFqLtq6WjN");

#[program]
pub mod football_daos {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // // Before we create the DAO, we need to create the account that the DAO exists to manage
    // pub fn create_governed_account(ctx: Context<CreateGovernedAccount>) -> Result<()> {
    //     Ok(())
    // }

    // My idea of how to create a dao
    // Step 1 Create the community and (optionally) the Council Mints
    pub fn mint_token(ctx: Context<CreateCommunityMint>) -> Result<()> {
        // Create the MintTo struct for our context
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        token::mint_to(cpi_ctx, 100)?;

        Ok(())
    }

    // Step 2 Create the realm
    // Note the function create_realm automatically creates the holding accounts for the community and council tokens. See here: https://docs.rs/spl-governance/latest/src/spl_governance/instruction.rs.html#492
    pub fn create_realm(ctx: Context<CreateRealm>, realm_name: String) -> Result<()> {
        let current_program_pubkey: Pubkey =
            Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();

        let created_realm = spl_instruction::create_realm(
            &current_program_pubkey, // program_id: &Pubkey, This is the id of the custom governance program
            &current_program_pubkey, // realm_authority: &Pubkey, // I suppose to start with it will be this program, but in the next step we need to transfer this authority to a realm Governance so the DAO is self-governed
            &ctx.accounts.mint.key(), // community_token_mint: &Pubkey, // This needs to be created before the realm account
            ctx.accounts.user.key, //payer: &Pubkey, // I suppose this will be the account requesting the transaction
            None, // council_token_mint: Option<Pubkey>, // This needs to be created before the realm account
            None, // community_voter_weight_addin: Option<Pubkey>, // I think this might be optional - since it is an option, maybe it can have a value of None
            None, // max_community_voter_weight_addin: Option<Pubkey>, // I think this might be optional - since it is an option, maybe it can have a value of None
            realm_name, // name: String, // Name of the DAO should be supplied by the user
            60, // min_community_weight_to_create_governance: u64, // Min number of voter’s community weight required to create a governance. Maybe this can be a calculated percentage of the total mint supply
            MintMaxVoteWeightSource::SupplyFraction(10 ^ 10), //community_mint_max_vote_weight_source: MintMaxVoteWeightSource // The source used for community mint max vote weight source. No idea what this means. Values betweewn 0-100
        );
        Ok(())
    }

    // // Step 3 Create a realm Governance and transfer the realm account's realm_authority to it.
    // pub fn create_realm_governance() -> Result<()> {
    //     let created_realm_governance = spl_spl_instruction::create_governance(
    //         program_id: &Pubkey, // I think this refers to the current program that is building the DAO. Check this out https://docs.rs/spl-governance/latest/src/spl_governance/instruction.rs.html#700
    //         realm: &Pubkey, // The Pubkey of the realm created in the previous step
    //         governed_account: Option<&Pubkey>, // The pubkey of the governed account, in this case the realm_account
    //         token_owner_record: &Pubkey, // Idk what this is for? It seems to store some kind of tokens, but I don't understand what for. See here https://docs.rs/spl-governance/latest/spl_governance/state/token_owner_record/index.html
    //         payer: &Pubkey, // Probably the person creating this transaction, although DAOs can have treasuries, so maybe this should be set up at some point after the inital DAO setup?
    //         create_authority: &Pubkey, // I don't know what this is. Need to research it.
    //         voter_weight_record: Option<Pubkey>, // I don't know what this is, but it seems to be optional
    //         config: GovernanceConfig // Needs to be prepared before this. Need to create an instance of the GovernanceConfig struct for this Governance
    //     );

    //     // Now need to modify the realm account to make its authority the realm governance.
    //     // I am pretty sure that it is possible to create all the PubKeys in advance before actually making the accounts
    //     // That would make it easier, we could set the correct authority from the start even if the realm governance account has not been created
    //     // For now, we will use the set_realm_authority method
    //     let updated_authority = spl_instruction::set_realm_authority(
    //         program_id: &Pubkey, // Idk which program it is looking for now? Is it this program?
    //         realm: &Pubkey, // The pubkey of the realm
    //         realm_authority: &Pubkey, // The current realm authority which should be this program I think?
    //         new_realm_authority: Option<&Pubkey>, // The Pubkey of the realm governance created above
    //         action: SetRealmAuthorityAction // I think it needs to be SetRealmAuthorityAction::SetChecked Take a look at this: https://docs.rs/spl-governance/latest/src/spl_governance/state/realm.rs.html#54
    //     )

    // }

    // // Step 4 Create the governance account for the governed account
    // pub fn create_token_governance() -> Result<()> {
    //     let created_token_governance = spl_instruction::create_token_governance(
    //         program_id: &Pubkey, //
    //         realm: &Pubkey,
    //         governed_token: &Pubkey,
    //         governed_token_owner: &Pubkey,
    //         token_owner_record: &Pubkey,
    //         payer: &Pubkey,
    //         create_authority: &Pubkey,
    //         voter_weight_record: Option<Pubkey>,
    //         config: GovernanceConfig,
    //         transfer_account_authorities: bool
    //     )
    //     Ok(())
    // }

    // // Step 5 Create governance for the Mint that created the token account
    // pub fn create_mint_governance() -> Result<()> {
    //     let created_mint_governance = spl_instruction::create_mint_governance(
    //         program_id: &Pubkey,
    //         realm: &Pubkey,
    //         governed_mint: &Pubkey,
    //         governed_mint_authority: &Pubkey,
    //         token_owner_record: &Pubkey,
    //         payer: &Pubkey,
    //         create_authority: &Pubkey,
    //         voter_weight_record: Option<Pubkey>,
    //         config: GovernanceConfig,
    //         transfer_mint_authorities: bool
    //     )
    //     Ok(())
    // }

    // // Step 6 distribute tokens to token holders - Should this happen in an earlier step?
    // // Who is the authority for distributing tokens? How can people buy or acquire tokens? Is that part of the DAO or a separate mechanism?

    // // Step 7 Allow token holders to create proposals use spl_instruction::create_proposal()
    // fn add_proposal() -> Result<()> {
    //     let added_proposal = spl_instruction::create_proposal(
    //         program_id: &Pubkey, // Current program
    //         governance: &Pubkey, // Pubkey of the governance account that this proposal is for
    //         proposal_owner_record: &Pubkey, // The account making the proposal
    //         governance_authority: &Pubkey, // ??? I am guessing this is the realm_governance ???
    //         payer: &Pubkey, // User submiting the proposal
    //         voter_weight_record: Option<Pubkey>, // ??? Really need to research what the voter_weight thing means
    //         realm: &Pubkey, // Pubkey of the realm
    //         name: String, //  Name of the proposal
    //         description_link: String, // Link to the description - why is this separate?
    //         governing_token_mint: &Pubkey, // The Mint of the governing token being used to vote. This is probably how it knows if it is a community vote or a council vote
    //         vote_type: VoteType, // Either SingleChoice or MultiChoice, see this: https://docs.rs/spl-governance/latest/spl_governance/state/proposal/enum.VoteType.html
    //         options: Vec<String>, // I suppose these are the options you can vote for. In the function is is a vec of strings, but in the struct it becomes a vec of ProposalOptions. This function must turn the strings into the ProposalOption datatype. See here: https://docs.rs/spl-governance/latest/spl_governance/state/proposal/struct.ProposalOption.html
    //         use_deny_option: bool, // Whether people can vote to reject the proposal. This is mandatory for the proposals with executable instructions attached to them. See here: https://docs.rs/spl-governance/latest/spl_governance/state/proposal/struct.ProposalV2.html
    //         proposal_index: u32 // The index of the proposal, used to seed its address.
    //     )

    //     // After creating the proposal, the proposal owner needs to insert a transaction that will be executed for each option that may pass in the proposal
    //     let new_transaction = spl_instruction::insert_transaction(
    //         program_id: &Pubkey, // Current program
    //         governance: &Pubkey, // The governance account of the proposal
    //         proposal: &Pubkey, // The proposal account
    //         token_owner_record: &Pubkey, // ???
    //         governance_authority: &Pubkey, // ???
    //         payer: &Pubkey, // The account sumbiting the proposal
    //         option_index: u8, // The index of the option that this executable instruction is for
    //         index: u16, // This is used to seed the address of this proposal_transaction account, maybe there can be more than one proposal_transaction per ProposalOption? See here: https://docs.rs/spl-governance/latest/src/spl_governance/instruction.rs.html#1172
    //         hold_up_time: u32, // The amount of time to wait until this instruction can be executed after its proposal option passes
    //         instructions: Vec<InstructionData> // InstructionData is a Struct. See here: https://docs.rs/spl-governance/latest/spl_governance/state/proposal_transaction/struct.InstructionData.html
    //     )

    // }

    // // Step 8 Allow token holders to deposit tokens and vote on proposals
    // // To do this, users must deposit their tokens into a PDA token owner record, see here: https://docs.rs/spl-governance/latest/src/spl_governance/instruction.rs.html#564-605
    // fn deposit_governing_tokens() -> Result<()> {
    //     let deposit = spl_instruction::deposit_governing_tokens(
    //         program_id: &Pubkey, // Id of this program
    //         realm: &Pubkey, // Id of the realm
    //         governing_token_source: &Pubkey, // I suppose the wallet from which tokens are being transferred
    //         governing_token_owner: &Pubkey, // The owner (either single or multisig) of the deposited governing SPL Tokens This is who can authorize a withdrawal of the tokens
    //         governing_token_transfer_authority: &Pubkey, // ??? Idk what this is ???
    //         payer: &Pubkey, // The person depositing the tokens
    //         amount: u64, // The amount being transferred
    //         governing_token_mint: &Pubkey // The mint of the governing token
    //     )
    // }

    // fn vote() -> Result<()> {
    //     let vote = spl_instruction::cast_vote(
    //         program_id: &Pubkey, // This program
    //         realm: &Pubkey, // The realm
    //         governance: &Pubkey, // The governance of the proposal being voted for
    //         proposal: &Pubkey, // The proposal being voted for
    //         proposal_owner_record: &Pubkey, // The owner of the proposal being voted for
    //         voter_token_owner_record: &Pubkey, // The record of the deposited tokens of the person making the vote
    //         governance_authority: &Pubkey, // ??? I really need to research what the governance authority is ???
    //         governing_token_mint: &Pubkey, // The mint of the token being used to vote
    //         payer: &Pubkey, // The person submitting the vote. I suppose these will have to eventually become the treasury since the wallets used to vote may not have Sol in them beyond rent, only governance tokens
    //         voter_weight_record: Option<Pubkey>, // This is optional, so I suppose it is only relevant if we created a voter_weight_record as part of the proposal
    //         max_voter_weight_record: Option<Pubkey>, // This is optional, so I suppose it is only relevant if we created a max_voter_weight_record as part of the proposal
    //         vote: Vote // It is an enum. See here for the options: https://docs.rs/spl-governance/latest/spl_governance/state/vote_record/enum.Vote.html
    //     )
    // }

    // // Step 9 Allow proposals to close
    // fn close_proposal() -> Result<()> {
    //     let prposal_close = spl_instruction::finalize_vote(
    //         program_id: &Pubkey, // Current program
    //         realm: &Pubkey, // Current Realm
    //         governance: &Pubkey, // Governance for the proposal being closed
    //         proposal: &Pubkey, // Proposal being closed
    //         proposal_owner_record: &Pubkey, // Owner of the proposal being closed
    //         governing_token_mint: &Pubkey, // Mint of the proposal being closed
    //         max_voter_weight_record: Option<Pubkey> // // This is optional, so I suppose it is only relevant if we created a max_voter_weight_record as part of the proposal
    //     )
    // }

    // // Step 10 allow execution of succcessful transactions
    // fn execute_transaction() -> Result<()> {
    //     let execution = spl_instruction::execute_transaction(
    //         program_id: &Pubkey, // This program
    //         governance: &Pubkey, // The governance of the proposal who's transaction is being executed
    //         proposal: &Pubkey, // The proposal who's transaction is being executed
    //         proposal_transaction: &Pubkey, // The transaction being executed
    //         instruction_program_id: &Pubkey, // I guess that this is the program that processes the instruction? I thought that the governance account processes the instructions... Is this just the governance account then?
    //         instruction_accounts: &[AccountMeta] // From looking at the InstructionData struct, this seems to be the accounts that need to be passed to the instruction processor account
    //     )
    // }
}

#[derive(Accounts)]
pub struct Initialize {}

// Create community mint
#[derive(Accounts)]
pub struct CreateCommunityMint<'info> {
    /// CHECK: This is the token that we want to mint
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the token account that we want to mint tokens to
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    /// CHECK: the authority of the mint account
    pub authority: Signer<'info>,
}

// Create realm
#[derive(Accounts)]
pub struct CreateRealm<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// // Account structure
// // These are the accounts involved in creating a DAO
// // The realm account ties everything together. It commands the community mint account and council mint account
// #[account]
// pub struct realm_account {
//     // https://docs.rs/spl-governance/latest/spl_governance/state/realm/struct.RealmV2.html
//     pub account_type: GovernanceAccountType, // See this: https://docs.rs/spl-governance/latest/spl_governance/state/enums/enum.GovernanceAccountType.html
//     pub community_mint: Pubkey,
//     pub config: RealmConfig,
//     pub reserved: [u8; 6],
//     pub voting_proposal_count: u16,
//     pub authority: Option<Pubkey>,
//     pub name: String,
//     pub reserved_v2: [u8; 128],
//     // Governance Realm Account Account PDA seeds“ [‘governance’, name]
// }

// // Configuration of the realm account
// #[repr(C)]
// pub struct RealmConfig {
//     // https://docs.rs/spl-governance/latest/spl_governance/state/realm/struct.RealmConfig.html
//     pub use_community_voter_weight_addin: bool,
//     pub use_max_community_voter_weight_addin: bool,
//     pub reserved: [u8; 6],
//     pub min_community_weight_to_create_governance: u64,
//     pub community_mint_max_vote_weight_source: MintMaxVoteWeightSource,
//     pub council_mint: Option<Pubkey>,
// }

// // Configuration instruction arguments for the realm account
// #[repr(C)]
// pub struct RealmConfigArgs {
//     // https://docs.rs/spl-governance/latest/spl_governance/state/realm/struct.RealmConfigArgs.html
//     pub use_council_mint: bool,
//     pub min_community_weight_to_create_governance: u64,
//     pub community_mint_max_vote_weight_source: MintMaxVoteWeightSource,
//     pub use_community_voter_weight_addin: bool,
//     pub use_max_community_voter_weight_addin: bool,
// }

// // The community mint account mints tokens that allow the community to vote on proposals that require community votes
// #[account]
// pub struct community_mint_account {}

// // The council mint account mints tokens that allow the council to vote on proposals that require council votes
// #[account]
// pub struct council_mint_account {}

// // The governed account is the account that the DAO exists to control. A Solana DAO can have many of these, but for simplicity lets stick with one
// #[account]
// pub struct governed_account {
//     // Will need to make sure that this account is owned by the right program, that its authority is the right program and that it's mint is the system program for mints
//     // This needs to be a token account created by the spl token program
// }

// // The governance account is a program account which has authority over the governed account. It is derived from the realm account. A Solana DAO can have many of these but we are sticking with 1 for now
// #[account]
// pub struct governance_account {
//     // https://docs.rs/spl-governance/latest/spl_governance/state/governance/struct.GovernanceV2.html
//     pub account_type: GovernanceAccountType, // See this: https://docs.rs/spl-governance/latest/spl_governance/state/enums/enum.GovernanceAccountType.html
//     pub realm: Pubkey,
//     pub governed_account: Pubkey,
//     pub proposals_count: u32,
//     pub config: GovernanceConfig,
//     pub reserved: [u8; 6],
//     pub voting_proposal_count: u16,
//     pub reserved_v2: [u8; 128],
// }

// // The configuration of the governance account
// #[repr(C)]
// pub struct GovernanceConfig {
//     // https://docs.rs/spl-governance/latest/spl_governance/state/governance/struct.GovernanceConfig.html
//     pub vote_threshold_percentage: VoteThresholdPercentage,
//     pub min_community_weight_to_create_proposal: u64,
//     pub min_transaction_hold_up_time: u32,
//     pub max_voting_time: u32,
//     pub vote_tipping: VoteTipping,
//     pub proposal_cool_off_time: u32,
//     pub min_council_weight_to_create_proposal: u64,
// }

// // Every proposal takes the form of an account. It is derived from the governance account
// #[account]
// pub struct proposal_account {
//     // https://docs.rs/spl-governance/latest/spl_governance/state/proposal/struct.ProposalV2.html
//     pub account_type: GovernanceAccountType,
//     pub governance: Pubkey,
//     pub governing_token_mint: Pubkey,
//     pub state: ProposalState,
//     pub token_owner_record: Pubkey,
//     pub signatories_count: u8,
//     pub signatories_signed_off_count: u8,
//     pub vote_type: VoteType,
//     pub options: Vec<ProposalOption>,
//     pub deny_vote_weight: Option<u64>,
//     pub veto_vote_weight: Option<u64>,
//     pub abstain_vote_weight: Option<u64>,
//     pub start_voting_at: Option<UnixTimestamp>,
//     pub draft_at: UnixTimestamp,
//     pub signing_off_at: Option<UnixTimestamp>,
//     pub voting_at: Option<UnixTimestamp>,
//     pub voting_at_slot: Option<Slot>,
//     pub voting_completed_at: Option<UnixTimestamp>,
//     pub executing_at: Option<UnixTimestamp>,
//     pub closed_at: Option<UnixTimestamp>,
//     pub execution_flags: InstructionExecutionFlags,
//     pub max_vote_weight: Option<u64>,
//     pub max_voting_time: Option<u32>,
//     pub vote_threshold_percentage: Option<VoteThresholdPercentage>,
//     pub reserved: [u8; 64],
//     pub name: String,
//     pub description_link: String,
// }

// // Options for the proposal
// pub struct ProposalOption {
//     pub label: String,
//     pub vote_weight: u64,
//     pub vote_result: OptionVoteResult,
//     pub transactions_executed_count: u16,
//     pub transactions_count: u16,
//     pub transactions_next_index: u16,
// }

// // Proposal transaction - still don't understand what role this plays. Something to do with the executable code maybe? I don't see it in the proposal account or proposal option struct
// #[repr(C)]
// pub struct ProposalTransactionV2 {
//     pub account_type: GovernanceAccountType,
//     pub proposal: Pubkey,
//     pub option_index: u8,
//     pub transaction_index: u16,
//     pub hold_up_time: u32,
//     pub instructions: Vec<InstructionData>,
//     pub executed_at: Option<UnixTimestamp>,
//     pub execution_status: TransactionExecutionStatus,
//     pub reserved_v2: [u8; 8],
// }
