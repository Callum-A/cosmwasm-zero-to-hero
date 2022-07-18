use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// 13 Query
// + use crate::state:{Poll, Ballot};
use crate::state::{Ballot, Poll};

// 06 Instantiate
// - #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// - #[serde(rename_all = "snake_case")]
// - pub struct InstantiateMsg {
// -     pub val: String,
// - }
// + #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// + #[serde(rename_all = "snake_case")]
// + pub struct InstantiateMsg {
// +     pub admin: Option<String>,
// + }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

// 08 ExecuteMsg
// - #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// - #[serde(rename_all = "snake_case")]
// - pub enum ExecuteMsg {
// -     CustomMsg { val: String },
// - }
// + #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// + #[serde(rename_all = "snake_case")]
// + pub enum ExecuteMsg {
// +     CreatePoll {
// +         poll_id: String,
// +         question: String,
// +         options: Vec<String>,
// +     },
// +     Vote {
// +         poll_id: String,
// +         vote: String,
// +     },
// + }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll {
        poll_id: String,
        question: String,
        options: Vec<String>,
    },
    Vote {
        poll_id: String,
        vote: String,
    },
}

// 12 Query
// - #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// - #[serde(rename_all = "snake_case")]
// - pub enum QueryMsg {
// -     CustomMsg { val: String },
// - }
// + #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// + #[serde(rename_all = "snake_case")]
// + pub enum QueryMsg {
// +     AllPolls {},
// +     Poll {
// +         poll_id: String,
// +     },
// +     Vote {
// +         poll_id: String,
// +         address: String,
// +     },
// + }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
    Poll { poll_id: String },
    Vote { poll_id: String, address: String },
}

// 12 QueryMsg
// - #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// - #[serde(rename_all = "snake_case")]
// - pub struct CustomResponse {
// -     val: String,
// - }

// 13 Query
// + #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// + pub struct AllPollsResponse {
// +     pub polls: Vec<Poll>,
// + }
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllPollsResponse {
    pub polls: Vec<Poll>,
}

// 13 Query
// + #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// + pub struct PollResponse {
// +     pub poll: Option<Poll>,
// + }
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PollResponse {
    pub poll: Option<Poll>,
}

// 13 Query
// + #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// + pub struct VoteResponse {
// +     pub vote: Option<Ballot>,
// + }
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VoteResponse {
    pub vote: Option<Ballot>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
