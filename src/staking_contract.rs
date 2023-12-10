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
     fn claim_rewards_for_user(
        &self,
        user: &ManagedAddress,
        staking_pos: &mut StakingPosition<Self::Api>,
    ) {
        let reward_amount = self.calculate_rewards(staking_pos);
        let current_block = self.blockchain().get_block_nonce();
        staking_pos.last_action_block = current_block;

        if reward_amount > 0 {
            self.send().direct_egld(user, &reward_amount);
        }
    }

    fn calculate_rewards(&self, staking_position: &StakingPosition<Self::Api>) -> BigUint {

        let current_total_staking = self.total_staking().get();
        let reward_per_round = BigUint::from(300u64);
        let current_block = self.blockchain().get_block_nonce();
        let user_stake = &staking_position.stake_amount;

        if current_block <= staking_position.last_action_block {
            return BigUint::zero();
        }

        let user_share = user_stake / &current_total_staking;

        let block_diff: u64 = current_block - staking_position.last_action_block;
        let seconds_elapsed = block_diff * 6;

        &reward_per_round * seconds_elapsed * user_share
    }


    //view functions
    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;
    


    //storage

    #[storage_mapper("totalStaking")]
    fn total_staking(&self) -> SingleValueMapper<BigUint<Self::Api>>;
}