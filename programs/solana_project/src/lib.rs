use anchor_lang::prelude::*;

declare_id!("CyYX9fHRsarN73mVQp8Ygj8s8M3rv69PRM1iLfPcx3eX");

#[program]

pub mod vote_app {
    use super::*;

    pub fn register_candidate(
        ctx: Context<RegisterCandidate>,
        c_name: String,
        party_name: String,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.set_inner(Candidate {
            c_id: ctx.accounts.payer.key(),
            party_name: party_name,
            c_name: c_name,
            votes: 0,
        });
        Ok(())
    }

    pub fn register_voter(ctx: Context<RegisterVoter>, v_name: String) -> Result<()> {
        let voter = &mut ctx.accounts.voter;
        voter.set_inner(Voter {
            v_id: ctx.accounts.payer.key(),
            v_name: v_name,
            is_voted: false,
        });
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>) -> Result<()> {
        let voter = &mut ctx.accounts.voter;
        let candidate = &mut ctx.accounts.candidate;

        require!(voter.is_voted == false, VotingError::AlreadyVoted);
        require!(
            voter.v_id == ctx.accounts.payer.key(),
            VotingError::NotTheOwner
        );

        candidate.votes = candidate.votes + 1;
        voter.is_voted = true;
        Ok(())
    }

}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    c_id: Pubkey,
    #[max_len(20)]
    party_name: String,
    #[max_len(20)]
    c_name: String,
    votes: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Voter {
    v_id: Pubkey,
    #[max_len(20)]
    v_name: String,
    is_voted: bool,
}

#[error_code]
pub enum VotingError {
    #[msg("Already voted")]
    AlreadyVoted,

    #[msg("Not the owner")]
    NotTheOwner,
}

#[derive(Accounts)]
#[instruction(c_name: String)]

pub struct RegisterCandidate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init,
space = 8 + Candidate::INIT_SPACE,
payer = payer,
seeds = [c_name.as_bytes(),payer.key().as_ref()],
bump
)]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(v_name: String)]

pub struct RegisterVoter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init,
space = 8 + Voter::INIT_SPACE,
payer = payer,
seeds = [v_name.as_bytes(),payer.key().as_ref()],
bump
)]
    pub voter: Account<'info, Voter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info>{
 #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
 pub voter: Account<'info, Voter>,
    #[account(mut)]
 pub candidate: Account<'info, Candidate>,
}