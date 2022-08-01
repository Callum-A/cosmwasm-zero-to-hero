# Part Twelve - QueryMsg

So you're asking, we can change the state of our contract but what if we want to read some details?
We don't need to pay transaction fees on that, do we?

No, my friend, this is what querying is for!

So as we did with all other sections let's head to where it starts `src/msg.rs`.

Now there should be a type of message that looks very empty and stands out, if you read the title of this chapter.

QueryMsg allows us to query our contract's state, so let's think of some use cases we want for our eventual frontend dApp.

1. Query all polls in a list format.
2. Query a singular poll for a detailed view.
3. Query a user's vote for a given poll.

These are vital for our functionality. Without them, our website would suck and not work.

So like I did with the other message types. Let's think of what we need.

So for querying all polls, we don't particularly need anything. If we wanted to improve UX we may want to add pagination, but for now just grabbing all of them will be fine.

For querying one all we need is the `poll_id` as that's the key of our map.

For querying a user's vote we need a user's address and the `poll_id` of the poll as this is the key for our ballots map. I'll add the caveat here that due to this being a query we no longer have the `info` variable to determine the sender so the user's address must be contained in the message!

Seems simple enough? Let's get to it.

Writing a QueryMsg is the same as writing an ExecuteMsg, you'll pick this up in no time.

Starting with the simplest, `AllPolls` as it has no parameters. This is what it looks like:

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
}
// Following code omitted
```

Boring? I know right.

So the next one, `Poll` needs a `String` parameter for the `poll_id`. Here's what I have for it:

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
    Poll {
        poll_id: String,
    },
}
// Following code omitted
```

This should be feeling very familiar, that's why I'm rattling through these. If this confuses you, I'd recommend rereading the ExecuteMsg chapter to help refresh.

Now for our last one, remember we need the user's address as a parameter as well as a `poll_id`. I like to use a `String` type for a user address so that I can make a validation call on the smart contract side for increased security.

Here's what that looks like:

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPolls {},
    Poll {
        poll_id: String,
    },
    Vote {
        poll_id: String,
        address: String,
    },
}
// Following code omitted
```

We had to remove `QueryMsg::CustomMsg` in this enum, this has broken a helper in the `src/helpers.rs` file we need to remove, we can simply delete all code locations in that file.

We can also remove the following code in `src/msg.rs` as it will not be used:

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}
```

We also need to correct `examples/schema.rs` by changing the import from `src/msg.rs` to:

```rust
// Remove CustomResponse
use cw_starter::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
```

Also, remove the schema export for it:

```rust
export_schema(&schema_for!(CustomResponse), &out_dir);
```

There are our three messages, this should cover the basic functionality of this tutorial. Feel free to add your routes, with smart contract development the world is your oyster.

## Follow Up Exercises

1. Add a query message to retrieve the config of our contract.
    - Hints
        - What arguments might you need if any?
        - Remember our config is stored in the `Item` variable `CONFIG`.
2. What might we have to do if we want to get all votes for a user?
    - Hints
        - This is pretty advanced, you may have to go dipping into the Cosmwasm docs or some advanced projects. I'd recommend looking at `cw3-dao` in the `dao-contracts` repo. [Repo](https://github.com/DA0-DA0/dao-contracts/tree/main/contracts/cw3-dao).
        - If only there was a way to PREFIX query a map with a composite key. (Hint: I'm not being subtle here)
