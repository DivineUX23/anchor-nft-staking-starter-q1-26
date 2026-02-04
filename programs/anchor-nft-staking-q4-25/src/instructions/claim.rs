use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, TokenAccount, mint_to}, token_interface::TokenInterface,
};

use crate::state::{StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub rewards_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Box<Account<'info, StakeConfig>>,

    #[account(
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump,
    )]
    pub reward_mint: Box<Account<'info, Mint>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"config", 
        &[self.config.bump],
        ]];

    let mint = MintTo{
        mint: self.reward_mint.to_account_info(),
        to: self.rewards_ata.to_account_info(),
        authority: self.config.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), mint, signer_seeds);

    let amount = self.user_account.points;

    mint_to(cpi_ctx, amount as u64)?;

    self.user_account.points -= amount;

    Ok(())

    }
}
