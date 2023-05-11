# Test gasless transaction

This step shows how to make gasless transactions in Avalanche EVM chains, using Ava Labs provided gas relayers.

## Requirements

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
export GAS_RELAYER_RPC_URL=TODO
```

## Trusted forwarder contract address

Ava Labs has already deployed a trusted forwarder contract for Glitch Hackathon.

Use the following:

```bash
# copy this for examples here
export TRUSTED_FORWARDER_CONTRACT_ADDRESS=0x52C84043CD9c865236f11d9Fc9F56aa003c1f922
```

## Registered domain name and verion

Ava Labs has already registered domain name and version for Glitch Hackathon.

Use the following:

```bash
# copy this for examples here
export DOMAIN_NAME="my domain name"
export DOMAIN_VERSION="my domain version"
```

## Registered type name and suffix data string

Ava Labs has already registered type name and suffix data for Glitch Hackathon.

Use the following:

```bash
# copy this for examples here
export TYPE_NAME="my type name"
export TYPE_SUFFIX_DATA="bytes8 typeSuffixDatadatadatada)"
```

## Simple counter contract address

Ava Labs has already deployed the [`GaslessCounter` contract](../../src/GaslessCounter.sol) that is compliant with [OpenGSN EIP-2771](https://github.com/opengsn/gsn/blob/master/packages/contracts/src/ERC2771Recipient.sol).

Use the following as the recipient contract address:

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

We need ABI-encode the target function calldata when calling the contract. We will use the `increment` function in the [`GaslessCounter` contract](../../src/GaslessCounter.sol):

```solidity
function increment() public {
    number++;

    last = _msgSender(); // not "msg.sender"
}
```

To ABI-encode the "increment" function call as calldata:

In Rust:

```rust
// TODO: rust
```

In Javacript:

```javascript
// TODO: javascript
```

### Step 4. create EIP-712 message

Now we need construct a structed message that is compliant with OpenGSN trusted forwarder contract and Ava Labs gas relayer server (see [EIP-712](https://eips.ethereum.org/EIPS/eip-712)).

In Rust:

```rust
// TODO: rust
```

In Javacript:

```javascript
// TODO: javascript
```

### Step 5. sign EIP-712 message

Once we create the EIP-712 message, we need to sign the message:

In Rust:

```rust
// TODO: rust
```

In Javacript:

```javascript
// TODO: javascript
```

### Step 6. send EIP-712 message and signature

Once we sign the EIP-712 message, we need to send the message to the gas relayer server:

In Rust:

```rust
// TODO: rust
```

In Javacript:

```javascript
// TODO: javascript
```

### Step 7. confirm "increment" result

Once we send the message with signature, the counter should have incremented. To check, run the following:

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# ???

cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getLast()"
# ???
```
