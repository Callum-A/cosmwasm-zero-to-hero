#!/bin/bash

DEFAULT_DEV_ADDRESS="juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"

echo "Provisioning - juno address $DEFAULT_DEV_ADDRESS"

# pinched and adapted from whoami/DA0DA0
IMAGE_TAG=${2:-"v2.3.0-beta"}
CONTAINER_NAME="juno_cw_starter"
BINARY="docker exec -i $CONTAINER_NAME junod"
DENOM='ujunox'
CHAIN_ID='testing'
RPC='http://localhost:26657/'
TXFLAG="--gas-prices 0.1$DENOM --gas auto --gas-adjustment 1.3 -y -b block --chain-id $CHAIN_ID --node $RPC"
BLOCK_GAS_LIMIT=${GAS_LIMIT:-100000000} # should mirror mainnet

echo "Building $IMAGE_TAG"
echo "Configured Block Gas Limit: $BLOCK_GAS_LIMIT"

# kill any orphans
docker kill $CONTAINER_NAME
docker volume rm -f junod_data

# run junod setup script
docker run --rm -it \
    -e PASSWORD=xxxxxxxxx \
    -e STAKE_TOKEN=$DENOM \
    -e GAS_LIMIT="$GAS_LIMIT" \
    --mount type=volume,source=junod_data,target=/root \
    ghcr.io/cosmoscontracts/juno:$IMAGE_TAG /opt/setup_junod.sh $DEFAULT_DEV_ADDRESS

# we need app.toml and config.toml to enable CORS
# this means config wrangling required
docker run -v junod_data:/root --name helper busybox true
docker cp docker/app.toml helper:/root/.juno/config/app.toml
docker cp docker/config.toml helper:/root/.juno/config/config.toml
docker rm helper

docker run --rm -d --name $CONTAINER_NAME \
    -p 1317:1317 -p 26656:26656 -p 26657:26657 \
    --mount type=volume,source=junod_data,target=/root \
    ghcr.io/cosmoscontracts/juno:$IMAGE_TAG ./run_junod.sh

# compile
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.4

# copy wasm to docker container
docker cp artifacts/cw_starter.wasm $CONTAINER_NAME:/cw_starter.wasm

# validator addr
VALIDATOR_ADDR=$($BINARY keys show validator --address)
echo "Validator address:"
echo $VALIDATOR_ADDR

BALANCE_1=$($BINARY query bank balances $VALIDATOR_ADDR)
echo "Pre-store balance:"
echo $BALANCE_1

# you ideally want to run locally, get a user and then
# pass that addr in here
echo "Address to deploy contracts: $DEFAULT_DEV_ADDRESS"
echo "TX Flags: $TXFLAG"

# upload howl wasm
CONTRACT_CODE=$($BINARY tx wasm store "/cw_starter.wasm" --from validator $TXFLAG --output json | jq -r '.logs[0].events[-1].attributes[0].value')
echo "Stored: $CONTRACT_CODE"

BALANCE_2=$($BINARY query bank balances $VALIDATOR_ADDR)
echo "Post-store balance:"
echo $BALANCE_2

INIT='{"admin":null}'
echo "$INIT" | jq .
$BINARY tx wasm instantiate $CONTRACT_CODE "$INIT" --from "validator" --label "cw_starter" $TXFLAG --no-admin
RES=$?

# get contract addr
CONTRACT_ADDRESS=$($BINARY query wasm list-contract-by-code $CONTRACT_CODE --output json | jq -r '.contracts[-1]')

echo $CONTRACT_ADDRESS

printf "\n ------------------------ \n"
printf "Config Variables \n\n"

echo "NEXT_PUBLIC_CW_STARTER_CODE_ID=$CONTRACT_CODE"
echo "NEXT_PUBLIC_CW_STARTER_ADDRESS=$CONTRACT_ADDRESS"

echo $RES
exit $RES