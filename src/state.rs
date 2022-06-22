use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct FlashloanProgram {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub flashloan_token_account_pubkey: Pubkey,
}
