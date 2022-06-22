use solana_program::program_error::ProgramError;

use crate::error::MyFlashloanProgramError::{InvalidInstruction,InstructionUnpackError};

pub enum MyFlashloanProgramInstruction {
    Ok(())
}