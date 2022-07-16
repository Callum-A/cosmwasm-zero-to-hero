# Part Seven - Instantiate Test

Alright now we have implemented instantiating our contract let's get to testing it!

Unit tests are vital to smart contract development, every developer should be implementing tests and striving to maximise test coverage. This can be achieved by both Unit-tests and integration tests.

We'll cover both in this series but for now let's get to unit testing.

So let's look at our `contract.rs` file, at the bottom there should be something that looks like:

```rust
#[cfg(test)]
mod tests {}
```

This module is where we will implement our tests, the flag above shows it is for testing.

Let's start filling this out with what we will need, starting with the imports and global helper variables.

```rust
// Previous code omitted
#[cfg(test)]
mod tests {
    use cosmwasm_std::attr; // helper to construct an attribute e.g. ("action", "instantiate")
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; // mock functions to mock an environment, message info, dependencies
    use crate::contract::instantiate; // the contract instantiate function
    use crate::msg::InstantiateMsg; // our instantate method

    // Two fake addresses we will use to mock_info
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";
}
```

Just to touch on the topic of mocking, mocking is faking values for the sake of testing. For example `mock_dependencies` can fake the value needed for the `DepsMut` type for our `instantiate` call.

The two global address variables we will use when mocking info, for this case setting admin correctly.

Now I will show you how to write a test function:

```rust
#[cfg(test)]
mod tests {
    // Previous code omitted
    #[test]
    fn test_instantiate() {

    }
    // Following code omitted
}
```

There it is, there's our test! Well currently it does nothing, but if you run `cargo test` in your terminal you will now see that 1 test passed.

Well now let's get to our testing logic using those imports from earlier!

Let's start with some setup!

```rust
#[cfg(test)]
mod tests {
    // Previous code omitted
    #[test]
    fn test_instantiate() {
        // Mock the dependencies, must be mutable so we can pass it as a mutable, empty vector means our contract has no balance
        let mut deps = mock_dependencies();
        // Mock the contract environment, contains the block info, contract address, etc.
        let env = mock_env();
        // Mock the message info, ADDR1 will be the sender, the empty vec means we sent no funds.
        let info = mock_info(ADDR1, &vec![]);
    }
    // Following code omitted
}
```

Right this is all new so it's ok to be confused! Effectively these three new variables are mocking the `deps`, `env` and `info` parameters of our `instantiate` function.

Now let's have a look at some familiar stuff!

```rust
#[cfg(test)]
mod tests {
    // Previous code omitted
    #[test]
    fn test_instantiate() {
        // Mock the dependencies, must be mutable so we can pass it as a mutable, empty vector means our contract has no balance
        let mut deps = mock_dependencies(&vec![]);
        // Mock the contract environment, contains the block info, contract address, etc.
        let env = mock_env();
        // Mock the message info, ADDR1 will be the sender, the empty vec means we sent no funds.
        let info = mock_info(ADDR1, &vec![]);

        // Create a message where we (the sender) will be an admin
        let msg = InstantiateMsg { admin: None };
        // Call instantiate, unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    }
    // Following code omitted
}
```

Alright those next two parts should seem somewhat familiar! The first part is the `InstantiateMsg` we wrote. In this case we provide `None` as the admin meaning the sender (`ADDR1`) will be the admin.

The part after that is simply calling our `instantiate` function and capturing the response. We unwrap the `Result` wrapper to assert success as we expect it to succeed.

That leaves the `res` variable with type `Response` we created the response within the `Ok(...)` line of our `instantiate` function.

So we expect this `res` variable to have two attributes, one of which is simply `("action", "instantiate")` and the other we expect to be `("admin", "ADDR1")` as we provided `None` for the admin in our instantiate message.

If only there was a way we could `assert` that this was `equal` to what we expect.

Can you see where I am getting at?

If not that's fine, enter `assert_eq!`. `assert_eq!` is a macro used in testing to assert two values are the same. It will panic otherwise and fail the test.

Now let's show you how to use it. A `Response` object has a member variable which is a vector of attributes (`Vec<Attribute>`). In our case this can be accessed by using `res.attributes`. Now we want to assert that this is equal to another vector of attributes.

Alright enough text let's just show you how.

```rust
#[cfg(test)]
mod tests {
    // Previous code omitted
    #[test]
    fn test_instantiate() {
        // Mock the dependencies, must be mutable so we can pass it as a mutable, empty vector means our contract has no balance
        let mut deps = mock_dependencies(&vec![]);
        // Mock the contract environment, contains the block info, contract address, etc.
        let env = mock_env();
        // Mock the message info, ADDR1 will be the sender, the empty vec means we sent no funds.
        let info = mock_info(ADDR1, &vec![]);

        // Create a message where we (the sender) will be an admin
        let msg = InstantiateMsg { admin: None };
        // Call instantiate, unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }
    // Following code omitted
}
```

Alright so the first part makes sense right! So we want to assert two values are equal, one of these values is `res.attributes` (the first parameter). The other is a vector which we have hardcoded, as we expect the values to be equal. (Note: `vec![]` is a handy macro to make vectors inline). Also the `attr` function is simply a helper function that creates an `Attribute` struct for you. It's call signature is as follows `attr(key, value)`.

So now let's run `cargo test`, you should be met by the 1 test passing again. However we now know this test does something!

Alright some of you more eagle-eyed readers may have noticed our test does not cover the other scenario with our `InstantiateMsg`. This leads me on to our next section:

## Follow Up Exercises

Alright now we're getting into this, I'm going to set some follow-up exercises for you to undertake on your own! This is the perfect way to prove your understanding. I can't hold your hand all the time I'm afraid!

1. Write a new test called `test_instantiate_with_admin` that tests instantiating with the admin parameter set.
    - Hints
        - How can we give the `Option` type a `String` value? Maybe use the `Some` structure?
        - You should use `ADDR2` as your specified admin.
        - How is our `assert_eq!` call going to be different?

That's it for this part, as always thanks for reading all!
