# Test gasless transaction

This step shows how to make gasless transactions in Avalanche EVM chains, using Ava Labs provided gas relayers.

You will need the following in this tutorial:

- [foundry](https://github.com/foundry-rs/foundry) to deploy/test simple EIP-2771 compliant smart contracts
- Rust or Javascript -- we show examples in both languages, you can choose either one
- Avalanche EVM chain RPC URLs (to be provided below, by Ava Labs -- see [Glitch Hackathon DEVNET RPC URL](./2-connect-to-glitch-devnet-and-fund-the-wallet.eng.md#rpc-url))
- Avalanche gas relayer RPC URLs (to be provided below, by Ava Labs -- see [section below](#avalanche-gas-relayer-rpc-urls))
- Trusted forwarder contract address (to be provided below, by Ava Labs -- see [section below](#trusted-forwarder-contract-address))
- Registered domain name and verion (to be provided below, by Ava Labs -- see [section below](#registered-domain-name-and-verion))
- Registered type name and suffix data string (to be provided below, by Ava Labs -- see [section below](#registered-type-name-and-suffix-data-string))
- Simple counter contract address (e.g., recipient contract for testing gasless transaction -- see [section below](#simple-counter-contract-address))

## Avalanche gas relayer RPC URLs

Ava Labs has already set up dedicated gas relayer servers for Glitch Hackathon.

Use the following:

```bash
# copy this for examples here
export GAS_RELAYER_RPC_URLS=TODO
```

## Trusted forwarder contract address

Ava Labs has already deployed a trusted forwarder contract for Glitch Hackathon.

Use the following:

```bash
# copy this for examples here
export TRUSTED_FORWARDER_CONTRACT_ADDRESS=TODO
```

## Registered domain name and verion

Ava Labs has already registered domain name and version for Glitch Hackathon.

Use the following:

TODO

## Registered type name and suffix data string

Ava Labs has already registered type name and suffix data for Glitch Hackathon.

Use the following:

TODO

## Simple counter contract address

Ava Labs has already deployed the [simple counter contract](../../src/GaslessCounter.sol) that is compliant with [OpenGSN EIP-2771](https://github.com/opengsn/gsn/blob/master/packages/contracts/src/ERC2771Recipient.sol).

Use the following as the recipient contract address:

```yaml
TODO
```

```bash
export COUNTER_RECIPIENT_CONTRACT_ADDRESS=TODO
```

## Call the contract with zero balance

Try calling the contract with zero balance account, and expect failures like the following:

```bash
# THIS SHOULD FAIL
# account with no balance cannot send any transaction
# due to no gas
#
# private key "1af42b797a6bfbd3cf7554bed261e876db69190f5eb1b806acbd72046ee957c3"
# maps to "0xb513578fAb80487a7Af50e0b2feC381D0BD8fa9D"
cast send \
--private-key=1af42b797a6bfbd3cf7554bed261e876db69190f5eb1b806acbd72046ee957c3 \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"increment()"
# (code: -32000, message: gas required exceeds allowance (0), data: None)
```

## Call the contract without paying gas

Now let's see how to call such contract without paying any gas using our gas relayer.

### Step 1. check the current contract status

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# 0x0000000000000000000000000000000000000000000000000000000000000001

cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getLast()"
# 0x00000000000000000000000009cdb41fcec6410a00c7751257c33e9ea0d0c835
```

### Step 2. get the current forwarder contract nonce of your key

```bash
# use "0xb513578fAb80487a7Af50e0b2feC381D0BD8fa9D" as an example
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${TRUSTED_FORWARDER_CONTRACT_ADDRESS} \
"getNonce(address)" \
0xb513578fAb80487a7Af50e0b2feC381D0BD8fa9D
# TODO
```

### Step 3. create "increment" calldata

```bash
# TODO
```
