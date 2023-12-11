# Staking Contract

Overview

The Staking Contract is a smart contract built on the Elrond blockchain that allows users to stake EGLD tokens and earn rewards over time. The contract provides functionalities for staking, unstaking, claiming rewards, and checking stake-related information.

Procedure

1) build contract --> mxpy contract build
2) Select wallet --> <https://devnet-wallet.multiversx.com>
3) Create account
4) Create key.pem file
5) Ask for EGLD in a faucet <https://devnet-wallet.multiversx.com/faucet>
6) Deploy contract

mxpy --verbose contract deploy --bytecode=./output/staking-contract.wasm --recall-nonce --pem=./wallet.pem --gas-limit=40000000 --send --outfile="upgrade-devnet.interaction.json" --proxy=https://devnet-gateway.multiversx.com --chain=D

Result

New user account --> erd1hgulazcqjqcrj8mguurzvr4s4p5rkrsl50ca7sw8jhq8ak93esssyt5x8f

New smart constract account --> erd1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq6gq4hu
Transaction Fee --> 0.01825465 xEGLD
gas used --> 2097245

Challenge

● Users can deposit their EGLD tokens into the contract to become a
staker.
● Assume rewards are distributed based on a global speed. For
example, 0.0003 EGLD are distributed per second among all users.
● Users earn rewards in proportion to their stake. These rewards are
continuously distributed (of course, they are kept at the staking smart
contract).
● Users can withdraw their staked EGLD or claim their rewards at any
time.

Constants

    BLOCKS_IN_YEAR: The number of blocks in a year.
    MAX_PERCENTAGE: The maximum percentage value.

Struct: StakingPosition

    stake_amount: The amount of EGLD tokens staked by a user.
    last_action_block: The block number of the user's last staking action.

Implementation of EncodeDefault and DecodeDefault

The EncodeDefault and DecodeDefault traits are implemented for the StakingPosition struct, allowing default encoding and decoding. They are necessary for correctly decoding. More info:

 <https://docs.multiversx.com/developers/data/defaults>

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

**We assume that before staking more tokens, we need to claims pending rewards. That is not on the callenge.**
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

**We use the pattern check-effects-interaction to avoid the re-entrancy attack.**

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
