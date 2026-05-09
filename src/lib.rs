use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("Ar6THRPrbwkjAg7L1pH6hcicwp5RFSNXpKHvfE2oef4T");

#[program]
pub mod solana_strive {
    use super::*;

    pub fn create_goal(ctx: Context<CreateGoal>, description: String, amount: u64) -> Result<()> {
        let goal = &mut ctx.accounts.goal_account;
        goal.user = ctx.accounts.user.key();
        goal.description = description;
        goal.staked_amount = amount;
        goal.is_completed = false;

        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.goal_account.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.goal_account.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn complete_goal(ctx: Context<CompleteGoal>) -> Result<()> {
        let goal = &mut ctx.accounts.goal_account;
        if goal.is_completed {
            return err!(MyError::AlreadyCompleted);
        }
        goal.is_completed = true;
        let amount = goal.staked_amount;
        **goal.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user.try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGoal<'info> {
    #[account(init, payer = user, space = 8 + 32 + 40 + 8 + 1)]
    pub goal_account: Account<'info, Goal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteGoal<'info> {
    #[account(mut, has_one = user)]
    pub goal_account: Account<'info, Goal>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Goal {
    pub user: Pubkey,
    pub description: String,
    pub staked_amount: u64,
    pub is_completed: bool,
}

#[error_code]
pub enum MyError {
    #[msg("Goal already finished!")]
    AlreadyCompleted,
}
