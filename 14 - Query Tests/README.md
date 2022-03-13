# Part Fourteen - Query Tests

Alright we're back to testing, let's start taking a look at our testing section in `src/contract.rs`.

Let's start with our `AllPoll` query. Call the test `test_query_all_polls`:

```rust
// Previous code omitted
#[test]
fn test_query_all_polls() {

}
// Following code omitted
```

This should be second nature now. Similarly to our execute tests we need to instantiate the contract, lets copy over that code from another test:

```rust
// Previous code omitted
#[test]
fn test_query_all_polls() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
// Following code omitted
```

We also need some polls to query! Lets pull in some execute code from our earlier tests, lets create two polls:

```rust
// Previous code omitted
#[test]
fn test_query_all_polls() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a second poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_2".to_string(),
        question: "What's your colour?".to_string(),
        options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
}
// Following code omitted
```

We now have two polls so we expect two polls when we query our route.

Lets start writing some new code, firstly we need to define a `msg` using our `QueryMsg` enum:

```rust
// Previous code omitted
#[test]
fn test_query_all_polls() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a second poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_2".to_string(),
        question: "What's your colour?".to_string(),
        options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Query
    let msg = QueryMsg::AllPolls {};
}
// Following code omitted
```

Looks very similar to our execute messages except we use the `QueryMsg` enum, and of course we want all polls so we use the `AllPoll`.

Next up I'm going to cover the other side of why we need to encode our responses to `Binary`, now we need to decode them back to our structs.

First lets get the raw binary and store it in a `bin` variable.

```rust
// Previous code omitted
#[test]
fn test_query_all_polls() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a second poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_2".to_string(),
        question: "What's your colour?".to_string(),
        options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Query
    let msg = QueryMsg::AllPolls {};
    let bin = query(deps.as_ref(), env, msg).unwrap();
}
// Following code omitted
```

There is also a slight difference here, instead of using `deps.as_mut` which allows the dependencies to be mutable so we can change our contract state. We use `deps.as_ref` as queries cannot change the state of a contract.

That seems simply enough, we now have the raw binary stored, but now you ask how do we decode it? Well Cosmwasm provides a helper `from_binary` for that. However when doing this we need to define a type for our variable so that `from_binary` knows what to expect.

This is what it looks like:

```rust
// Previous code omitted
#[test]
fn test_query_all_polls() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a second poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_2".to_string(),
        question: "What's your colour?".to_string(),
        options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Query
    let msg = QueryMsg::AllPolls {};
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllPollsResponse = from_binary(&bin).unwrap();
}
// Following code omitted
```

Without the `AllPollsResponse` it would throw an error, `from_binary` also returns an `StdResult` so we `unwrap` it as we assume success.

So now we have our struct we defined, we expect two polls to be contained in the `res.polls` variable. Lets assert this length using the `Vec` `.len()` method. Heres the code:

```rust
// Previous code omitted
#[test]
fn test_query_all_polls() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a second poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_2".to_string(),
        question: "What's your colour?".to_string(),
        options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    };
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Query
    let msg = QueryMsg::AllPolls {};
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllPollsResponse = from_binary(&bin).unwrap();
    assert_eq!(res.polls.len(), 2);
}
// Following code omitted
```

That's our first test written and an overview of how to unit-test query routes. Run `cargo test` in your terminal to show that it is working!

Lets now test the `Poll` route, there are two main cases:

1. We query for a poll and it exists, expect the response to be `Some(poll)`.
2. We query for a poll and it does not exist, expect the response to be `None`.

So let's outline our test, call it `test_query_poll`:

```rust
// Previous code omitted
#[test]
fn test_query_poll() {

}
// Following code omitted
```

Similarly to above we need to instantiate and create a poll, let's copy over some code to do that:

```rust
// Previous code omitted
#[test]
fn test_query_poll() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
// Following code omitted
```

This should now be second nature, I don't need to explain what this does now. If you are confused double back to our instantiate test and execute test chapters.

So lets query for a poll that exists, here's what the message looks like:

```rust
// Previous code omitted
#[test]
fn test_query_poll() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for the poll that exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_1".to_string(),
    };
}
// Following code omitted
```

We simply need to give it the `poll_id` we defined above as we are checking for a poll that exists here.

Now we need to go through the `Binary` process again, but remember to type it with the correct response variable. If you've forgotten the type it's called `PollResponse`.

```rust
// Previous code omitted
#[test]
fn test_query_poll() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for the poll that exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_1".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PollResponse = from_binary(&bin).unwrap();
}
// Following code omitted
```

This looks the same as our previous test just a difference in that `res` type when decoding `Binary`.

So now we need to assert a value, luckily the `Option` type has a method called `is_some()` which returns true when a value is present. We expect a value to present. We need to assert this value, we can do this using the following:

```rust
// Previous code omitted
#[test]
fn test_query_poll() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for the poll that exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_1".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PollResponse = from_binary(&bin).unwrap();
    // Expect a poll
    assert!(res.poll.is_some());
}
// Following code omitted
```

`assert!` is similar to `assert_eq!` except it expects the value to be true. It just makes the code simpler to write.

So that's the first case covered, now let's copy that code expect change a few things. The first is to change `poll_id` to something that does not exist:

```rust
// Previous code omitted
#[test]
fn test_query_poll() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for the poll that exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_1".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PollResponse = from_binary(&bin).unwrap();
    // Expect a poll
    assert!(res.poll.is_some());

    // Query for the poll that does not exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_not_exist".to_string(),
    };
}
// Following code omitted
```

