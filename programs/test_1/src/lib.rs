use anchor_lang::prelude::*;

declare_id!("HNTBeRrrjZvLdTUzQfyXz2ZYEU98icrwQpQtRUi12YvB");

#[program]
pub mod test_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
