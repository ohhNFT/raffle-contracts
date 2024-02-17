use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    ensure, Addr, Coin, Decimal, Env, HexBinary, StdError, StdResult, Storage, Timestamp,
};

use cw_storage_plus::{Item, Map};
use utils::state::AssetInfo;

pub const ATLAS_DAO_STARGAZE_TREASURY: &str =
    "stars1jyg4j6t4kdptgsx6q55mu0f434zqcfppkx6ww9gs7p4x7clgfrjq29sgmc";
pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);
pub const DECIMAL_FRACTIONAL: u128 = 1_000_00;
pub const MAX_TICKET_NUMBER: u32 = 100000; // The maximum amount of tickets () that can be in a raffle
pub const MINIMUM_RAFFLE_DURATION: u64 = 1; // default minimum raffle duration, in blocks
pub const MINIMUM_RAFFLE_TIMEOUT: u64 = 120; // The raffle timeout is a least 2 minutes
pub const NOIS_AMOUNT: u128 = 500000;
pub const RAFFLE_INFO: Map<u64, RaffleInfo> = Map::new("raffle_info");
pub const RAFFLE_TICKETS: Map<(u64, u32), Addr> = Map::new("raffle_tickets");
pub const STATIC_RAFFLE_CREATION_FEE: u128 = 1; // default static tokens required to create raffle
pub const USER_TICKETS: Map<(&Addr, u64), u32> = Map::new("user_tickets");

#[cw_serde]
pub struct Config {
    /// The name of the contract
    pub name: String,
    /// The owner address of the contract
    pub owner: Addr,
    /// The address to recieve all fees generated by the contract
    pub fee_addr: Addr,
    /// The most recent raffle id
    pub last_raffle_id: Option<u64>,
    /// The minimum duration, in seconds, in which users can buy raffle tickets
    pub minimum_raffle_duration: u64,
    /// The minimum interval, in seconds, during which users can provide entropy to the contract.
    pub minimum_raffle_timeout: u64,
    // The maximum number of participants available to participate in any 1 raffle
    pub max_tickets_per_raffle: Option<u32>,
    /// A % cut of all raffle fee's generated to go to the fee_addr
    pub raffle_fee: Decimal,
    /// locks the contract from new raffles being created
    pub lock: bool,
    /// The nois_proxy contract address
    pub nois_proxy_addr: Addr,
    /// The expected fee token denomination of the nois_proxy contract
    pub nois_proxy_coin: Coin,
    pub creation_coins: Vec<Coin>,
}

impl Config {
    pub fn validate_fee(&self) -> Result<(), StdError> {
        ensure!(
            self.raffle_fee <= Decimal::one(),
            StdError::generic_err("The Total Fee rate should be lower than 1")
        );
        Ok(())
    }
}

#[cw_serde]
pub struct NoisProxy {
    // The price to pay the proxy for randomness
    pub price: Coin,
    // The address of the nois-proxy contract deployed onthe same chain as this contract
    pub address: Addr,
}

// RAFFLES

pub fn load_raffle(storage: &dyn Storage, raffle_id: u64) -> StdResult<RaffleInfo> {
    RAFFLE_INFO.load(storage, raffle_id)
}

#[cw_serde]
pub struct RaffleInfo {
    pub owner: Addr, // owner/admin of the raffle
    pub assets: Vec<AssetInfo>, // assets being raffled off 
    pub raffle_ticket_price: AssetInfo, // cost per ticket
    pub number_of_tickets: u32, // number of tickets purchased 
    pub randomness: Option<HexBinary>, // randomness seed provided by nois_proxy
    pub winner: Option<Addr>, // winner is determined here
    pub is_cancelled: bool, 
    pub raffle_options: RaffleOptions, 
}

#[cw_serde]
pub enum RaffleState {
    Created,
    Started,
    Closed,
    Finished,
    Claimed,
    Cancelled,
}

impl std::fmt::Display for RaffleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaffleState::Created => write!(f, "created"),
            RaffleState::Started => write!(f, "started"),
            RaffleState::Closed => write!(f, "closed"),
            RaffleState::Finished => write!(f, "finished"),
            RaffleState::Claimed => write!(f, "claimed"),
            RaffleState::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Queries the raffle state
