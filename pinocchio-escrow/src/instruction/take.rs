use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey, ProgramResult,
};
use pinocchio_token::state::TokenAccount;

use crate::{constants::ESCROW_SEED, state::Escrow};

pub fn process_take(accounts: &[AccountInfo]) -> ProgramResult {
    let [taker, maker, mint_a, mint_b, taker_ata_a, taker_ata_b, maker_ata_b, vault, escrow, _system_program, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !taker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let escrow_account = Escrow::load(escrow)?;
    assert_eq!(escrow_account.mint_a, *mint_a.key());
    assert_eq!(escrow_account.mint_b, *mint_b.key());

    // Get transfer amount (from vault to taker).
    let transfer_amount;
    {
        let vault_account = pinocchio_token::state::TokenAccount::from_account_info(vault)?;
        transfer_amount = vault_account.amount();
    }

    // Validate escrow account.
    let escrow_pda = pubkey::create_program_address(
        &[
            ESCROW_SEED.as_bytes(),
            maker.key().as_ref(),
            &[escrow_account.bump as u8],
        ],
        &crate::ID,
    )?;
    if escrow.key() != &escrow_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate vault owner.
    assert!(unsafe {
        TokenAccount::from_account_info_unchecked(vault)
            .unwrap()
            .owner()
            == escrow.key()
    });

    // Transfer token from taker to maker.
    pinocchio_token::instructions::Transfer {
        from: taker_ata_b,
        to: maker_ata_b,
        authority: taker,
        amount: u64::from_le_bytes(escrow_account.receive_amount),
    }
    .invoke()?;

    // Transfer token from vault to taker.
    let bump = [escrow_account.bump as u8];
    let seed = [
        Seed::from(ESCROW_SEED.as_bytes()),
        Seed::from(maker.key()),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    pinocchio_token::instructions::Transfer {
        from: vault,
        to: taker_ata_a,
        authority: escrow,
        amount: transfer_amount,
    }
    .invoke_signed(&[seeds.clone()])?;

    // Close vault account.
    pinocchio_token::instructions::CloseAccount {
        account: vault,
        destination: maker,
        authority: escrow,
    }
    .invoke_signed(&[seeds])?;

    // Close escrow account.
    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0
    };

    Ok(())
}
