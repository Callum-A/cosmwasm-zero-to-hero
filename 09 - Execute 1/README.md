# Part Nine - Execute 1

This is the first part where we will be writing core execution logic.

So we implemented our messages last time, let's take a look at `src/contract.rs` and more specifically that unimplemented execution function.

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}
```

Looks very empty right? Well as your contracts grow this will be a lot of the logic. This function effectively becomes a switch case (in Rust more specifically a matching case) redirecting appropriate messages to appropriate function calls.

So let's start implementing this redirection! To start with we want to `match` the `msg` variable for all its different types. Luckily Rust supports enum pattern matching.

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut, // removed _ as needed later
    env: Env, // removed _ as needed later
    info: MessageInfo, // removed _ as needed later
    msg: ExecuteMsg, // remove _ as used now
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => unimplemented!(),
        ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
    }
}
```

So let's talk through what this does, well to start with it determines what type of message the `msg` variable is. It then destructures it into the variables that make up the message (the parts within the `{}`). Those variables line up with the ones we defined in the message in the last chapter.

And then the `=>` means it calls the function, in our case we simply use the `unimplemented!` macro as a placeholder.

So let's start writing our first implementation. So I like to follow the following pattern for my `execute` and `query` messages. The entry point is `execute` but each message calls its function often named after the message. For example for `CreatePoll` the function name would be `execute_create_poll` now at a glance I can tell that this function executes the create a poll message.

I also like the pass all the `deps`, `env`, and `info` variables to each execute function, as I may need them for logic within the body, for example accessing storage.

We also need the relevant information, for our `CreatePoll` example, this is the `poll_id`, `question` and `options`. So this is how our `execute` body is now going to look:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut, // removed _ as needed later
    env: Env, // removed _ as needed later
    info: MessageInfo, // removed _ as needed later
    msg: ExecuteMsg, // remove _ as used now
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

For now your IDE may be screaming at you that this function does not exist, so let's create it! Let's place it after the `execute` function but before the `query` function.

```rust
// Previous code omitted
fn execute_create_poll(
    deps: DepsMut,
    _env: Env, // _env as we won't be using it
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    unimplemented!()
}
// Following code omitted
```

We define the function with the corresponding types for the parameter and also match the return type of the root `execute` function. This allows us to return our function calls directly in the match case. We've handled the type `Result<Response, ContractError>` before in our `instantiate`.

So let's start implementing the code right? First I want to introduce you to the `ContractError` type. So we need an error state, as the developer of this polling service, we can say the max amount of options is 10. Any more and we will throw an error.

Now we can implement this error. Head to the `src/errors.rs` file. It should currently look like this:

```rust
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
```

It contains some basic errors already, the ability to construct an error from a Cosmwasm `StdError` and a base `Unauthorized` error.

Let's add our new error, call it something clear. I'm going with `TooManyOptions`. Now let's discuss the `#[error("...")]` part above each error. This is what the errors string value will be, it allows support of custom arguments but for now, we can simply hard code our string to something like `"Too many poll options"`. This is what our new `ContractError` enum will look like:

```rust
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Too many poll options")]
    TooManyOptions {},
}
```

Alright with that added it's time to head back to `src/contract.rs` and implement it. What we need to do is check the options `Vec` length, and IF it is over 10, return an error. How we achieve this with `Result` is by using `Err()` and contained within `Err` will be our new `ContractError` here is how that looks:

```rust
fn execute_create_poll(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }
    unimplemented!()
}
```

We've now performed some basic validation, we can now begin constructing our `Poll`. If you remember correctly we store options on our `Poll` as a `Vec<(String, u64)>` whereas we only pass options as a `Vec<String>` as all votes start at 0.

So we need to construct a new `Vec` before we can create a poll.

We also need to work out who the creator is, we can use `info` for this by using `info.sender`.

The question is already provided for us so we don't need to perform any more processing on that.

Poll ID is only used to store the poll in our storage map, so no more processing is needed on that side.

So how do we create our new options `Vec`, this is how I did it:

```rust
fn execute_create_poll(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    unimplemented!()
}
```

What this does is create a mutable `Vec<(String, u64)>` (this allows us to modify it). It then iterates over options storing the current option in the `option` variable.

It then adds a tuple of `(option, 0)` to the back of the new mutable `Vec`.

This means we now have our options in the correct format.

Let's construct our `Poll` but firstly we need to import the struct and the `Map` into storage. This can be seen below:

```rust
// We can add it to our existing config imports
use crate::state::{Config, CONFIG, Poll, POLLS};
```

Alright here's how constructing our `Poll` looks:

```rust
fn execute_create_poll(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        question,
        options: opts
    };

    unimplemented!()
}
```

As hinted above it uses `info.sender` for the creator. It simply uses the `question` parameter with no further processing for its question. And it uses our newly created `opts` `Vec` for the options.

So how do we use this? Well, firstly we need to store it under a key, in our case `poll_id`. We also need to give it a value to store, in our case `poll`. It also requires `deps.storage` to store values for our contract. So putting this all together how does this look?

```rust
fn execute_create_poll(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        question,
        options: opts
    };

    POLLS.save(deps.storage, poll_id, &poll)?;

    unimplemented!()
}
```

Remember we need to pass it a reference of our `poll` by prefixing it with the `&` character. What this function call effectively does is it takes our `deps.storage` and stores `poll` under the key `poll_id`. The suffixing `?` means it automatically unwraps the result throwing any errors, this is standard practice for storage as if it fails here we want to tell the user.

And now we need to remove that pesky `unimplemented!` so let's just return a response similar to how we did in `instantiate`.

```rust
fn execute_create_poll(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        question,
        options: opts
    };

    POLLS.save(deps.storage, poll_id, &poll)?;

    Ok(Response::new())
}
```

Looks very familiar, doesn't it? But there we go, we have now created an endpoint to store our polls in storage.

In the next part, we will cover the `Vote` execute endpoint.

## Follow Up Exercises

1. Think about what attributes we could add to our response.
    - Hints
        - What information do you think would be useful to the user?
