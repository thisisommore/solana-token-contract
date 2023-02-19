use anchor_lang::prelude::*;
use anchor_spl::token::{MintTo, self,Token,Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_contract {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) ->
    Result<()> {
 
        let cpi_accounts = MintTo {
            mint:ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority:ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, 5)?;

        Ok(())
    }

    pub fn transfer_token(ctx:Context<TransferToken>) -> Result<()> {
        let transfer_instruction = Transfer{
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program,transfer_instruction);

        token::transfer(cpi_ctx, 2)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    /// CHECK: this is the token we want to mint
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    pub token_program: Program<'info,Token>,

    /// CHECK: this is the token account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: this is the authority
    // #[account(mut)]
    pub authority: AccountInfo<'info>,

}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub token_program:Program<'info,Token>,
    
    /// CHECK: this is the from
    #[account(mut)]
    pub from:UncheckedAccount<'info>,

    /// CHECK: this is the to
    #[account(mut)]
    pub to:AccountInfo<'info>,
    pub from_authority:Signer<'info>
}