use mollusk_svm::result::Check;
use mollusk_svm::{program, Mollusk};
use pinocchio_amm::constants::CONFIG_SEED;
use pinocchio_amm::instruction::{
    DepositInstructionData, InitializeInstructionData, SwapInstructionData, WithdrawInstructionData,
};
use pinocchio_amm::state::Config;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use solana_sdk::{account::WritableAccount, program_option::COption, program_pack::Pack};
use spl_token::state::AccountState;

use pinocchio_amm::ID;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const PAYER: Pubkey = pubkey!("9vCdf2rh7hA7JdSVV1LEbJGFDNLjk1KHGTZW1wSRN6vC");

pub const DEPOSIT_AMOUNT: u64 = 10;
pub const RECEIVE_AMOUNT: u64 = 9;

pub fn mollusk() -> Mollusk {
    let mut mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_amm");
    mollusk.add_program(
        &spl_token::ID,
        "tests/elf_files/spl_token",
        &mollusk_svm::program::loader_keys::LOADER_V3,
    );
    mollusk
}

#[test]

fn test_initialize() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();
    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let authority = Pubkey::new_from_array([0x01; 32]);
    let authority_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let seed: u64 = 1;
    let (config, config_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(CONFIG_SEED.as_bytes()), &seed.to_le_bytes()],
        &PROGRAM,
    );
    let config_account = Account::new(0, 0, &system_program);

    let mint_x = Pubkey::new_from_array([0x02; 32]);
    let mut mint_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_y = Pubkey::new_from_array([0x03; 32]);
    let mut mint_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault_x = Pubkey::new_from_array([0x04; 32]);
    let mut vault_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault_y = Pubkey::new_from_array([0x05; 32]);
    let mut vault_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_lp = Pubkey::new_from_array([0x06; 32]);
    let mut mint_lp_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::Some(config),
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_lp_account.data_as_mut_slice(),
    )
    .unwrap();

    // Create the instruction data
    let fee: u16 = 500;
    let instruction_data = InitializeInstructionData {
        seed: seed.to_le_bytes(),
        fee: fee.to_le_bytes(),
        config_bump,
    };

    // instruction discriminator = 0
    let mut ser_instruction_data = vec![0];

    // Serialize the instruction data
    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&instruction_data));

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new(vault_x, false),
            AccountMeta::new(vault_y, false),
            AccountMeta::new(mint_lp, false),
            AccountMeta::new(config, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (authority, authority_account),
            (mint_x, mint_x_account),
            (mint_y, mint_y_account),
            (vault_x, vault_x_account),
            (vault_y, vault_y_account),
            (mint_lp, mint_lp_account),
            (config, config_account),
            (system_program, system_account),
            (token_program, token_account),
        ],
        &[Check::success()],
    );
}

#[test]

