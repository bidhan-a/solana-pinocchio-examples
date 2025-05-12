use mollusk_svm::result::Check;
use mollusk_svm::Mollusk;
use pinocchio_counter::constants::COUNTER_SEED;
use pinocchio_counter::instruction::CreateInstructionData;
use pinocchio_counter::state::Counter;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use pinocchio_counter::ID;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const PAYER: Pubkey = pubkey!("9vCdf2rh7hA7JdSVV1LEbJGFDNLjk1KHGTZW1wSRN6vC");

pub const INITIAL_COUNT: u64 = 42;

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_counter");
    mollusk
}

#[test]

fn test_create() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let maker = Pubkey::new_from_array([0x01; 32]);
    let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let (counter, counter_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(COUNTER_SEED.as_bytes()), &maker.to_bytes()],
        &PROGRAM,
    );
    let counter_account = Account::new(0, 0, &system_program);

    // Create the instruction data
    let instruction_data = CreateInstructionData {
        count: INITIAL_COUNT.to_le_bytes(),
        bump: counter_bump,
    };

    // instruction discriminator = 0
    let mut ser_instruction_data = vec![0];

    // Serialize the instruction data
    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&instruction_data));

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new(counter, true),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let expected_counter_state = Counter {
        maker: *maker.as_array(),
        count: (INITIAL_COUNT).to_le_bytes(),
        bump: counter_bump,
    };
    let ser_expected_counter_state = bytemuck::bytes_of(&expected_counter_state);

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (maker, maker_account),
            (counter, counter_account),
            (system_program, system_account),
        ],
        &[
            Check::success(),
            Check::account(&counter)
                .data(ser_expected_counter_state)
                .build(),
        ],
    );
}

#[test]

fn test_increment() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let maker = Pubkey::new_from_array([0x01; 32]);
    let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let (counter, counter_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(COUNTER_SEED.as_bytes()), &maker.to_bytes()],
        &PROGRAM,
    );
    let mut counter_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Counter::LEN),
        Counter::LEN,
        &PROGRAM.into(),
    );
    let counter_state = Counter {
        maker: *maker.as_array(),
        count: INITIAL_COUNT.to_le_bytes(),
        bump: counter_bump,
    };
    counter_account.data = bytemuck::bytes_of(&counter_state).to_vec();

    // instruction discriminator = 1
    let ser_instruction_data = vec![1];

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new(counter, true),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let expected_counter_state = Counter {
        maker: *maker.as_array(),
        count: (INITIAL_COUNT + 1).to_le_bytes(),
        bump: counter_bump,
    };
    let ser_expected_counter_state = bytemuck::bytes_of(&expected_counter_state);

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (maker, maker_account),
            (counter, counter_account),
            (system_program, system_account),
        ],
        &[
            Check::success(),
            Check::account(&counter)
                .data(ser_expected_counter_state)
                .build(),
        ],
    );
}

#[test]

fn test_decrement() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let maker = Pubkey::new_from_array([0x01; 32]);
    let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let (counter, counter_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(COUNTER_SEED.as_bytes()), &maker.to_bytes()],
        &PROGRAM,
    );
    let mut counter_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Counter::LEN),
        Counter::LEN,
        &PROGRAM.into(),
    );
    let counter_state = Counter {
        maker: *maker.as_array(),
        count: INITIAL_COUNT.to_le_bytes(),
        bump: counter_bump,
    };
    counter_account.data = bytemuck::bytes_of(&counter_state).to_vec();

    // instruction discriminator = 2
    let ser_instruction_data = vec![2];

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new(counter, true),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let expected_counter_state = Counter {
        maker: *maker.as_array(),
        count: (INITIAL_COUNT - 1).to_le_bytes(),
        bump: counter_bump,
    };
    let ser_expected_counter_state = bytemuck::bytes_of(&expected_counter_state);

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (maker, maker_account),
            (counter, counter_account),
            (system_program, system_account),
        ],
        &[
            Check::success(),
            Check::account(&counter)
                .data(ser_expected_counter_state)
                .build(),
        ],
    );
}

#[test]

fn test_delete() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let maker = Pubkey::new_from_array([0x01; 32]);
    let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let (counter, counter_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(COUNTER_SEED.as_bytes()), &maker.to_bytes()],
        &PROGRAM,
    );
    let mut counter_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Counter::LEN),
        Counter::LEN,
        &PROGRAM.into(),
    );
    let counter_state = Counter {
        maker: *maker.as_array(),
        count: INITIAL_COUNT.to_le_bytes(),
        bump: counter_bump,
    };
    counter_account.data = bytemuck::bytes_of(&counter_state).to_vec();

    // instruction discriminator = 3
    let ser_instruction_data = vec![3];

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(maker, true),
            AccountMeta::new(counter, true),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (maker, maker_account),
            (counter, counter_account),
            (system_program, system_account),
        ],
        &[
            Check::success(),
            Check::account(&counter).lamports(0).build(),
            Check::account(&counter).closed().build(),
        ],
    );
}
