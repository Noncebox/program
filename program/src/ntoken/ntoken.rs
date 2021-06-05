//! All fee information, to be used for validation currently

use crate::error::SwapError;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};
use solana_program::pubkey::Pubkey;

/// Encapsulates all fee information and calculations for swap operations
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ntoken {

    /// 接收 nft 地址
    pub pool_ntoken_address : Pubkey,

    ///
    pub nonce : u8,

    /// 质押者
    pub owner : Pubkey,

    /// token mint address
    pub token_mint_address : Pubkey
}

impl Ntoken {
    pub fn new(&mut self,pool_ntoken_address:Pubkey,mint:Pubkey,nonce:u8){
        self.pool_ntoken_address = pool_ntoken_address;
        self.nonce = nonce;
        self.token_mint_address = mint;
    }
    pub fn create_ntoken(&mut self,mint:Pubkey){
        self.token_mint_address = mint;
    }
}

/// IsInitialized is required to use `Pack::pack` and `Pack::unpack`
impl IsInitialized for Ntoken {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for Ntoken {}
impl Pack for Ntoken {
    const LEN: usize = 97;
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 97];
        let (
            pool_ntoken_address,
            nonce,
            owner,
            token_mint_address,
        ) = mut_array_refs![output, 32,1, 32, 32];
        pool_ntoken_address.copy_from_slice(self.pool_ntoken_address.as_ref());
        nonce[0] = self.nonce;
        owner.copy_from_slice(self.owner.as_ref());
        token_mint_address.copy_from_slice(self.token_mint_address.as_ref());
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Ntoken, ProgramError> {
        let input = array_ref![input, 0, 97];
        #[allow(clippy::ptr_offset_with_cast)]
            let (
            pool_ntoken_address,
            nonce,
            owner,
            token_mint_address,
        ) = array_refs![input, 32,1, 32, 32];
        Ok(Self {
            pool_ntoken_address: Pubkey::new_from_array(*pool_ntoken_address),
            nonce:nonce[0],
            owner: Pubkey::new_from_array(*owner),
            token_mint_address: Pubkey::new_from_array(*token_mint_address),
        })
    }
}