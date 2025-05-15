use pinocchio::program_error::ProgramError;

pub mod deposit;
pub mod initialize;
pub mod swap;
pub mod withdraw;

pub use deposit::*;
pub use initialize::*;
pub use swap::*;
pub use withdraw::*;

#[repr(u8)]
pub enum AMMInstruction {
    Initialize,
    Deposit,
    Swap,
    Withdraw,
}

impl TryFrom<&u8> for AMMInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(AMMInstruction::Initialize),
            1 => Ok(AMMInstruction::Deposit),
            2 => Ok(AMMInstruction::Swap),
            3 => Ok(AMMInstruction::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
