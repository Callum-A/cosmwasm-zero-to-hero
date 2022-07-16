use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
