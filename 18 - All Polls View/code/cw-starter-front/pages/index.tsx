import type { NextPage } from 'next';
import Link from 'next/link';
import WalletLoader from 'components/WalletLoader';
import { useSigningClient } from 'contexts/cosmwasm';
import { useEffect, useState } from 'react';
import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';

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

export default Home;
