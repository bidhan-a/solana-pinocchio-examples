use pinocchio::program_error::ProgramError;

pub mod deposit;
pub mod withdraw;

pub use deposit::*;
pub use withdraw::*;

#[repr(u8)]
pub enum VaultInstruction {
    Deposit,
    Withdraw,
}

impl TryFrom<&u8> for VaultInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(VaultInstruction::Deposit),
            1 => Ok(VaultInstruction::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
