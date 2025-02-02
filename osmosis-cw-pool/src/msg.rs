use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use osmosis_std::types::osmosis::incentives::MsgCreateGauge;
use white_whale::pool_network::asset::AssetInfo;

#[cw_serde]
pub struct InstantiateMsg {
    pub white_whale_pool: String,
    pub after_pool_created: Option<AfterPoolCreated>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Checks if the swap amount exceeds the minimum_receive. This message  is to be called
    /// internally by the contract.
    AssertMinimumReceive {
        asset_info: AssetInfo,
        prev_balance: Uint128,
        minimum_receive: Uint128,
        receiver: String,
    },
    /// Checks if the swap amount falls behind the maximum_receive. This message is to be called
    /// internally by the contract.
    AssertMaximumReceive {
        asset_info: AssetInfo,
        prev_balance: Uint128,
        maximum_receive: Uint128,
        receiver: String,
    },
}

#[cw_serde]
pub enum SudoMsg {
    /// SetActive sets the active status of the pool.
    SetActive { is_active: bool },
    /// SwapExactAmountIn swaps an exact amount of tokens in for as many tokens out as possible.
    /// The amount of tokens out is determined by the current exchange rate and the swap fee.
    /// The user specifies a minimum amount of tokens out, and the transaction will revert if that amount of tokens
    /// is not received.
    SwapExactAmountIn {
        sender: String,
        token_in: Coin,
        token_out_denom: String,
        token_out_min_amount: Uint128,
        swap_fee: Decimal,
    },
    /// SwapExactAmountOut swaps as many tokens in as possible for an exact amount of tokens out.
    /// The amount of tokens in is determined by the current exchange rate and the swap fee.
    /// The user specifies a maximum amount of tokens in, and the transaction will revert if that amount of tokens
    /// is exceeded.
    SwapExactAmountOut {
        sender: String,
        token_in_denom: String,
        token_in_max_amount: Uint128,
        token_out: Coin,
        swap_fee: Decimal,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// GetSwapFee returns the pool's swap fee, based on the current state.
    /// Pools may choose to make their swap fees dependent upon state
    /// (prior TWAPs, network downtime, other pool states, etc.)
    /// This is intended to be fee that is collected by liquidity providers.
    /// If the contract provider wants to collect fee for itself, it should implement its own fee collection mechanism.
    #[returns(GetSwapFeeResponse)]
    GetSwapFee {},

    /// Returns whether the pool has swaps enabled at the moment
    #[returns(IsActiveResponse)]
    IsActive {},

    /// GetTotalShares returns the total number of LP shares in the pool

    /// GetTotalPoolLiquidity returns the coins in the pool owned by all LPs
    #[returns(TotalPoolLiquidityResponse)]
    GetTotalPoolLiquidity {},

    /// Returns the spot price of the 'base asset' in terms of the 'quote asset' in the pool,
    /// errors if either baseAssetDenom, or quoteAssetDenom does not exist.
    /// For example, if this was a UniV2 50-50 pool, with 2 ETH, and 8000 UST
    /// pool.SpotPrice(ctx, "eth", "ust") = 4000.00
    #[returns(SpotPriceResponse)]
    SpotPrice {
        quote_asset_denom: String,
        base_asset_denom: String,
    },

    /// CalcOutAmtGivenIn calculates the amount of tokenOut given tokenIn and the pool's current state.
    /// Returns error if the given pool is not a CFMM pool. Returns error on internal calculations.
    #[returns(CalcOutAmtGivenInResponse)]
    CalcOutAmtGivenIn {
        token_in: Coin,
        token_out_denom: String,
        swap_fee: Decimal,
    },

    /// CalcInAmtGivenOut calculates the amount of tokenIn given tokenOut and the pool's current state.
    /// Returns error if the given pool is not a CFMM pool. Returns error on internal calculations.
    #[returns(CalcInAmtGivenOutResponse)]
    CalcInAmtGivenOut {
        token_out: Coin,
        token_in_denom: String,
        swap_fee: Decimal,
    },

    /// Returns the config of the contract
    #[returns(Config)]
    GetConfig {},
}

#[cw_serde]
pub struct GetSwapFeeResponse {
    pub swap_fee: Decimal,
}

#[cw_serde]
pub struct IsActiveResponse {
    pub is_active: bool,
}

#[cw_serde]
pub struct TotalPoolLiquidityResponse {
    pub total_pool_liquidity: Vec<Coin>,
}

#[cw_serde]
pub struct SpotPriceResponse {
    pub spot_price: Decimal,
}

#[cw_serde]
pub struct CalcOutAmtGivenInResponse {
    pub token_out: Coin,
}

#[cw_serde]
pub struct CalcInAmtGivenOutResponse {
    pub token_in: Coin,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct AfterPoolCreated {
    pub create_pool_guages: Option<CreatePoolGauges>,
}

#[cw_serde]
pub enum CreatePoolGauges {
    // This works exactly like `gamm`'s.
    DefaultLockableDurations {},
    // Custom guages can be created.
    Custom { msgs: Vec<MsgCreateGauge> },
}

#[cw_serde]
pub struct Config {
    pub white_whale_pool: Addr,
}