fn test_deposit() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();
    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let authority = Pubkey::new_from_array([0x00; 32]);

    let user = Pubkey::new_from_array([0x01; 32]);
    let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let seed: u64 = 1;
    let fee: u16 = 500;

    let mint_x = Pubkey::new_from_array([0x02; 32]);
    let mut mint_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_y = Pubkey::new_from_array([0x03; 32]);
    let mut mint_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_y_account.data_as_mut_slice(),
    )
    .unwrap();

    // Config account.
    let (config, config_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(CONFIG_SEED.as_bytes()), &seed.to_le_bytes()],
        &PROGRAM,
    );
    let mut config_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Config::LEN),
        Config::LEN,
        &PROGRAM.into(),
    );
    let config_state = Config {
        seed: seed.to_le_bytes(),
        authority: *authority.as_array(),
        mint_x: *mint_x.as_array(),
        mint_y: *mint_y.as_array(),
        fee: fee.to_le_bytes(),
        config_bump: config_bump,
    };
    config_account.data = bytemuck::bytes_of(&config_state).to_vec();

    let vault_x = Pubkey::new_from_array([0x04; 32]);
    let mut vault_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault_y = Pubkey::new_from_array([0x05; 32]);
    let mut vault_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_x = Pubkey::new_from_array([0x06; 32]);
    let mut user_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_y = Pubkey::new_from_array([0x07; 32]);
    let mut user_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_lp = Pubkey::new_from_array([0x08; 32]);
    let mut mint_lp_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::Some(config),
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_lp_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_lp = Pubkey::new_from_array([0x09; 32]);
    let mut user_lp_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_lp,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_lp_account.data_as_mut_slice(),
    )
    .unwrap();

    // Create the instruction data
    let amount: u64 = 100;
    let max_x: u64 = 50;
    let max_y: u64 = 50;
    let instruction_data = DepositInstructionData {
        amount: amount.to_le_bytes(),
        max_x: max_x.to_le_bytes(),
        max_y: max_y.to_le_bytes(),
    };

    // instruction discriminator = 1
    let mut ser_instruction_data = vec![1];

    // Serialize the instruction data
    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&instruction_data));

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new(vault_x, false),
            AccountMeta::new(vault_y, false),
            AccountMeta::new(user_x, false),
            AccountMeta::new(user_y, false),
            AccountMeta::new(mint_lp, false),
            AccountMeta::new(user_lp, false),
            AccountMeta::new(config, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (user, user_account),
            (mint_x, mint_x_account),
            (mint_y, mint_y_account),
            (vault_x, vault_x_account),
            (vault_y, vault_y_account),
            (user_x, user_x_account),
            (user_y, user_y_account),
            (mint_lp, mint_lp_account),
            (user_lp, user_lp_account),
            (config, config_account),
            (system_program, system_account),
            (token_program, token_account),
        ],
        &[Check::success()],
    );
}

#[test]
fn test_swap() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();
    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let authority = Pubkey::new_from_array([0x00; 32]);

    let user = Pubkey::new_from_array([0x01; 32]);
    let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let seed: u64 = 1;
    let fee: u16 = 500;

    let mint_x = Pubkey::new_from_array([0x02; 32]);
    let mut mint_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_y = Pubkey::new_from_array([0x03; 32]);
    let mut mint_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_y_account.data_as_mut_slice(),
    )
    .unwrap();

    // Config account.
    let (config, config_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(CONFIG_SEED.as_bytes()), &seed.to_le_bytes()],
        &PROGRAM,
    );
    let mut config_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Config::LEN),
        Config::LEN,
        &PROGRAM.into(),
    );
    let config_state = Config {
        seed: seed.to_le_bytes(),
        authority: *authority.as_array(),
        mint_x: *mint_x.as_array(),
        mint_y: *mint_y.as_array(),
        fee: fee.to_le_bytes(),
        config_bump: config_bump,
    };
    config_account.data = bytemuck::bytes_of(&config_state).to_vec();

    let vault_x = Pubkey::new_from_array([0x04; 32]);
    let mut vault_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault_y = Pubkey::new_from_array([0x05; 32]);
    let mut vault_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_x = Pubkey::new_from_array([0x06; 32]);
    let mut user_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_y = Pubkey::new_from_array([0x07; 32]);
    let mut user_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_lp = Pubkey::new_from_array([0x08; 32]);
    let mut mint_lp_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::Some(config),
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_lp_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_lp = Pubkey::new_from_array([0x09; 32]);
    let mut user_lp_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_lp,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_lp_account.data_as_mut_slice(),
    )
    .unwrap();

    // Create the instruction data
    let amount: u64 = 100;
    let min: u64 = 90;
    let instruction_data = SwapInstructionData {
        is_x: 1,
        amount: amount.to_le_bytes(),
        min: min.to_le_bytes(),
    };

    // instruction discriminator = 2
    let mut ser_instruction_data = vec![2];

    // Serialize the instruction data
    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&instruction_data));

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new(vault_x, false),
            AccountMeta::new(vault_y, false),
            AccountMeta::new(user_x, false),
            AccountMeta::new(user_y, false),
            AccountMeta::new(mint_lp, false),
            AccountMeta::new(user_lp, false),
            AccountMeta::new(config, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (user, user_account),
            (mint_x, mint_x_account),
            (mint_y, mint_y_account),
            (vault_x, vault_x_account),
            (vault_y, vault_y_account),
            (user_x, user_x_account),
            (user_y, user_y_account),
            (mint_lp, mint_lp_account),
            (user_lp, user_lp_account),
            (config, config_account),
            (system_program, system_account),
            (token_program, token_account),
        ],
        &[Check::success()],
    );
}

