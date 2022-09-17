# Part Eleven - Execute Tests

In the last part, we started adding some actual functionality to our smart contract. Allowing the user to create and vote on polls.

You guessed it, we need to test it!

So let's just quickly walk through the scenarios we want to test:

1. Successful poll creation.
2. Unsuccessful poll creation (more than 10 options).
3. Successful voting on a poll we have not already voted for.
4. Successful voting a poll we have already voted for.
5. Unsuccessful voting a poll, (invalid option).

That sounds like a lot to test! It's not. I'm going to split it into several test functions.

1. `test_execute_create_poll_valid` - tests the valid creation of a poll. AKA we expect no errors.
2. `test_execute_create_poll_invalid` - tests the invalid creation of a poll. AKA we expect errors.
3. `test_execute_vote_valid` - tests the valid creation of votes, for both scenarios of no existing vote and an existing vote. AKA we expect errors.
4. `test_execute_vote_invalid` - tests the invalid creation of votes. AKA we expect errors.

So let's get started!

## Create Poll Valid Tests

So let's define the valid state. We currently only perform one validation check, so our valid state is a poll
with less than 10 options.

Alright, let's define the test function, this was last covered 4 chapters ago so let me remind you. Remember this should be placed within the `mod tests {}` part in `src/contract.rs`.

```rust
#[test]
fn test_execute_create_poll_valid() {

}
```

We also need to import the `execute` entry point of the contract and the `ExecuteMsg` enum:

```rust
// Can import it with the instantiate import
use crate::contract::{instantiate, execute};
use crate::msg::{InstantiateMsg, ExecuteMsg};
```

Right here's some more pre-work we need to do, what you ask? Well, to execute a contract it must be instantiated! So let's copy the instantiate code from our instantiate test and simplify it a bit. Here's what it looks like:

```rust
#[test]
fn test_execute_create_poll_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
```

We don't need to query the attributes as we already cover that in our dedicated instantiate test. We can simply `unwrap` the instantiate to assert success. We also `clone` `info` and `env` as we are going to use these throughout.

Alright now we have instantiated our test contract, Let's start writing some new code!

```rust
#[test]
fn test_execute_create_poll_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // New execute msg
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
}
```

So we defined an execute message of type `CreatePoll` and fill it in with valid values. (Less than 10 options). Sorry if your favorite coin isn't present, you could always add it!

So now we need to call our `execute` function with this info, so make sure you import it. Here's what it looks like:

```rust
#[test]
fn test_execute_create_poll_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // New execute msg
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };

    // Unwrap to assert success
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
```

So we call the `execute` function using our mocked `deps`, `env` and `info` and capture the result.

Unwrapping will throw any errors if it fails. I'd recommend here if you've been following the follow-up exercises to check the `res` variable's attributes, this will make our test more hardy.

I'm merely showing you how so I won't get bogged down with this. For a refresh as to how to go back to chapter 7. For now, this test suits the purpose of this tutorial.

And there we have it, our first test done! You're really getting the hang of this you know!

Run `cargo test` in your terminal to see it in action!

## Create Poll Invalid Tests

Alright, this test needs the same setup, so I'm going to skip the explanation as we've already done it once! Here it is:

```rust
#[test]
fn test_execute_create_poll_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
```

Alright, so what does our invalid message look like? Well, it needs 11 options, so let's get writing:

```rust
#[test]
fn test_execute_create_poll_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite number?".to_string(),
        options: vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
        ],
    };
}
```

I know this code is ugly and hardcoded, my answer is: don't care it's a test case.

