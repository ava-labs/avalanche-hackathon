# Test simple smart contracts

We will use [Foundry](https://github.com/foundry-rs/foundry) to test simple [`Counter` contract](../../../src/Counter.sol) using the C-Chain API. And if you are testing against other networks than the local DEVNET (e.g., Glitch Hackathon DEVNET, Fuji public testnet), *please make sure to use the correct RPC URL (e.g., `--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc` only works for the local network)*.

You can get `EVM_CHAIN_RPC_URL` from [local DEVNET](./1-connect-to-local-devnet-and-fund-the-wallet.eng.md#rpc-url) or [Glitch Hackathon DEVNET](./2-connect-to-glitch-devnet-and-fund-the-wallet.eng.md#rpc-url).

```bash
# make sure you have access to the simple counter contract file
# https://github.com/ava-labs/avalanche-hackathon/blob/main/src/Counter.sol
#
# to make sure your local git repo has all the deps
# run the following two commands
cd ./avalanche-hackathon
git submodule update --init --recursive
forge update
```

```bash
# use "ewoq" key
export TEST_PRIVATE_KEY="56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027"
```

```bash
cd ./avalanche-hackathon
forge create \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=${TEST_PRIVATE_KEY} \
--rpc-url=${EVM_CHAIN_RPC_URL} \
./src/Counter.sol:Counter
```

```bash
# deployed address and transaction hash may differ
Deployer: 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
Deployed to: 0x17aB05351fC94a1a67Bf3f56DdbB941aE6c63E25
Transaction hash: ...
```

```bash
export COUNTER_CONTRACT_ADDRESS="0x17aB05351fC94a1a67Bf3f56DdbB941aE6c63E25"
```

**To increment the counter:**

```bash
# use the "ewoq" key
cast send \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=${TEST_PRIVATE_KEY} \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_CONTRACT_ADDRESS} \
"increment()"
```

```bash
# to see the debug events
cast receipt \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${TRANSACTION_HASH}
```

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
```

```bash
cast --to-dec 0x0000000000000000000000000000000000000000000000000000000000000001
# 1
```

```bash
# set to "ewoq" key address
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_CONTRACT_ADDRESS} \
"getLast()"
# 0x0000000000000000000000008db97c7cece249c2b98bdc0226cc4c2a57bf52fc
```
