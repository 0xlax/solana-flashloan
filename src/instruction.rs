use solana_program::program_error::ProgramError;

use crate::error::FlashloanError::{InvalidInstruction,InstructionPackError};

pub enum FlashloanProgramInstruction {

    InitFlashloanProgram{},

    ExecuteOperation{
        amount: u64,
    }

    // Account borrows from the reserve by invoking flashload instruction
    FlashloanCall{
        amount: u64,
            execute_operation_ix_data: Vec<u8>
    },
    
}

impl FlashloanProgramInstruction {
    pub fn unpack(input :&[u8]) -> Result<Self, ProgramError> {
        let (tag, _rest) = input.split_first().ok_or(InvalidInstruction)?;
        
        Ok(match tag {
            0 => Self::InitFlashloanProgram {},
                1 => {
                    let (amount, _rest) = Self::unpack_u64(rest)?;

				Self::ExecuteOperation{
					amount
                },
                2 => {
                    let (amount, execute_operation_ix_data_slice) = Self::unpack_u64(rest)?;
                    let execute_operation_ix_data = execute_operation_ix_data_slice.to_vec();
                    
                    Self::FlashloanCall {
                        amount,
                        execute_operation_ix_data,
                    }
                },
            _ => return Err(InvalidInstruction.into()),
        })
    }
    fn unpack_u64(input: &[u8]) -> Result<u64, ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(InstructionPackError)?;
            Ok((amount, rest))
        } else {
            Err(InstructionPackError.into())
        }
    }

	fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if !input.is_empty() {
            let (amount, rest) = input.split_at(1);
            let amount = amount
                .get(..1)
                .and_then(|slice| slice.try_into().ok())
                .map(u8::from_le_bytes)
                .ok_or(InstructionPackError)?;
            Ok((amount, rest))
        } else {
            Err(InstructionPackError.into())
        }
    }

	/// Packs a [LendingInstruction](enum.LendingInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match *self {
            Self::InitFlashloanProgram {} => {
                buf.push(0);
            }
            Self::ExecuteOperation{
                amount,
            } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
			Self::MyFlashloanCall {
                amount,
				execute_operation_ix_data
            } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
				buf.append(&execute_operation_ix_data);
            }
        }
        buf
    }
}