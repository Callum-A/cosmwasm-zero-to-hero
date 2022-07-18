use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

// 12 QueryMsg
// - use cw_starter::msg::{CustomResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
// + use cw_starter::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_starter::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// 06 Instantiate
// - use cw_starter::state::State;
// + use cw_starter::state::{Config, Poll, Ballot};
use cw_starter::state::{Ballot, Config, Poll};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    // 06 Instantiate
    // - export_schema(&schema_for!(State), &out_dir);
    // + export_schema(&schema_for!(Config), &out_dir);
    // + export_schema(&schema_for!(Poll), &out_dir);
    // + export_schema(&schema_for!(Ballot), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema(&schema_for!(Poll), &out_dir);
    export_schema(&schema_for!(Ballot), &out_dir);
    // 12 QueryMsg
    // - export_schema(&schema_for!(CustomResponse), &out_dir);
}
