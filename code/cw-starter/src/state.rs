use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
// 05 State
// - use cw_storage_plus::Item;
// + use cw_storage_plus::{Item, Map};
use cw_storage_plus::{Item, Map};

// 05 State
// - #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// - pub struct State {
// -     pub count: i32,
// -     pub owner: Addr,
// - }
// + #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// + pub struct Config {
// +     pub admin: Addr,
// + }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

// 05 State
// + #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// + pub struct Poll {
// +     pub creator: Addr,
// +     pub question: String,
// +     pub options: Vec<(String, u64)>,
// + }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

// 05 State
// + #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// + pub struct Ballot {
// +     pub option: String,
// + }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

// 05 State
// - pub const STATE: Item<State> = Item::new("state");
// + pub const CONFIG: Item<Config> = Item::new("config");
pub const CONFIG: Item<Config> = Item::new("config");

// 05 State
// + pub const POLLS: Map<String, Poll> = Map::new("polls");
// A map with a String key and Poll value.
// The key will be a UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");

// + pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