/// This function depends on the block time to return the RaffleState.
/// As actions can only happen in certain time-periods, you have to be careful when testing off-chain
/// If the chains stops or the block time is not accurate we might get some errors (let's hope it never happens)
pub fn get_raffle_state(env: Env, raffle_info: RaffleInfo) -> RaffleState {
    if raffle_info.is_cancelled {
        RaffleState::Cancelled
    } else if env.block.time < raffle_info.raffle_options.raffle_start_timestamp {
        RaffleState::Created
    } else if env.block.time
        < raffle_info
            .raffle_options
            .raffle_start_timestamp
            .plus_seconds(raffle_info.raffle_options.raffle_duration)
    {
        RaffleState::Started
    } else if env.block.time
        < raffle_info
            .raffle_options
            .raffle_start_timestamp
            .plus_seconds(raffle_info.raffle_options.raffle_duration)
            .plus_seconds(raffle_info.raffle_options.raffle_timeout)
        || raffle_info.randomness == None
    {
        RaffleState::Closed
    } else if raffle_info.winner.is_none() {
        RaffleState::Finished
    } else {
        RaffleState::Claimed
    }
}

#[cw_serde]
pub struct RaffleOptions {
    pub raffle_start_timestamp: Timestamp, // If not specified, starts immediately
    pub raffle_duration: u64, // length, in seconds the duration of a raffle
    pub raffle_timeout: u64, // the cooldown time between the end of ticket sales & winner being determined.
    pub comment: Option<String>, // raffle description
    pub max_ticket_number: Option<u32>, // max amount of tickets able to be purchased
    pub max_ticket_per_address: Option<u32>, // max amount of tickets able to bought per address
    pub raffle_preview: u32, // ? 
}

#[cw_serde]
pub struct RaffleOptionsMsg {
    pub raffle_start_timestamp: Option<Timestamp>,
    pub raffle_duration: Option<u64>,
    pub raffle_timeout: Option<u64>,
    pub comment: Option<String>,
    pub max_ticket_number: Option<u32>,
    pub max_ticket_per_address: Option<u32>,
    pub raffle_preview: Option<u32>,
}

impl RaffleOptions {
    pub fn new(
        env: Env,
        assets_len: usize,
        raffle_options: RaffleOptionsMsg,
        config: Config,
    ) -> Self {
        Self {
            raffle_start_timestamp: raffle_options
                .raffle_start_timestamp
                .unwrap_or(env.block.time)
                .max(env.block.time),
            raffle_duration: raffle_options
                .raffle_duration
                .unwrap_or(config.minimum_raffle_duration)
                .max(config.minimum_raffle_duration),
            raffle_timeout: raffle_options
                .raffle_timeout
                .unwrap_or(config.minimum_raffle_timeout)
                .max(config.minimum_raffle_timeout),
            comment: raffle_options.comment,
            max_ticket_number: raffle_options.max_ticket_number,
            max_ticket_per_address: raffle_options.max_ticket_per_address,
            raffle_preview: raffle_options
                .raffle_preview
                .map(|preview| {
                    if preview >= assets_len.try_into().unwrap() {
                        0u32
                    } else {
                        preview
                    }
                })
                .unwrap_or(0u32),
        }
    }

    pub fn new_from(
        current_options: RaffleOptions,
        assets_len: usize,
        raffle_options: RaffleOptionsMsg,
        config: Config,
    ) -> Self {
        Self {
            raffle_start_timestamp: raffle_options
                .raffle_start_timestamp
                .unwrap_or(current_options.raffle_start_timestamp)
                .max(current_options.raffle_start_timestamp),
            raffle_duration: raffle_options
                .raffle_duration
                .unwrap_or(current_options.raffle_duration)
                .max(config.minimum_raffle_duration),
            raffle_timeout: raffle_options
                .raffle_timeout
                .unwrap_or(current_options.raffle_timeout)
                .max(config.minimum_raffle_timeout),
            comment: raffle_options.comment.or(current_options.comment),
            max_ticket_number: raffle_options
                .max_ticket_number
                .or(current_options.max_ticket_number),
            max_ticket_per_address: raffle_options
                .max_ticket_per_address
                .or(current_options.max_ticket_per_address),
            raffle_preview: raffle_options
                .raffle_preview
                .map(|preview| {
                    if preview >= assets_len.try_into().unwrap() {
                        0u32
                    } else {
                        preview
                    }
                })
                .unwrap_or(current_options.raffle_preview),
        }
    }
}
