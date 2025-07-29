use anchor_lang::prelude::*;

declare_id!("CwfXu54P5bsW7r5CokUzvYbSc5NSKATRouRmjtp9UZwu");

#[program]
pub mod program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
