# Part Ten - Execute 2

Alright so last time we implemented the `execute_create_poll` function. Now we are going to do the same for voting.

So open up `src/contract.rs` and take a look. Your main `execute` call should look something like this:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(deps, env, info, poll_id, question, options),
        ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
    }
}
```

So I explained my structure in the last chapter, let's write the code for calling the `execute_vote` function. We'll pass all parameters such as `deps`, `env`, `info` as well as our `poll_id` and `vote`.

This should look something like:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(deps, env, info, poll_id, question, options),
        ExecuteMsg::Vote { poll_id, vote } => execute_vote(deps, env, info, poll_id, vote),
    }
}
```

Now let's implement the function definition so our IDE stops screaming at us. I place it below my `execute_create_poll` function definition:

```rust
// Previous code omitted
fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    unimplemented!()
}
// Following code omitted
```

This should all feel familiar, it's why I'm no longer explaining it so in depth. You should start to pick up these patterns very quickly.

Alright this function is more complex so let's take it slow and think of what we need to do:

1. We need to a load a poll, check if it exists.
2. We need to update a user's ballot and if they have already voted take one away from their old vote option.
3. We need to increment the user's new vote option.
4. We need to save the poll's new state as well as the ballot.

This is going to get into some real nitty-gritty rust, so you've been warned.

Firstly we need to load the poll, but a poll might not exist so we need to use the `may_load` storage helper. Which wraps it in an `Option` in case it doesn't:

```rust
// Previous code omitted
fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;
    unimplemented!()
}
// Following code omitted
```

We have to clone the `poll_id` as we plan to use it multiple times. I'll go over how we can optimise this in a later chapter.

Do you remember the match case we used? We can also use this over `Option` to cater for both states, `null` or a `Poll` value.

We can do this by:

```rust
// Previous code omitted
fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;

    match poll {
        Some(mut poll) => unimplemented!(),
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
}
// Following code omitted
```

As you can see if a poll exists (the `Some` branch) we take that poll and store it in a now mutable `poll` variable, which will we use later.

If a poll does not exist we simply error, for now I placed a placeholder `Unauthorized` error.

So what's the next step, let's deal with the user's ballot and if it already exists. This is going to get complex so I'll show you it and talk you through it step by step.

```rust
// Previous code omitted
fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;

    match poll {
        Some(mut poll) => {
            Some(mut poll) => { // The poll exists
            BALLOTS.update(
                deps.storage,
                (info.sender, poll_id.clone()),
                |ballot| -> StdResult<Ballot> {
                    match ballot {
                        Some(ballot) => {
                            // We need to revoke their old vote
                            // Find the position
                            let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();
                            // Decrement by 1
                            poll.options[position_of_old_vote].1 -= 1;
                            // Update the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                        None => {
                            // Simply add the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                    }
                },
            )?;
        },
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
}
// Following code omitted
```

So firstly we call update on the `BALLOTS` storage `Map`, we use `deps.storage` as standard and we create the composite key we defined in `src/state.rs`. We then define an action that will be called on the matching key.
You can think of this action as an inline function or a lambda.

This action takes in both scenarios where a ballot exists and where on does not. We need to handle both.

So we use another trusty match case! The `Some` option is where a ballot already exists and is more complex as we need to decrement their old vote by one. We'll talk through this first.

How I handle decrementing their vote count is by finding the position of it by using the old ballot (now contained in the `ballot` variable). To do this we use another action in the `position` function, `position` simply returns an `Option<usize>` of the position in the `Vec` or `None` if it is not. We conditionally return the position where `option.0 == ballot.option`. The first part (`option.0`) effectively translates to the first element of the options tuple we defined (`(String, u64)`) so we want to match it with whatever ballot was casted before. This will find us the index the vote count is stored in.

We simply unwrap the `Option` value it returns to assert success.

The next line simply uses this index, now stored in `position_of_old_vote` to decrement the count by 1. The `.1` part means it is accessing the second part of the tuple (the `u64` part).

We then simply want to return a new `Ballot` for the key to be set to, we do this by defining a new `Ballot` with the user specified vote and returning it using the `Ok` syntax to assert success.

The other part is much simpler, if a user has not voted before, we can simply create their new ballot as we do not have to worry about their old vote.

I promise this is as complex as it gets!

So now we need to increment the count of their new vote by 1, some of you may have realised we can use the same pattern we use to decrement their old vote. Let me show you:

```rust
// Previous code omitted
fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;

    match poll {
        Some(mut poll) => {
            Some(mut poll) => { // The poll exists
            BALLOTS.update(
                deps.storage,
                (info.sender, poll_id.clone()),
                |ballot| -> StdResult<Ballot> {
                    match ballot {
                        Some(ballot) => {
                            // We need to revoke their old vote
                            // Find the position
                            let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();
                            // Decrement by 1
                            poll.options[position_of_old_vote].1 -= 1;
                            // Update the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                        None => {
                            // Simply add the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                    }
                },
            )?;

            // Find the position of the new vote option and increment it by 1
            let position = poll
                .options
                .iter()
                .position(|option| option.0 == vote);
            if position.is_none() {
                return Err(ContractError::Unauthorized {});
            }
            let position = position.unwrap();
            poll.options[position].1 += 1;
        },
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
}
// Following code omitted
```

That code is pretty much the same as the `position_of_old_vote` snippet except it's finding the new vote position and incrementing its count by 1. Makes sense right?

Then finally we need to actually save the new `poll` variable as we have since updated it, we can do this with a similar call:

```rust
// Previous code omitted
fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    let poll = POLLS.may_load(deps.storage, poll_id.clone())?;

    match poll {
        Some(mut poll) => {
            Some(mut poll) => { // The poll exists
            BALLOTS.update(
                deps.storage,
                (info.sender, poll_id.clone()),
                |ballot| -> StdResult<Ballot> {
                    match ballot {
                        Some(ballot) => {
                            // We need to revoke their old vote
                            // Find the position
                            let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();
                            // Decrement by 1
                            poll.options[position_of_old_vote].1 -= 1;
                            // Update the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                        None => {
                            // Simply add the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                    }
                },
            )?;

            // Find the position of the new vote option and increment it by 1
            let position = poll
                .options
                .iter()
                .position(|option| option.0 == vote);
            if position.is_none() {
                return Err(ContractError::Unauthorized {});
            }
            let position = position.unwrap();
            poll.options[position].1 += 1;

            // Save the update
            POLLS.save(deps.storage, poll_id, &poll)?;
            Ok(Response::new())
        },
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
}
// Following code omitted
```

We save it under the `poll_id` key and give it a reference to our `poll` variable. This should seem very familiar to saving an `Item` except we have to specify a key to store it under as it can support multiple!

I also snuck in the return call of `Ok`. I hope that didn't throw anyone off, but that's it we're done with this function!

And that means you made it to the end of another chapter!

## Follow Up Exercises

1. Implement a better error for if a poll does not exist. Call it `PollNotFound`
    - Hints
        - Go back a chapter and check how we implemented the limit for options.
2. Add better attributes on the `Ok` call, this will help with testing in the next chapter
    - Hints
        - What would be useful to the user?
