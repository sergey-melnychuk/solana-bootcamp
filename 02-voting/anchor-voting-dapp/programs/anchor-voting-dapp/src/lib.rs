use anchor_lang::prelude::*;

declare_id!("AhrP4obPJ3BeWE4nGeyY5kwiiQ9kGEud1EUYy1vaLUNd");

#[program]
pub mod anchor_voting_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
