use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program:: instruction::Instrucitons;

use std::convert::Into;
use std::ops::Deref

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_multisig {
    use super::*;

    // Initialise Multisig PDS with two parameter inputs
    // Threshold - for minimum number of signers required
    // Owners - Address of multisig account holders
    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        owners: Vec<Pubkey>,
        threshold: u64,
    ) -> ProgramResult {
        require!(!owner.is_empty() && owner.len() <= OWNER_MAX_SIZE, InvalidOwnersLen);
        require!(threshold > 0 && threshold < owner.len() as u64, InvalidThreshold);
        assert_unique_owners(&owners)?;

        let multisig = &mut ctx.accounts.multisig;
        multisig.owners = owners;
        multisig.threshold = threshold;
        multisig.nonce = *ctx.bumps.get("multisig_signer").unwrap();
        multisig.owner_set_seqno = 0;
        Ok(())
    }


    // Transaction account to be signer bt createn and must one of the owner
    // of the multisig
    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        accs: Vec<TransactionAccount>,
        data: Vec<u8>,
    ) -> ProgramResult {
        let owner_index = ctx
            .accounts
            .multisig
            .owners
            .iter()
            .position(|a| a == ctx.accounts.proposer.key)
            .ok_or(ErrorCode::InvalidOwner)?;

        let mut signers = vec![false; ctx.accounts.multisig.owners.len()];
        signers[owner_index] = true;

        let tx = &mut ctx.accounts.transaction;
        tx.program_id = pid;
        tx.accounts = accs;
        tx.data = data;
        tx.signers = signers;
        tx.multisig = ctx.accounts.multisig.key();
        tx.did_execute = false;
        tx.owner_set_seqno = ctx.accounts.multisig.owner_set_seqno;

        Ok(())

    )

    // Approve transaction on behalf of the owner of the multisig
    pub fn approve(ctx: Context<Approve>) -> ProgramResult {
        let owner_index = ctx
            .accounts
            .multisig
            .owners
            .iter()
            .position(|a| a == ctx.accounts.owner.key)
            .ok_or(ErrorCOde::InvalidOwner)?;
        ctx.accounts.transaction.signers[owner_index] = true;
    }
}

#[derive(Accounts)]
pub struct Initialize {}
