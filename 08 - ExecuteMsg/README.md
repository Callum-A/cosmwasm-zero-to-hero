# Part Eight - ExecuteMsg

Alright it's time to head back to a file we're familiar with. `src/msg.rs`.

You've written messages before, so I have faith you'll pick this up pretty quick. We're going to add the backbone of our execute structure today. Well not the backbone more the start of it.

We're not implementing the logic of these purely the messages which lead to the logic.

These two tasks are `Creating a Poll` and `Voting on a Poll`.

So let's open `src/msg.rs` and take a look at `ExecuteMsg`. It should look pretty empty:

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}
// Following code omitted
```

Alright let's draw some differences to `InstantiateMsg`. `ExecuteMsg` is an enum so it can contain a whole array of values. To put in plain terms, we only have one type of `InstantiateMsg` but we already have two types of `ExecuteMsg` (`Create a Poll` and `Vote on a Poll`).

So how does this change how we write code for it? Not much really, let's run you through an example for `Creating a Poll`.

First let's think of what information we need when creating a poll. It might be worth going back to look at the `State` chapter or looking in `src/state.rs` to refresh yourself. Let's talk it through part-by-part:

-   Creator - This can be inferred using the helper `info` variable I explained during the `Instantiate` chapter. We don't need this in our `CreatePoll` message.
-   Question - User specified, definitely needs to be in the message. So we need a `String` question in our `CreatePoll` message.
-   Options - User specified, definitely needs to be in the message. So we need a `Vec<String>` in our message. (As all vote counts start at 0).
-   Poll ID - Generated client side as it is a UUID. So we also need a `String` ID.

So there's our three fields, so how does that look in code, well here it is if you haven't written it already:

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePoll {
        poll_id: String,
        question: String,
        options: Vec<String>,
    }
}
// Following code omitted
```

It's that simple, so let's go through the same process for casting a vote.

-   Voter - This can be inferred using the helper `info` variable.
-   Poll ID - We need to know what poll the user is voting on, must be passed via the client. So we need a `String` field.
-   Vote - What option is the user voting for. We need a `String` field for this too.

The code for this looks like:

```rust
// Previous code omitted
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
// Following code omitted
```

There we go! In this section you've learned how to define different types of `ExecuteMsg`! Next section we will cover actually implementing the logic for them.

## Follow Up Exercises

1. How would you go about writing an `ExecuteMsg` for deleting a poll?
    - Hints
        - What information would you need? Think about what we need to retrieve it from our `Map` in storage.
2. What if a user wanted to revoke their vote from a poll?
    - Hints
        - As above think about what information you need to retrieve their `Ballot` from the `Map` in storage.
