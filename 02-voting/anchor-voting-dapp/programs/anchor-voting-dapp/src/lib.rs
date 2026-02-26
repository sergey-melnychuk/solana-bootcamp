use anchor_lang::prelude::*;

#[cfg(test)]
mod tests;

declare_id!("BLxZWhJKQSraa7NQGdbvK2yPj3Pyytg2fxZCJmEqesLw");

#[program]
pub mod anchor_voting_dapp {
   use super::*;

   pub fn initialize_poll(ctx: Context<InitializePoll>, poll_id: u64, description: String, poll_start: u64, poll_end: u64) -> Result<()> {
       let poll = &mut ctx.accounts.poll;
       poll.poll_id = poll_id;
       poll.poll_start = poll_start;
       poll.poll_end = poll_end;
       poll.description = description;
       Ok(())
   }

   pub fn close_poll(_ctx: Context<ClosePoll>, _poll_id: u64) -> Result<()> {
       Ok(())
   }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct ClosePoll<'info> {
   #[account(mut)]
   pub signer: Signer<'info>,
   #[account(
       mut,
       close = signer,
       seeds = [poll_id.to_le_bytes().as_ref()],
       bump,
   )]
   pub poll: Account<'info, Poll>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
   #[account(mut)]
   pub signer: Signer<'info>,
   #[account(
       init,
       payer = signer,
       space = 8 + Poll::INIT_SPACE,
       seeds = [
           poll_id.to_le_bytes().as_ref(),
       ],
       bump,
   )]
   pub poll: Account<'info, Poll>,
   pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
   pub poll_id: u64,
   #[max_len(280)]
   pub description: String,
   pub poll_start: u64,
   pub poll_end: u64,
   pub poll_index: u64,
   pub candidate_amount: u64,
}
