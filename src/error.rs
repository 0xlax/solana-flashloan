use thiserror::Error;
use solana_program::program_error::ProgramError;

pub enum FlashloanError{
    #[error("invalid Instruction")]
    InvalidInstruction,

    #error("Instruction packing error")
    InstructionPackError,

    #error("Invalid Rent Exempt")
    NotRentExempt,
}

impl From<FlashloanError> for ProgramError {
    fn from(e: FlashloanError) -> self {
        ProgramError::Custom(e as u32)
    }
}