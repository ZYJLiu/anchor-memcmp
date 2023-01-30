use anchor_lang::prelude::*;

const DISCRIMINATOR_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_FIRST_NAME_LENGTH: usize = 10 * 4;
const MAX_LAST_NAME_LENGTH: usize = 10 * 4;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_memcmp {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        first_name: String,
        last_name: String,
    ) -> Result<()> {
        require_gte!(10, first_name.len(), MyError::FirstNameTooLong);
        require_gte!(10, last_name.len(), MyError::LastNameTooLong);

        let data = &mut ctx.accounts.data;

        let mut array_of_zeroes = vec![0u8; MAX_FIRST_NAME_LENGTH - first_name.len()];
        let new_first_name = first_name + std::str::from_utf8(&array_of_zeroes).unwrap();
        data.first_name = new_first_name;

        array_of_zeroes = vec![0u8; MAX_LAST_NAME_LENGTH - last_name.len()];
        let new_last_name = last_name + std::str::from_utf8(&array_of_zeroes).unwrap();
        data.last_name = new_last_name;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(first_name: String, last_name: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [first_name.as_bytes(), last_name.as_bytes()],
        bump,
        payer = payer,
        space = FullName::LEN
    )]
    pub data: Account<'info, FullName>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct FullName {
    pub first_name: String,
    pub last_name: String,
}

impl FullName {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_FIRST_NAME_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_LAST_NAME_LENGTH;
}

#[error_code]
pub enum MyError {
    #[msg("First name exceed 10 characters")]
    FirstNameTooLong,
    #[msg("Last name exceed 10 characters")]
    LastNameTooLong,
}
