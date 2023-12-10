#![no_std]
use multiversx_sc::codec::{EncodeDefault, DecodeDefault};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const BLOCKS_IN_YEAR: u64 = 60 * 60 * 24 * 365 / 6;
pub const MAX_PERCENTAGE: u64 = 10_000;
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
        let stake_mapper = self.staking_position(&caller);

        let new_user = self.staked_addresses().insert(caller.clone());
        let mut staking_pos = if !new_user {
            stake_mapper.get()
        } else {
            let current_block = self.blockchain().get_block_epoch();
            StakingPosition {
                stake_amount: BigUint::zero(),
                last_action_block: current_block,
            }
        };

        self.claim_rewards_for_user(&caller, &mut staking_pos);
        self.update_total_staking(&payment_amount);
        staking_pos.stake_amount += payment_amount;
        stake_mapper.set(&staking_pos);
    }

    #[endpoint]
    fn unstake(&self, opt_unstake_amount: OptionalValue<BigUint>) {
        // Fase de Checks
        let caller = self.blockchain().get_caller();
        self.require_user_staked(&caller);
    
        let stake_mapper = self.staking_position(&caller);
        let mut staking_pos = stake_mapper.get();
    
        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => staking_pos.stake_amount.clone(),
        };
        require!(
            unstake_amount > 0 && unstake_amount <= staking_pos.stake_amount,
            "Invalid unstake amount"
        );
        // Fase de Effects
        staking_pos.stake_amount -= &unstake_amount;
    
        if staking_pos.stake_amount > 0 {
            stake_mapper.set(&staking_pos);
        } else {
            stake_mapper.clear();
            self.staked_addresses().swap_remove(&caller);
        }
        self.decrease_total_staking(&unstake_amount);
    
        // Fase de Interactions
        self.claim_rewards_for_user(&caller, &mut staking_pos);
        self.send().direct_non_zero_egld(&caller, &unstake_amount);
    }

     //Private Functions

     fn require_user_staked(&self, user: &ManagedAddress) {
        require!(self.staked_addresses().contains(user), "Must stake first");
    }

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

    fn update_total_staking(&self, amount: &BigUint<Self::Api>) {
        let current_total_staking = self.total_staking().get();
        let new_total_staking = current_total_staking + amount;
        self.total_staking().set(&new_total_staking);
    }

    fn decrease_total_staking(&self, amount: &BigUint<Self::Api>) {
        let current_total_staking = self.total_staking().get();
    
        require!(amount <= &current_total_staking, "Cannot subtract more than total staking");
    
        let new_total_staking = current_total_staking - amount;
        self.total_staking().set(&new_total_staking);
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