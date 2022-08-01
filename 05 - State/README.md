# Part Five - State

We are finally getting to writing code! So let's get started, make sure you have your generated `cw-starter` project open in your text editor.

Let's start in the `src/state.rs` file. Upon opening it you should see code like the following:

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
```

Let's talk through this section by section.

The first 5 lines of code are importing other structs, interfaces and functions from other packages. To give a quick overview without getting bogged down too much:

-   JsonSchema allows structs to be serialized and deserialized to and from JSON
-   Deserialize and Serialize provide the serialization described above.
-   Addr is a Cosmos address, under the hood it is simply a string.
-   Item is a helper provided by storage plus. It effectively means we can store an item in storage. In this case, the `STATE` variable is an `Item` that stores a singular `State` struct.

We want to make some changes to this, let's rename it accordingly. I prefer my global state to be called `Config` as it is the configuration of my contract. Let's do this now and remove the `count` and `owner` variable from the struct and rename it to `Config`. Let's also rename `STATE` to `CONFIG`.

It should now look like this:

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {

}

pub const CONFIG: Item<Config> = Item::new("config");
```

Now let's think about what global configs we want, we probably want a poll admin of some sort. Let's store this as an address in config.

```rust
// Previous code omitted
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr
}
// Following code omitted
```

This admin will be able to delete any polls. Creators of a poll will also be able to delete their own polls.

So let's propose a question, how do we store these polls? What might we need to store?

Firstly let's look at what makes up a poll. For example:

```
What's your favourite Cosmos coin? <- Question
1. Juno <- Option - 2 votes <- Vote Count
2. Osmosis - 1 votes
3. Cosmos Hub - 3 votes
```

We can simplify a poll down to, a question with a range of answers with corresponding vote counts. Let's not forgot a poll also has a creator!

So in Smart Contract and Rust terms what does that look like?

1. Poll Creator
    - As we know users interact with smart contracts via transactions.
    - These transactions are signed for and are publically viewable, they also have the user's address attached.
    - We will use the user's address to store as the creator! (We can use that `Addr` type I described earlier)
2. Question
    - In simple terms, a question is a string of characters.
    - We will use the `String` type to store this.
    - An example of a string is `"The quick brown fox jumped over the lazy dog."`.
3. Options
    - This is a tricky one and not one obviously available.
    - Some users of other programming languages may suggest using a `Map` structure however these cannot be stored in cosmwasm storage plus.
    - What we will use instead is a Vector (list) of pairs (tuples of length 2).
        - For the poll example above the options vector will look like this:
        - [("Juno", 2), ("Osmosis", 1), ("Cosmos Hub", 3)]
        - The first element of each tuple is the option, and the second element is the count.
        - In Rust we will use a `Vec<(String, u64)>`

Let's now code this struct! Let's place it just below the `Config` struct we defined earlier.

```rust
// Previous code omitted
// Derive JSON serialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}
// Following code omitted
```

Some of you may notice a problem we have in our model. How do we keep track of who has already voted and what they voted for?

The answer is simple, we will use a secondary structure: `Ballot`. This ballot will simply store what option they voted for. (Don't worry if you're wondering where we store who cast this ballot, I'll get to that later).

Let's place this `Ballot` below the `Poll` struct.

```rust
// Previous code omitted
// Derive JSON serialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}
// Following code omitted
```

So we have defined our structs, but we still do not store them. As explained earlier we can store an `Item` but what if we want multiple?

We achieve this by using a `Map`, this interface is provided by `cw-storage-plus` and allows storage of values with keys. If you are unfamiliar with a `Map` data structure I recommend you do some reading on it.

So how do these maps work? Well firstly we need to define one, so let's talk through logic. Each poll is going to need a unique identifier to act as the key, for simplicities sake we are going to use a UUID that can be generated client side. This will be a `String` key.

So how do we define it?

Well, let's talk you through it.

Firstly at the top of the file, we need to modify one of the imports. Change:

```rust
use cw_storage_plus::Item;
```

To:

```rust
use cw_storage_plus::{Item, Map};
```

Now we import both `Item` and `Map` from `cw-storage-plus`.

So how do we define a `Map`` that we can use in our contract? Let's place it below our config `Item`.

```rust
// Previous code omitted
pub const CONFIG: Item<Config> = Item::new("config");

// A map with a String key and Poll value.
// The key will be a UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");
```

We also need to give it a string parameter for the storage key, let's simply give it the string `"polls"`.

Now, let's handle the `Ballot` storage. This is also going to be a `Map` but let's give our model thought for a second.

One user can vote on many polls.

So how can we store a user's vote across multiple polls?

We're going to use a composite key, (similar to the tuple options defined earlier). This composite key will be in the format of `(Addr, String)`. Where `Addr` is the address of the voter and `String` is the `Poll` UUID this vote is for.

So let's define this map below our `POLLS` map, remember to give it a storage key.

```rust
// Previous code omitted
pub const POLLS: Map<String, Poll> = Map::new("polls");
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");
```

Right we have finished coding for this one! Hopefully, you learned how to write custom structs and how to store them in storage.

Disclaimer! If you currently run any command such as `cargo test` or `cargo wasm` the build will break! Don't worry this is expected as we have modified our storage code without changing code on the contract side.

We'll fix this next time.
