// use anchor_lang::prelude::*;

// declare_id!("HJJvjsdr1TAkwvdR2Nzj5seqe2QyghhjuWhWGECTD9rK");

// #[program]
// pub mod solana_project {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(())
//     }
   
    

// }

// #[derive(Accounts)]
// pub struct Initialize {}








use anchor_lang::prelude::*;

declare_id!("Cv4aLWmQAVJCpnkGDWZ1SZ4JtnSHzkoY7ym4AZZt2iwh");

#[program]
pub mod vote_app {
    use super::*;
    
    pub fn register_candidate(
        ctx:  Context<RegisterCandidate>,
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


#[derive(Accounts)]
#[instruction(c_name:String)]
pub struct RegisterCandidate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, space= 8 + Candidate::INIT_SPACE, payer = payer,
    seeds=[c_name.as_bytes(),payer.key().as_ref()],bump)]

    pub candidate: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
    }