#[test]
fn test_withdraw() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();
    let (token_program, token_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let authority = Pubkey::new_from_array([0x00; 32]);

    let user = Pubkey::new_from_array([0x01; 32]);
    let user_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let seed: u64 = 1;
    let fee: u16 = 500;

    let mint_x = Pubkey::new_from_array([0x02; 32]);
    let mut mint_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_y = Pubkey::new_from_array([0x03; 32]);
    let mut mint_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_y_account.data_as_mut_slice(),
    )
    .unwrap();

    // Config account.
    let (config, config_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(CONFIG_SEED.as_bytes()), &seed.to_le_bytes()],
        &PROGRAM,
    );
    let mut config_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Config::LEN),
        Config::LEN,
        &PROGRAM.into(),
    );
    let config_state = Config {
        seed: seed.to_le_bytes(),
        authority: *authority.as_array(),
        mint_x: *mint_x.as_array(),
        mint_y: *mint_y.as_array(),
        fee: fee.to_le_bytes(),
        config_bump: config_bump,
    };
    config_account.data = bytemuck::bytes_of(&config_state).to_vec();

    let vault_x = Pubkey::new_from_array([0x04; 32]);
    let mut vault_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let vault_y = Pubkey::new_from_array([0x05; 32]);
    let mut vault_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: config,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_x = Pubkey::new_from_array([0x06; 32]);
    let mut user_x_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_x,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_x_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_y = Pubkey::new_from_array([0x07; 32]);
    let mut user_y_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_y,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_y_account.data_as_mut_slice(),
    )
    .unwrap();

    let mint_lp = Pubkey::new_from_array([0x08; 32]);
    let mut mint_lp_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::Some(config),
            supply: 100_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_lp_account.data_as_mut_slice(),
    )
    .unwrap();

    let user_lp = Pubkey::new_from_array([0x09; 32]);
    let mut user_lp_account = Account::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: mint_lp,
            owner: user,
            amount: 100_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_lp_account.data_as_mut_slice(),
    )
    .unwrap();

    // Create the instruction data
    let amount: u64 = 100;
    let max_x: u64 = 50;
    let max_y: u64 = 50;
    let instruction_data = WithdrawInstructionData {
        amount: amount.to_le_bytes(),
        max_x: max_x.to_le_bytes(),
        max_y: max_y.to_le_bytes(),
    };

    // instruction discriminator = 3
    let mut ser_instruction_data = vec![3];

    // Serialize the instruction data
    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&instruction_data));

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(mint_x, false),
            AccountMeta::new_readonly(mint_y, false),
            AccountMeta::new(vault_x, false),
            AccountMeta::new(vault_y, false),
            AccountMeta::new(user_x, false),
            AccountMeta::new(user_y, false),
            AccountMeta::new(mint_lp, false),
            AccountMeta::new(user_lp, false),
            AccountMeta::new(config, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (user, user_account),
            (mint_x, mint_x_account),
            (mint_y, mint_y_account),
            (vault_x, vault_x_account),
            (vault_y, vault_y_account),
            (user_x, user_x_account),
            (user_y, user_y_account),
            (mint_lp, mint_lp_account),
            (user_lp, user_lp_account),
            (config, config_account),
            (system_program, system_account),
            (token_program, token_account),
        ],
        &[Check::success()],
    );
}