So now we need to call execute, but how can we assert an error? We know `unwrap` assumes success, if only there was a way we could unwrap an error. (See where I'm going with this). Enter `unwrap_err`:

```rust
#[test]
fn test_execute_create_poll_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite number?".to_string(),
        options: vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
        ],
    };

    // Unwrap error to assert failure
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
```

There we have it, another test complete!

Run `cargo test` in your terminal to see it in action!

## Vote Valid

Once again here we are writing another test case, let's copy over the code we know we'll need:

```rust
#[test]
fn test_execute_vote_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
```

Similar to what we've seen before. But remember there is another prerequisite.

We need a poll to vote on! So let's copy some more code from our `execute_create_poll_valid` test.

```rust
#[test]
fn test_execute_vote_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
```

We've seen this code before that's why I'm brushing over it. If I lose you here I'd recommend going back over some earlier sections.

Now onto some new code, let's define a `Vote` message. This will be our initial vote (no existing `Ballot`) let's vote for Juno.

```rust
#[test]
fn test_execute_vote_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the vote, first time voting
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "Juno".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
```

Make sure the `poll_id` matches the one above, if you're worried extract it into a `String` variable and use that throughout.

Once again we have a similar execute call, however this time with a vote message. If you added attributes to your `Vote` response, I'd recommend querying them here as we did in chapter 7.

Now there's one more case we need to test, changing our vote.

So what we do, we simply create a new vote message using the same `info` variable we have been using throughout.

Let's change the vote to Osmosis, (sorry Cosmos Hub I still love you though).

```rust
#[test]
fn test_execute_vote_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the vote, first time voting
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "Juno".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Change the vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "Osmosis".to_string(),
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
```

Again ensure `poll_id` matches and you use the same `info` variable we defined at the top of the case.

There we go we now cover the valid states for voting.

Run `cargo test` in your terminal to see it in action!

## Vote Invalid

We're on the last stretch! Long chapter I know but trust me you're doing great.

Before we start let's go through our invalid vote options:

1. Voting on a poll that does not exist.
2. Voting for an option that does not exist.

Let's cover the poll not existing option first.

Copy over that instantiate code!

```rust
#[test]
fn test_execute_vote_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
```

Alright, that's all familiar let's get to the new stuff. Let's take a vote from earlier but not create the poll to create our error case. Here's what mine looks like:

```rust
#[test]
fn test_execute_vote_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the vote, some_id poll is not created yet.
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "Juno".to_string(),
    };
    // Unwrap to assert error
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
}
```

Once again we call the trusty `unwrap_err` to assert failure. This should make sense as the `poll_id` `some_id` is not yet present in our lookup map, meaning the look-up now fails.

Alright, that's our first error case checked off.

Onto the next, but first, we need to create a poll to test this one.
Let's copy over the Cosmos coin poll creation code:

```rust
#[test]
fn test_execute_vote_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the vote, some_id poll is not created yet.
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "Juno".to_string(),
    };
    // Unwrap to assert error
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    // Create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
```

This is all familiar, if not head back and reread the earlier sections.

Right, now we need to create a vote with an invalid option. I'm going to use `DVPN` as mine (other Cosmos coins are available). Here's how the message looks:

```rust
#[test]
fn test_execute_vote_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the vote, some_id poll is not created yet.
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "Juno".to_string(),
    };
    // Unwrap to assert error
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    // Create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Vote on a now existing poll but the option "DVPN" does not exist
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "DVPN".to_string(),
    };
}
```

As you can see the poll now exists. But the option does not, we are now testing for a different error!

So say it with me now! Let's unwrap that error!

```rust
#[test]
fn test_execute_vote_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create the vote, some_id poll is not created yet.
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "Juno".to_string(),
    };
    // Unwrap to assert error
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

    // Create the poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Vote on a now existing poll but the option "DVPN" does not exist
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id".to_string(),
        vote: "DVPN".to_string(),
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
```

And there we have it our last test.

Run `cargo test` in your terminal to see them all running!

I know this was a long chapter, but writing tests is critical for Cosmwasm development. You learn to love it, who doesn't love seeing 20 tests passed and 0 failed?

## Follow Up Exercises

1. Think about what could improve the `valid` test cases.
    - Hints:
        - Think about adding some attributes to their `Ok` response and `assert_eq!` to check that they are as expected.
2. Think about what could improve the `invalid` test cases.
    - Hints:
        - Think about adding some custom contract errors as we did before. Head into `src/errors.rs` and add them.
        - How can we check these errors are what we expect? There are multiple ways too, I use a `match` as we do for different message types. If you're confused ask in the Juno discord dev lounge!
