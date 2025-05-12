use pinocchio::program_error::ProgramError;

pub mod create;
pub mod decrement;
pub mod delete;
pub mod increment;

pub use create::*;
pub use decrement::*;
pub use delete::*;
pub use increment::*;

#[repr(u8)]
pub enum CounterInstruction {
    Create,
    Increment,
    Decrement,
    Delete,
}

impl TryFrom<&u8> for CounterInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(CounterInstruction::Create),
            1 => Ok(CounterInstruction::Increment),
            2 => Ok(CounterInstruction::Decrement),
            3 => Ok(CounterInstruction::Delete),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
