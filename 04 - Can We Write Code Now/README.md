# Part Four - Can We Write Code Now

No we're not going to write code this chapter. Instead I am going to give an overview of what we will be building.

## On Chain Polls

As our first project we're going to build and store polls on chain. Its functionality is as follows:

-   Any user can create a poll.
-   Any user can vote on a poll.
-   Polls can have different options.

For a textual example I will talk through a scenario with some users:

1. User A can create a poll for example:
    - What Cosmos coin is your favourite?
        1. Juno
        1. Osmosis
        1. Cosmos Hub
2. User A decides to vote on their own poll, they vote `Juno`
3. User B can also vote on the poll, they vote `Cosmos Hub`
4. After a certain amount of time User A (as the owner of the poll) can close the poll. User A closes the poll
5. User C attempts to vote on a closed poll. They cannot
6. The poll results are now visible for everyone to see on chain

In the next chapter I promise we will write some code!
