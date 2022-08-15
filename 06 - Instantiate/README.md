# Part Six - Instantiate

Firstly I mentioned how I broke the build last time! Let's fix that.

## Spring Cleaning

So how did I break the build? Well, we renamed the `STATE` variable to `CONFIG` and this `STATE` variable is imported in `examples/schema.rs`.

It no longer exists so Rust throws an error!

Let's go to `examples/schema.rs` and fix this.

To fix simply find the line that looks like this:

```rust
use cw_starter::state::State;
```

And replace it with:

```rust
use cw_starter::state::Config;
```

Next up find the line:

```rust
// Previous code omitted
export_schema(&schema_for!(State), &out_dir);
// Following code omitted
```

And replace it with:

```rust
export_schema(&schema_for!(Config), &out_dir);
```

What this does is output a JSON schema for our config. You can do this for any structure that implements `JsonSchema`.

Let's also generate a schema for our `Poll` and `Ballot` structs.

Firstly let's import them.

Modify the line we just correct from:

```rust
use cw_starter::state::Config;
```

To:

```rust
use cw_starter::state::{Config, Poll, Ballot};
```

We now import all three of our defined structs.

Let's now generate a schema for `Poll` and `Ballot`.

Below the export schema line we just corrected add:

```rust
// Previous code omitted
export_schema(&schema_for!(Config), &out_dir);
export_schema(&schema_for!(Poll), &out_dir);
export_schema(&schema_for!(Ballot), &out_dir);
// Following code omitted
```

This has now fixed the errors we introduced in the previous part. Verify this by running:

```bash
# This will generate new JSON files, for our config, poll and ballot
cargo schema
# Still 0 tests but should pass
cargo test
# Will generate the wasm under target/
cargo wasm
```

## Instantiating a Contract

When contracts are stored on the chain they must be instantiated. I cover storing contracts on a chain in a later section.

Instantiating a contract is like creating an object in other languages, however, it is achieved by a special message.

This message is an `InstantiateMsg` located under `src/msg.rs`.

Let's add something to it!

### The InstantiateMsg

When a user instantiates the contract, they can either specify an admin or leave it empty. 

How can we achieve this in Rust? Use the `Option` structure. For example `Option<String>` can either be a String or null.

This is perfect for our use case! You may wonder why we do not use `Option<Addr>`. This is due to validation; we want to check what the user is passing us is a valid address. I will show you how to do this in code.

Firstly let's add this option to our `InstantiateMsg`.

Currently, our `InstantiateMsg` looks like this:

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub val: String,
}
```

Boring! Let's update it to:

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}
```

Now let's use this in our contract!

### Instantiation

Alright, let's open `src/contract.rs`, this is where the magic happens.

Towards the top you should see something like:

```rust
// Previous code omitted
/*
const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 */

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}
// Following code omitted
```

We're going to correct this now and implement it! Currently, it simply throws an error saying it's not implemented.

To start with, let's use a standard called `cw2`, it allows contracts to store version and name as you look at the commented-out code lines.

Firstly let's uncomment those:

```rust
// Previous code omitted
const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}
// Following code omitted
```

Now let's store them using `cw2`, first, we have to import a helper function `set_contract_version`:

```rust
// Previous code omitted
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// Following code omitted
```

Then we need to use it in our instantiate method body:

```rust
// Previous code omitted
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut, // ensure you remove the preceeding _. _deps -> deps
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    unimplemented!()
}
// Following code omitted
```

As we have modified the instantiate arguments, now's an ideal time to talk through them:

-   `deps` - The dependencies, this contains your contract storage, the ability to query other contracts and balances, and some API functionality.
-   `env` - The environment, contains contract information such as its address, block information such as current height and time, as well as some optional transaction info.
-   `info` - Message metadata, contains the sender of the message (`Addr`) and the funds sent with it a `Vec<Coin>`.
-   `msg` - The `InstantiateMsg` you define in `src/msg.rs`.

So we set some contract metadata but we still haven't implemented any logic. Alright, let's create our `Config` struct and store it. Make sure you import the struct itself and the storage:

```rust
use crate::state::{Config, CONFIG};
```

Alright, we're ready to set our `Config` firstly let's use this instantiate message to work out who our admin is going to be. We can do that using:

```rust
// Previous code omitted
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo, // removed _
    msg: InstantiateMsg, // removed _
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}
// Following code omitted
```

Alright, that's a lot of code. Let's talk you through it step by step.

So the first two lines after `set_contract_version` unwrap our `Option` field and if it is `null` sets it to `info.sender.to_string()` which is the message sender's address as a string.

Rust features a lot of unwrapping of errors and providing defaults, so I recommend you understand this pattern.

In this case, if we gave it `null` in our instantiate as the admin, it would be set to the sender. Otherwise, it would use whatever value we passed it.

It then validates the address by passing it as a `&str` to the `deps.api.addr_validate()` function. This validates if an address is valid and throws an error otherwise. We handle this error by proceeding with the call with a `?` this automatically unwraps and throws the error if one is given. This means an `Invalid Address` error will be thrown back to the user if they provide an invalid one.

The next line creates a `Config` struct with our now validated admin address as the admin. We have to clone the validated address to avoid moving values.

The line following that stores it in our `CONFIG` storage. (Ensure you have imported `CONFIG` from `state.rs`). It does this by calling it with `deps.storage` which is our contracts storage and giving it the address of our newly created config variable. It does this by preceding it with the `&` character.

The final line is our return line indicated by no `;`. This returns a success using the `Ok` and `Result` structure.

Within the `Ok` structure, we create a response using various builder methods. We add two attributes, which for simple understanding are similar to HTTP headers. Get into a habit of writing good attributes as they help a lot with providing metadata to the front end. In this case, we add two, one of which tells the user what 'endpoint' they called and the other tells the user who the admin of the contract is.

We have implemented the first entry point of our contract! In the next section, we will implement tests for it! Tests are vital for smart contracts.
