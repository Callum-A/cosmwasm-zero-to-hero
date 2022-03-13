# Part Thirteen - Query

So we need to a little more work in `src/msg.rs` before continuing, but first some background.

Query messages are handled differently when returning from a `query` you don't return via a `Response` you must define a custom struct which can then be encoded to `Binary`. This value is then decoded when using a query client on the front end side back into a helpful format AKA JSON.

So how do we do this? Well it's all about defining structs and having them derive the correct features via macros.

So lets start by thinking about our `AllPolls` message and what we want to return. Well in a high-level translation we want to return a list of the `Poll` struct. How does this convert to rust? We use a `Vec<Poll>`. So here's how we define the struct.

Let's place it at the bottom of `src/msg.rs` and name it `AllPollsResponse` to make it clear what it is for.

```rust
// Previous code omitted
// Needed macro derivations
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllPollsResponse {
    pub polls: Vec<Poll>,
}
```

Make sure both the struct AND the polls member variable is marked as public via the `pub` keyword.

The derivations support serializing too and from `Binary`.

So we've done one let's think about our next one. `Poll` let's us pick one poll but a poll may not exist. The `Option` wrapper is perfect for this and deserializes nicely to `null` in JSON for our frontend later.

Here's how it looks:

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PollResponse {
    pub poll: Option<Poll>,
}
```

Notice the same derivations, the only difference really is the member variable `poll` which we explained above.

One last response struct, this time for `Vote`. This route has the same problem as before, a user may not have voted before. Let's use `Option` again:

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VoteResponse {
    pub vote: Option<Ballot>,
}
```

Looks very similar to our `PollResponse` but just using `Ballot` instead.

Alright that's all the setup done, let's move to `src/contract.rs` and scroll down to our currently unimplemented query function. It should look something like this:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented()!
}
```

Alright we're going to employ the same structure we used for ExecuteMsg, if you're not familiar with it head back to the corresponding chapter. Here's how it now looks:

```rust
// Note the removal of the _s as we use these variables later
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPolls {} => unimplemented!(),
        QueryMsg::Poll { poll_id } => unimplemented!(),
        QueryMsg::Vote { address, poll_id } => unimplemented!(),
    }
}
```

Let's start by implementing the `AllPoll` route, so lets define a new function `query_all_polls`, self explanatory right? Here's what it looks like:

```rust
// Previous code omitted
fn query_all_polls(deps: Deps, env: Env) -> StdResult<Binary> {
    unimplemented!()
}
// Following code omitted
```

Notice we pass it `deps` and `env` as that's all we need. It also returns the same type `StdResult<Binary>` as `query` so we can return it directly.

Now for a little bit of complex code, as we store our polls as a map, we need to retrieve all the values. In other languages such as Python this is often covered by a `.values()` call. In Cosmwasm it's a little different but I'll run you through:

```rust
// Previous code omitted
fn query_all_polls(deps: Deps, env: Env) -> StdResult<Binary> {
    let polls = POLLS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    unimplemented!()
}
// Following code omitted
```

So firstly we grab a range from polls using `deps.storage` to access it, the two `None`s mean we have no min amount and no max amount of values. `Order::Ascending` simply means return them in ascending order.

Next we have to do a little bit of processing using `map` with an action. This action must return an `StdResult<?>` so that's what the `Ok` is for, the type of `p` is a `Record<Poll>` so we must unwrap it, the `?`, and then it is a tuple. We want the second value the `.1` as the first is a `Vec` of bytes.

Thirdly and finally we have to collect our results into a `Vec` wrapped by `StdResult`. We can use `_` for the type of the `Vec` as this can be inferred as a `Poll` from our map function.

Alright that's the complex code for this chapter covered! Now let's get to returning a value.

So as I implied earlier we have to convert our structs to `Binary`. Make sure you import our `AllPollsResponse` struct.

We need to also use a useful function called `to_binary`, which takes a reference (`&`) to one of our helper return value structs we defined earlier.

Here's what it looks like for `AllPolls`:

```rust
// Previous code omitted
fn query_all_polls(deps: Deps, env: Env) -> StdResult<Binary> {
    let polls = POLLS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPollsResponse { polls })
}
// Following code omitted
```

Now we can go back to our `query` function and call this function.

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPolls {} => query_all_polls(deps, env),
        QueryMsg::Poll { poll_id } => unimplemented!(),
        QueryMsg::Vote { address, poll_id } => unimplemented!(),
    }
}
```

Lets work our way down, the code gets simpler from now I promise.

Define a `query_poll` function that takes `deps`, `env` and `poll_id`:

```rust
fn query_poll(deps: Deps, env: Env, poll_id: String) -> StdResult<Binary> {
    unimplemented!()
}
```

Alright now lets access our storage, I wonder if there's a helper function which lets us return an `Option<Poll>` instead of throwing an error if it is not present.

There is! It's the `may_load` function. Here's what it looks like:

```rust
fn query_poll(deps: Deps, env: Env, poll_id: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, poll_id)?;
}
```

The call looks the exact same as `load` but the `poll` variable is now an `Option<Poll>` instead of a `Poll`.

We can now plug this variable into our `PollResponse` struct, ensuring to encode it to `Binary`:

```rust
fn query_poll(deps: Deps, env: Env, poll_id: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, poll_id)?;
    to_binary(&PollResponse { poll })
}
```

Again very similar to our last one, let's plug it into our `query` function:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPolls {} => query_all_polls(deps, env),
        QueryMsg::Poll { poll_id } => query_poll(deps, env, poll_id),
        QueryMsg::Vote { address, poll_id } => unimplemented!(),
    }
}
```

Onto our last one, this has the exact same problem as `query_poll` we need that `may_load` function again. Let's define the function first:

```rust
fn query_vote(deps: Deps, env: Env, address: String, poll_id: String) -> StdResult<Binary> {
    unimplemented!()
}
```

We also need to validate the `address` variable remember, let's do that now using a helper function called `deps.api.addr_validate` it takes a reference to a `String`. We can call it like:

```rust
fn query_vote(deps: Deps, env: Env, address: String, poll_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    unimplemented!()
}
```

We `unwrap` it to assert success, this gives us the type `Addr` stored under `validate_address` which we can now use to key our map. Lets use the `may_load` function again to get the `Option<Ballot>` we need:

```rust
fn query_vote(deps: Deps, env: Env, address: String, poll_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    let vote = BALLOTS.may_load(deps.storage, (validated_address, poll_id))?;
    unimplemented!()
}
```

All very similar to what we have seen before now, and finally lets encode our result to `Binary`:

```rust
fn query_vote(deps: Deps, env: Env, address: String, poll_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    let vote = BALLOTS.may_load(deps.storage, (validated_address, poll_id))?;

    to_binary(&VoteResponse { vote })
}
```

And there we have it our last query function implemented.

Let's add it to our `query` function:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPolls {} => query_all_polls(deps, env),
        QueryMsg::Poll { poll_id } => query_poll(deps, env, poll_id),
        QueryMsg::Vote { address, poll_id } => query_vote(deps, env, address, poll_id),
    }
}
```

Next chapter we will test these functions individually. For now we're done!

## Follow Up Exercises

1. If you implemented the `Config` query message I set in the follow up exercises before, implement and add the function needed for it to the `query` function:
    - Hints
        - It should only take a `deps` and `env` variable.
        - Remember you will need to define a response struct for it.
