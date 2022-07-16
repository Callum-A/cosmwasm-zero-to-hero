# Part Fifteen - Spring Cleaning

I must admit, I've been neglecting some items. This chapter aims to remedy them and provide a couple other options to improve the contract.

You've probably noticed some warnings when running console commands such as `cargo test`.

Let's start correcting them:

## Lints

### Cargo Clippy

Clippy provides a linter and if you've been pushing your project to github you'll notice a workflow defined which calls this command. The workflow will probably be failing unless you've been keeping ontop of this yourself.

So here's how you can run it yourself:

```
cargo clippy --all-targets -- --D warnings
```

Run that from your root repo and you'll get an output with any complaints:

```
error: useless use of `vec!`
   --> src/contract.rs:366:42
    |
366 |         let mut deps = mock_dependencies(&vec![]);
    |                                          ^^^^^^^ help: you can use a slice directly: `&[]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#useless_vec

error: useless use of `vec!`
   --> src/contract.rs:368:37
    |
368 |         let info = mock_info(ADDR1, &vec![]);
    |                                     ^^^^^^^ help: you can use a slice directly: `&[]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#useless_vec

error: redundant clone
   --> src/contract.rs:430:59
    |
430 |         let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    |                                                           ^^^^^^^^ help: remove this
    |
note: this value is dropped without further use
   --> src/contract.rs:430:55
    |
430 |         let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    |                                                       ^^^^
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#redundant_clone

error: useless use of `vec!`
   --> src/contract.rs:406:42
    |
406 |         let mut deps = mock_dependencies(&vec![]);
    |                                          ^^^^^^^ help: you can use a slice directly: `&[]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#useless_vec

error: useless use of `vec!`
   --> src/contract.rs:408:37
    |
408 |         let info = mock_info(ADDR1, &vec![]);
    |                                     ^^^^^^^ help: you can use a slice directly: `&[]`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#useless_vec

error: could not compile `cw-starter` due to 5 previous errors
```

It shows tips on how to resolve them so I'm not going to cover that (and there's a lot). Instead I am simply going to correct them myself. The corrected code is available in `code/cw-starter` however I recommend you resolves these lints yourself and get in the habit of running `cargo clippy` yourself.

### Cargo FMT

Cargo also provides a formatter, `cw-template`'s workflows also check for this.

To format your project simply run from the root:

```
cargo fmt
```

I do this often when I'm developing and always before pushing to my Git.

### Final Notes on Linting

After this you should be able to run all these commands without warnings:

```
cargo clippy --all-targets -- --D warnings
cargo fmt
cargo test
cargo wasm
cargo schema
```

Rust provides some of the best tooling for linting, you'd be a fool not to use it!

## General Improvements

You may notice in our `execute` code we often clone the strings as the values are "moved" in Rust terms. We can fix this by using references where we store strings in storage. For example keys for our maps can become (in `src/state.rs`):

```rust
// Previous code omitted
pub const POLLS: Map<&str, Poll> = Map::new("polls");
pub const BALLOTS: Map<(Addr, &str), Ballot> = Map::new("ballots");
```

These changes are visible under the `code/cw-starter` part of this chapter, effectively where we previously used the `String` directly we simply need to prefix it with `&` for example:

```rust
// Before
let poll = POLLS.may_load(deps.storage, poll_id)?;
BALLOTS.update(
    deps.storage,
    (info.sender, poll_id),
    |ballot| -> StdResult<Ballot> {
        // Action code omitted
    },
)?
// After
let poll = POLLS.may_load(deps.storage, &poll_id)?;
BALLOTS.update(
    deps.storage,
    (info.sender, &poll_id),
    |ballot| -> StdResult<Ballot> {
        // Action code omitted
    },
)?
```

Again there's alot of changes to be made but your IDE should error where it hasn't been updated and the fixes are simple. This should be easy for you now!

Don't forget to check `clippy` and `fmt` after performing these upgrades!

## Functionality Improvements

Now these changes won't be reflected under `code/cw-template` as these are optional and subjective.

It's down to you to now expand functionality of your contract! Here are some ideas:

1. Charge users a 1 token fee for creating a poll to prevent spam.
    - Hints
        - The expected token denom must be stored in the global config items.
        - `info` has a variable called `info.funds` which contains a `Vec<Coin>` think what we need to do to check that funds are sent and are of the correct type?
        - Add an error if funds are not sent!
        - Investigate how we can send funds in our unit-tests, look at the second parameter (currently an empty vec) in our `mock_info` calls.
        - Test for this error!
2. Add a boolean field on a poll so it can be closed. User's cannot vote on a closed poll, throw an error if they attempt too.
    - Hints
        - We will need to modify our `Poll` type.
        - We will need to mofiy our `Vote` execute endpoint to error if a poll is closed.
        - We will also need to add a new endpoint allowing the creator of a poll to close it.
        - The global admin stored in config should also be able to close polls.
        - Error if anyone else tries to close a poll.
        - Add test cases for all use-cases listed above.
3. When a poll is closed return the 1 token fee to the creator so it acts as a deposit and not a payment.
    - Hints
        - You will need to look into CosmosMsgs and the `.add_message()` method on your `Response`.
        - This is getting complex, feel free to ask for help in discords!

## Closing Notes on the Smart Contract Part of the Tutorial

Well we've reached the end of the first part of contract development: next up building a dApp and providing a usable front-end.

Actually there'll be one more chapter teaching you how to setup a local deployment to make testing your contract much much easier with your front end.

I'd like to thank you all for making it this far and the support I have received recently. If this tutorial has helped even just one person it makes it so worth it.

Thanks again all, see you in the next part :).
