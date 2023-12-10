Staking Contract
Overview

The Staking Contract is a smart contract built on the Elrond blockchain that allows users to stake EGLD tokens and earn rewards over time. The contract provides functionalities for staking, unstaking, claiming rewards, and checking stake-related information.
Constants

    BLOCKS_IN_YEAR: The number of blocks in a year.
    MAX_PERCENTAGE: The maximum percentage value.

Struct: StakingPosition

    stake_amount: The amount of EGLD tokens staked by a user.
    last_action_block: The block number of the user's last staking action.

Implementation of EncodeDefault and DecodeDefault

The EncodeDefault and DecodeDefault traits are implemented for the StakingPosition struct, allowing default encoding and decoding.
Contract Functions
init

    Description: Initializes the Staking Contract.

stake

    Payable: Accepts EGLD tokens.
    Endpoint: Allows users to stake EGLD tokens.
    Logic:
        Checks if the payment amount is greater than 0.
        Retrieves the caller's staking position.
        Claims pending rewards for the user.
        Updates the total staking amount.
        Updates the user's staking position.

unstake

    Endpoint: Allows users to unstake EGLD tokens.
    Parameters: opt_unstake_amount - Optional unstake amount.
    Logic:
        Checks if the user is staked.
        Retrieves the user's staking position.
        Calculates the unstake amount.
        Performs unstaking effects.
        Claims rewards for the user.
        Sends the unstaked amount to the user.

claim_rewards

    Endpoint: Allows users to claim pending rewards.
    Logic:
        Retrieves the user's staking position.
        Claims rewards for the user.

require_user_staked

    Description: Checks if the user is staked.

claim_rewards_for_user

    Description: Claims rewards for a specific user.

calculate_rewards

    Description: Calculates rewards based on staking position and time.

update_total_staking

    Description: Updates the total staking amount.

decrease_total_staking

    Description: Decreases the total staking amount.

View Functions
get_stake_percentage

    Description: Calculates the stake percentage for a user.

calculate_rewards_for_user

    Description: Calculates rewards for a specific user.

Storage Mappers
staked_addresses

    Description: Stores the set of staked addresses.

staking_position

    Description: Stores the staking position for each address.

total_staking

    Description: Stores the total staking amount.

Usage

    Initialization:
        Call the init function.

    Staking:
        Call the stake function with EGLD payment.

    Unstaking:
        Call the unstake function with an optional unstake amount.

    Claiming Rewards:
        Call the claim_rewards function.

    Checking Stake Percentage:
        Call the get_stake_percentage function.

    Calculating Rewards for a User:
        Call the calculate_rewards_for_user function.

Important Notes

    Users must stake EGLD before unstaking or claiming rewards.
    Rewards are calculated based on staking duration and the user's stake percentage.

Feel free to customize and extend the contract based on your specific requirements.