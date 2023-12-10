#![no_std]
use multiversx_sc::codec::{EncodeDefault, DecodeDefault};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();
#[derive(TypeAbi, TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Debug)]
pub struct StakingPosition<M: ManagedTypeApi> {
    pub stake_amount: BigUint<M>,
    pub last_action_block: u64,
}

impl<M: ManagedTypeApi> EncodeDefault for StakingPosition<M> {
    fn is_default(&self) -> bool {
        self.stake_amount == 0 && self.last_action_block == 0
    }
}

impl<M: ManagedTypeApi> DecodeDefault for StakingPosition<M> {
    fn default() -> Self {
        StakingPosition {
            stake_amount: BigUint::default(),
            last_action_block: 0,
        }
    }
}

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {}

    //endpoint

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value().clone_value();
        require!(payment_amount > 0, "Must pay more than 0");

        let caller = self.blockchain().get_caller();
        self.staking_position(&caller)
            .update(|current_amount| *current_amount += payment_amount);
        self.staked_addresses().insert(caller);
    }

     //Private Functions



    //view functions
    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;
    


    //storage
}