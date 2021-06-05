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
pub struct Farm {
    ///
    pub lp_address : Pubkey,
    ///
    pub nonce : u8,
    ///
    pub reward_address : Pubkey,
    ///
    pub owner : Pubkey,
    ///
    pub amount : u64
}

impl Farm {
    pub fn new(&mut self,lp_address:Pubkey,reward_address:Pubkey,owner:Pubkey,nonce:u8){
        self.lp_address = lp_address;
        self.owner = owner;
        self.nonce = nonce;
        self.reward_address = reward_address;
    }
    pub fn stake(&mut self,amount:u64){
        self.amount = self.amount.add(amount);
    }
    pub fn unstake(&mut self,amount:u64){
        self.amount = self.amount.sub(amount);
    }
}

/// IsInitialized is required to use `Pack::pack` and `Pack::unpack`
impl IsInitialized for Farm {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for Farm {}
impl Pack for Farm {
    const LEN: usize = 105;
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 105];
        let (
            lp_address,
            nonce ,
            reward_address ,
            owner,
            amount
        ) = mut_array_refs![output, 32, 1, 32,32,8];
        lp_address.copy_from_slice(self.lp_address.as_ref());
        nonce[0] = self.nonce;
        reward_address.copy_from_slice(self.reward_address.as_ref());
        owner.copy_from_slice(self.owner.as_ref());
        *amount = self.amount.to_le_bytes()
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Farm, ProgramError> {
        let input = array_ref![input, 0, 105];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            lp_address,
            nonce ,
            reward_address ,
            owner,
            amount
        ) = array_refs![input, 32, 1, 32,32,8];
        Ok(Self {
            lp_address: Pubkey::new_from_array(*lp_address),
            nonce:nonce[0],
            reward_address: Pubkey::new_from_array(*reward_address),
            owner: Pubkey::new_from_array(*owner),
            amount: u64::from_le_bytes(*amount),
        })
    }
}