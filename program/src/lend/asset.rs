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
pub struct Assets {

    /// 接收 nft 地址
    pub supply : u64,
    ///
    pub borrow : u64,

    /// 用户
    pub owner : Pubkey,
}

impl Assets {
    pub fn new(&mut self,owner:Pubkey){
        self.owner = owner;
    }
    pub fn supply(&mut self,amount:u64){
        self.supply = self.supply.add(amount);
    }
    pub fn withdraw(&mut self,amount:u64){
        self.supply = self.supply.sub(amount);
    }
    pub fn borrow(&mut self,amount:u64){
        self.borrow = self.borrow.sub(amount);
    }
    pub fn pay_borrow(&mut self,amount:u64){
        self.borrow = self.borrow.sub(amount);
    }
}

/// IsInitialized is required to use `Pack::pack` and `Pack::unpack`
impl IsInitialized for Assets {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for Assets {}
impl Pack for Assets {
    const LEN: usize = 48;
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 48];
        let (
            supply,
            borrow,
            owner,
        ) = mut_array_refs![output, 8, 8, 32];
        *supply = self.supply.to_le_bytes();
        *borrow = self.borrow.to_le_bytes();
        owner.copy_from_slice(self.owner.as_ref());
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Assets, ProgramError> {
        let input = array_ref![input, 0, 48];
        #[allow(clippy::ptr_offset_with_cast)]
            let (
            supply,
            borrow,
            owner,
        ) = array_refs![input, 8, 8, 32];
        Ok(Self {
            supply: u64::from_le_bytes(*supply),
            borrow: u64::from_le_bytes(*borrow),
            owner: Pubkey::new_from_array(*owner),
        })
    }
}