I simply called my `poll_id`, `some_id_not_exist` to make it clear that it does not exist.

Next we can copy the `Binary` decoding lines.

```rust
// Previous code omitted
#[test]
fn test_query_poll() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for the poll that exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_1".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PollResponse = from_binary(&bin).unwrap();
    // Expect a poll
    assert!(res.poll.is_some());

    // Query for the poll that does not exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_not_exist".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PollResponse = from_binary(&bin).unwrap();
}
// Following code omitted
```

Next up we need to assert a value again. Luckily `Option` also has an `is_none()` method which returns true when an `Option` is None. We can assert that now:

```rust
// Previous code omitted
#[test]
fn test_query_poll() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for the poll that exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_1".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PollResponse = from_binary(&bin).unwrap();
    // Expect a poll
    assert!(res.poll.is_some());

    // Query for the poll that does not exists
    let msg = QueryMsg::Poll {
        poll_id: "some_id_not_exist".to_string(),
    };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PollResponse = from_binary(&bin).unwrap();
    // Expect none
    assert!(res.poll.is_none());
}
// Following code omitted
```

Boom 2/3 tests done, run `cargo test` in your terminal to see it in action!

Now onto our next test, lets call it `test_query_vote`:

```rust
// Previous code omitted
#[test]
fn test_query_vote() {

}
// Following code omitted
```

Alright lets outline our test scenarios:

1. Query for a vote that does exist, expect a value.
2. Query for a vote that does not exist, expect none.

We also need to make sure we instantiate our contract, create a poll and create a vote.

Lets bring over some code we've used before to do this:

```rust
// Previous code omitted
#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
}
// Following code omitted
```

I went fast over this as we've seen all this before.

Let's outline our new query message, lets cover our vote that exists case, meaning we are querying for the vote on `poll_id` `some_id_1` and from address `ADDR1`.

```rust
// Previous code omitted
#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for a vote that exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR1.to_string(),
    };
}
// Following code omitted
```

We use the `ADDR1` helper variable we defined right at the very start of the testing chapter.

Now lets add the `Binary` code, remember to set the type of `res` to `VoteResponse`:

```rust
// Previous code omitted
#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for a vote that exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR1.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
}
// Following code omitted
```

Very familiar just with a slight change, awesome work!

Now we're going to use that helper `assert!` with the `is_some()` method, again familiar so here it is:

```rust
// Previous code omitted
#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for a vote that exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR1.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    // Expect the vote to exist
    assert!(res.vote.is_some());
}
// Following code omitted
```

This test is half done now, let's cover the other case!

So we need to use a `poll_id` that does not exist, and hell why not lets also use a random address. We defined `ADDR2` for this use case earlier. Here's the message:

```rust
// Previous code omitted
#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for a vote that exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR1.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    // Expect the vote to exist
    assert!(res.vote.is_some());

    // Query for a vote that does not exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_2".to_string(),
        address: ADDR2.to_string(),
    };
}
// Following code omitted
```

As you can see we never create a poll with ID `some_id_2` and we also never created a vote using `ADDR2`.

Next let's copy over the decoding code:

```rust
// Previous code omitted
#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for a vote that exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR1.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    // Expect the vote to exist
    assert!(res.vote.is_some());

    // Query for a vote that does not exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_2".to_string(),
        address: ADDR2.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
}
// Following code omitted
```

All very familiar, next up we need to `assert!` the value using the `is_none()` method, again we've used it before so here it is:

```rust
// Previous code omitted
#[test]
fn test_query_vote() {
    let mut deps = mock_dependencies(&vec![]);
    let env = mock_env();
    let info = mock_info(ADDR1, &vec![]);
    // Instantiate the contract
    let msg = InstantiateMsg { admin: None };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a poll
    let msg = ExecuteMsg::CreatePoll {
        poll_id: "some_id_1".to_string(),
        question: "What's your favourite Cosmos coin?".to_string(),
        options: vec![
            "Cosmos Hub".to_string(),
            "Juno".to_string(),
            "Osmosis".to_string(),
        ],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create a vote
    let msg = ExecuteMsg::Vote {
        poll_id: "some_id_1".to_string(),
        vote: "Juno".to_string(),
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Query for a vote that exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_1".to_string(),
        address: ADDR1.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    // Expect the vote to exist
    assert!(res.vote.is_some());

    // Query for a vote that does not exists
    let msg = QueryMsg::Vote {
        poll_id: "some_id_2".to_string(),
        address: ADDR2.to_string(),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: VoteResponse = from_binary(&bin).unwrap();
    // Expect the vote to not exist
    assert!(res.vote.is_none());
}
// Following code omitted
```

There we have it, our last test is complete. Run `cargo test` in your terminal to see all our tests running. Satisfying right?

Next chapter will be a conclusion chapter for the smart contract side of this tutorial outlining some improvements and problems that I expect you to now have the knowledge to resolve. All in all this isn't the best implementation but you should now have the knowledge to go "Huh, I can probably do XYZ this way instead!". The world really is your oyster with smart contract development.

## Follow Up Exercises

1. If you added any custom query messages, such as retrieving the config add a test covering that method.
    - Hints
        - What do you expect to be returned?
2. Add a test case for the `AllPolls` route when no polls have been created.
    - Hints
        - What do we expect?
