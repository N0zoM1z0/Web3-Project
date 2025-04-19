use anchor_lang::prelude::*;

declare_id!("FuDHv13W8SD8KVp2kUJc8vgQ48vq5Dc3p3wK5hKX9ddu");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
