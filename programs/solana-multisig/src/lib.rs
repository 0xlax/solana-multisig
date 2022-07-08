use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program:: instruction::Instrucitons;

use std::convert::Into;
use std::ops::Deref

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_multisig {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
