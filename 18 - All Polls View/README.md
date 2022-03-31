# Part Eighteen - All Polls View

So firstly we need to be able to view polls right?
I'm going to assume you're familiar with Next.js already so lets head straight to the `pages/index.ts`.

You should see a component that looks like:

```typescript
const Home: NextPage = () => {
    const { walletAddress } = useSigningClient();

    return (
        <WalletLoader>
            <h1 className="text-6xl font-bold">
                Welcome to {process.env.NEXT_PUBLIC_CHAIN_NAME} !
            </h1>

            <div className="mt-3 text-2xl">
                Your wallet address is:{' '}
                <pre className="font-mono break-all whitespace-pre-wrap">
                    {walletAddress}
                </pre>
            </div>

            <div className="flex flex-wrap items-center justify-around max-w-4xl mt-6 max-w-full sm:w-full">
                <Link href="/send" passHref>
                    <a className="p-6 mt-6 text-left border border-secondary hover:border-primary w-96 rounded-xl hover:text-primary focus:text-primary-focus">
                        <h3 className="text-2xl font-bold">
                            Send to wallet &rarr;
                        </h3>
                        <p className="mt-4 text-xl">
                            Execute a trasaction to send funds to a wallet
                            address.
                        </p>
                    </a>
                </Link>
            </div>
        </WalletLoader>
    );
};
```

Let's tear out the stuff we won't need. It should now look something like:

```tsx
const Home: NextPage = () => {
    return <div>Home</div>;
};
```

The WalletLoader is a helper component defined in `components` which ensures a wallet is loaded for its children. Basically any wallet specific actions need to be contained within this. As we're allowing anyone to view polls, we do not need this.

Now we also will need our contract address, we're going to need it set in our `.env` lets take a look in `.env.local` it should currently look similar to:

```.env
NEXT_PUBLIC_CHAIN_ID=testing
NEXT_PUBLIC_CHAIN_NAME="Juno Local Test"
NEXT_PUBLIC_CHAIN_BECH32_PREFIX=juno
NEXT_PUBLIC_CHAIN_RPC_ENDPOINT=http://localhost:26657
NEXT_PUBLIC_CHAIN_REST_ENDPOINT=http://localhost:1317
NEXT_PUBLIC_STAKING_DENOM="ujunox"
NEXT_PUBLIC_COIN_DECIMALS=6
NEXT_PUBLIC_GAS_PRICE=0.025
NEXT_PUBLIC_SITE_TITLE="Starter Kit"
NEXT_PUBLIC_SITE_ICON_URL="/juno.svg"
```

In part 16 of a tutorial we ran a script that outputs a contract address and the next variables we need. Simply paste them into the `.env.local`. NOTE: these variables will need to be updated everytime you run the `deploy_local` script.

Your `.env` should now look like:

```.env
NEXT_PUBLIC_CHAIN_ID=testing
NEXT_PUBLIC_CHAIN_NAME="Juno Local Test"
NEXT_PUBLIC_CHAIN_BECH32_PREFIX=juno
NEXT_PUBLIC_CHAIN_RPC_ENDPOINT=http://localhost:26657
NEXT_PUBLIC_CHAIN_REST_ENDPOINT=http://localhost:1317
NEXT_PUBLIC_STAKING_DENOM="ujunox"
NEXT_PUBLIC_COIN_DECIMALS=6
NEXT_PUBLIC_GAS_PRICE=0.025
NEXT_PUBLIC_SITE_TITLE="Starter Kit"
NEXT_PUBLIC_SITE_ICON_URL="/juno.svg"
NEXT_PUBLIC_CW_STARTER_CODE_ID=1
NEXT_PUBLIC_CW_STARTER_ADDRESS=juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8
```

Now we can add the following code to our home page to get our address:

```tsx
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CW_STARTER_ADDRESS || '';

const Home: NextPage = () => {
    return <div>Home</div>;
};
```

Now we need to use a `useEffect` hook to load and store the polls in state, firstly lets setup the state:

```tsx
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CW_STARTER_ADDRESS || '';

const Home: NextPage = () => {
    const [polls, setPolls] = useState<any[]>([]);

    return <div>Home</div>;
};
```

I am choosing to not define types for ease of development, but I recommend you define interfaces to use.

Now we need to load polls on page load using `useEffect`:

```tsx
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CW_STARTER_ADDRESS || '';

const Home: NextPage = () => {
    const [polls, setPolls] = useState<any[]>([]);

    useEffect(() => {}, [signingClient]);

    return <div>Home</div>;
};
```

We now need to call our query, we achieve this by connecting to our RPC, and calling the query:

```tsx
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CW_STARTER_ADDRESS || '';

const Home: NextPage = () => {
    const [polls, setPolls] = useState<any[]>([]);

    useEffect(() => {
        const main = async () => {
            const client = await CosmWasmClient.connect(
                process.env.NEXT_PUBLIC_CHAIN_RPC_ENDPOINT || ''
            );
            const pollsResponse = await client.queryContractSmart(
                CONTRACT_ADDRESS,
                { all_polls: {} }
            );
            console.log(pollsResponse);
        };
        main();
    }, []);
    return <div>Home</div>;
};
```

As it is async we use a little hack closure function. Now the `pollsResponse` variable will contain our `PollResponse` we defined as JSON. Note the query message is snakecase as we set Serde to rename everything snakecase when creating our rust structs.

This means we can use `pollResponse.polls` to set the state:

```tsx
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CW_STARTER_ADDRESS || '';

const Home: NextPage = () => {
    const [polls, setPolls] = useState<any[]>([]);

    useEffect(() => {
        const main = async () => {
            const client = await CosmWasmClient.connect(
                process.env.NEXT_PUBLIC_CHAIN_RPC_ENDPOINT || ''
            );
            const pollsResponse = await client.queryContractSmart(
                CONTRACT_ADDRESS,
                { all_polls: {} }
            );
            console.log(pollsResponse);
            setPolls(pollsResponse.polls);
        };
        main();
    }, []);
    return <div>Home</div>;
};
```

Now we have set the variables we can now iterate over our polls in the HTML section:

```tsx
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CW_STARTER_ADDRESS || '';

const Home: NextPage = () => {
    const [polls, setPolls] = useState<any[]>([]);

    useEffect(() => {
        const main = async () => {
            const client = await CosmWasmClient.connect(
                process.env.NEXT_PUBLIC_CHAIN_RPC_ENDPOINT || ''
            );
            const pollsResponse = await client.queryContractSmart(
                CONTRACT_ADDRESS,
                { all_polls: {} }
            );
            console.log(pollsResponse);
            setPolls(pollsResponse.polls);
        };
        main();
    }, []);
    return (
        <div>
            {polls.map((poll) => {
                return <div key={poll.id}>{poll.id}</div>;
            })}
        </div>
    );
};
```

And what do we now see! Absolutely nothing!

This is because we have yet to create a poll in the next section we will add the form which allows users to create a poll!
