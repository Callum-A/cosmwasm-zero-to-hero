import { useState } from "react";
import { getKeplr, suggestChain } from "util/keplr";
import {
  SigningCosmWasmClient,
  CosmWasmClient,
} from "@cosmjs/cosmwasm-stargate";
import { config } from "util/config";
import { GasPrice } from "@cosmjs/stargate";

export interface ISigningCosmWasmClientContext {
  walletAddress: string;
  signingClient: SigningCosmWasmClient | null;
  loading: boolean;
  error: any;
  connectWallet: any;
  disconnect: Function;
}

export const useSigningCosmWasmClient = (): ISigningCosmWasmClientContext => {
  const [walletAddress, setWalletAddress] = useState("");
  const [signingClient, setSigningClient] =
    useState<SigningCosmWasmClient | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const connectWallet = async () => {
    setLoading(true);

    try {
      const chainId = config("chainId");
      const keplr = await getKeplr();
      suggestChain();

      // enable website to access kepler
      await keplr.enable(config("chainId"));

      // get offline signer for signing txs
      const offlineSigner = await keplr.getOfflineSignerAuto(chainId);

      // make client
      const client = await SigningCosmWasmClient.connectWithSigner(
        config("rpcEndpoint"),
        offlineSigner,
        {
          gasPrice: GasPrice.fromString(
            `${config("gasPrice")}${config("coinDenom")}`
          ),
        }
      );
      setSigningClient(client);

      // get user address
      const [{ address }] = await offlineSigner.getAccounts();
      setWalletAddress(address);

      setLoading(false);
    } catch (error) {
      setError(error);
    }
  };

  const disconnect = () => {
    if (signingClient) {
      signingClient.disconnect();
    }
    setWalletAddress("");
    setSigningClient(null);
    setLoading(false);
  };

  return {
    walletAddress,
    signingClient,
    loading,
    error,
    connectWallet,
    disconnect,
  };
};

export const getNonSigningClient = async () => {
  const client = await CosmWasmClient.connect(config("rpcEndpoint"));
  return client;
};
