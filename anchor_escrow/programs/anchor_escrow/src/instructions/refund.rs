use crate::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::close_account,
    token_interface::{
        transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
    mut,
    mint::token_program = token_program
  )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = maker,
    associated_token::token_program = token_program
  )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
    mut,
    close = maker,
    has_one = mint_a,
    seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
    bump = escrow.bump
  )]
    pub escrow: Account<'info, Escrow>,
    #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = escrow,
  )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        let seeds = [
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ];

        let escrow_signer_seed = &[&seeds[..]];

        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let transfer_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            escrow_signer_seed,
        );

        transfer_checked(transfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            escrow_signer_seed,
        );

        close_account(close_cpi_ctx)
    }
}
