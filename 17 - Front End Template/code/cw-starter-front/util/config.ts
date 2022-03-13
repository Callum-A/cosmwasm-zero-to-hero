export const configMap = {
  chainId: process.env.NEXT_PUBLIC_CHAIN_ID,
  chainName: process.env.NEXT_PUBLIC_CHAIN_NAME,
  rpcEndpoint: process.env.NEXT_PUBLIC_CHAIN_RPC_ENDPOINT,
  restEndpoint: process.env.NEXT_PUBLIC_CHAIN_REST_ENDPOINT,
  coinDenom: process.env.NEXT_PUBLIC_STAKING_DENOM,
  bech32Prefix: process.env.NEXT_PUBLIC_CHAIN_BECH32_PREFIX,
  coinDecimals: process.env.NEXT_PUBLIC_COIN_DECIMALS,
  gasPrice: process.env.NEXT_PUBLIC_GAS_PRICE,
};

export type ConfigKey = keyof typeof configMap;

export const config = (key: ConfigKey): string => {
  const value = configMap[key];
  if (!value) throw new Error(`Missing config for ${key}`);
  return value;
};

export const configObject = (
  ...keys: ConfigKey[]
): { [key in ConfigKey]: string } => {
  return keys.reduce((obj, key) => {
    obj[key] = config(key);
    return obj;
  }, {} as { [key in ConfigKey]: string });
};
