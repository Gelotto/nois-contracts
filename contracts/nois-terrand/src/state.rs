use cosmwasm_std::Binary;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use nois_ibc_protocol::Beacon;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub drand_pubkey: Binary,
    /// In test mode the next round calculation is detached from the clock.
    pub test_mode: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

// A map from round number to drand beacon
pub const BEACONS: Map<u64, Beacon> = Map::new("beacons");

pub const TEST_MODE_NEXT_ROUND: Item<u64> = Item::new("test_mode_next_round");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Job {
    pub round: u64,
    // The channel the job came from and we have to send the response to
    pub channel: String,
    // contract address on the app chain
    pub sender: String,
    // contract address on the app chain
    pub callback_id: Option<String>,
}

// Unprocessed jobs that are waiting for the correct round to come in
pub const JOBS: Map<u64, Vec<Job>> = Map::new("jobs");
