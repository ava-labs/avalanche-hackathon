# Test gasless transaction

This step shows how to make gasless transactions in Avalanche EVM chains, using Ava Labs provided gas relayers. The gas relay servers are provisioned with gas paying keys, which then pays the gas fees on behalf of the users.

## Requirements

You will need the following in this tutorial:

- [foundry](https://github.com/foundry-rs/foundry) to deploy/test simple EIP-2771 compliant smart contracts
- Rust or Javascript -- we show examples in both languages, you can choose either one
- Avalanche EVM chain RPC URLs (to be provided below, by Ava Labs -- see [Glitch Hackathon DEVNET RPC URL](./2-connect-to-glitch-devnet-and-fund-the-wallet.eng.md#rpc-url))
- Avalanche gas relayer RPC URLs (to be provided below, by Ava Labs -- see [section below](#avalanche-gas-relayer-rpc-urls))
- Wallet (create your own wallet using [core.app](https://core.app), see [section below](#set-up-a-wallet))
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

## Set up a wallet

Make sure you have access to your wallet private key and the EVM chain RPC:

```bash
export MY_WALLET_EVM_ADDRESS="0xTODO"
```

```bash
curl ${EVM_CHAIN_RPC_URL} \
-X POST \
-H "Content-Type: application/json" \
-d "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBalance\",\"params\":[\"${MY_WALLET_EVM_ADDRESS}\", \"latest\"],\"id\":0}"
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

Ava Labs has already deployed the [`GaslessCounter` contract](../../../src/GaslessCounter.sol) that is compliant with [OpenGSN EIP-2771](https://github.com/opengsn/gsn/blob/master/packages/contracts/src/ERC2771Recipient.sol).

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

Now let's see how to call such contract without paying any gas using our gas relayer servers.

### Step 1. check the current contract status

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# 0x0000000000000000000000000000000000000000000000000000000000000001
```

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getLast()"
# 0x00000000000000000000000009cdb41fcec6410a00c7751257c33e9ea0d0c835
```

### Step 2. get the current forwarder contract "nonce" of your key

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${TRUSTED_FORWARDER_CONTRACT_ADDRESS} \
"getNonce(address)" \
${MY_WALLET_EVM_ADDRESS}
# TODO
```

In Rust:

```rust
// TODO: rust
```

In Javacript:

```javascript
import Web3 from 'web3'
import * as ethUtil from 'ethereumjs-util'

const FORWARDER_ABI = JSON.parse(
    await readFile(
      new URL('./Forwarder.json', import.meta.url)
    )
)

const web3 = new Web3(new Web3.providers.HttpProvider(EVM_CHAIN_RPC_URL))
const forwarderContract = new web3.eth.Contract(FORWARDER_ABI.abi, FORWARDER_CONTRACT_ADDRESS);

ethUtil.bnToHex(Number(await forwarderContract.methods.getNonce(MY_WALLET_EVM_ADDRESS).call()))
```

### Step 3. create "increment" calldata for counter contract

We need ABI-encode the target function calldata when calling the contract. We will use the `increment` function in the [`GaslessCounter` contract](../../../src/GaslessCounter.sol):

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
import Web3 from 'web3'
import * as ethUtil from 'ethereumjs-util'

const web3 = new Web3(new Web3.providers.HttpProvider(EVM_CHAIN_RPC_URL))

const domain = {
    name: DOMAIN_NAME,
    version: DOMAIN_VERSION,
    chainId: ethUtil.bnToHex(await web3.eth.getChainId()),
    verifyingContract: TRUSTED_FORWARDER_CONTRACT_ADDRESS,
    salt:null
};

const types = {
    EIP712Domain: [
        {name: 'name',type: 'string',},
        {name: 'version',type: 'string',},
        {name: 'chainId',type: 'uint256'},
        {name: 'verifyingContract',type: 'address',},
    ],
    Message: [
        { name: 'from', type: 'address' },
        { name: 'to', type: 'address' },
        { name: 'value', type: 'uint256' },
        { name: 'gas', type: 'uint256' },
        { name: 'nonce', type: 'uint256' },
        { name: 'data', type: 'bytes' },
        { name: 'validUntilTime', type: 'uint256' },
        { name: 'typeSuffixDatadatadatada', type: 'bytes32'},
    ]
};

const estimateGas = '4000000';
const primaryType = 'Message';

const message = {
    data: callData,
    from: MY_WALLET_EVM_ADDRESS,
    gas: ethUtil.bnToHex(Number(estimateGas)),
    nonce: ethUtil.bnToHex(Number(await forwarderContract.methods.getNonce(MY_WALLET_EVM_ADDRESS).call())),
    to: COUNTER_RECIPIENT_CONTRACT_ADDRESS,
    validUntilTime: String('0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'),
    value: String('0x0'),
};

const data =  {
    domain,
    types,
    primaryType,
    message,
};
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

Once we sign the EIP-712 message, we need to send the message to the gas relayer server. The example request format is as follows:

```json
{
    "forwardRequest": {
        "domain": {
            "name": "my name",
            "version": "1",
            "chainId": "0xa868",
            "verifyingContract": "0x52c84043cd9c865236f11d9fc9f56aa003c1f922"
        },
        "types": {
            "EIP712Domain": [
                {
                    "name": "name",
                    "type": "string"
                },
                {
                    "name": "version",
                    "type": "string"
                },
                {
                    "name": "chainId",
                    "type": "uint256"
                },
                {
                    "name": "verifyingContract",
                    "type": "address"
                }
            ],
            "Message": [
                {
                    "name": "from",
                    "type": "address"
                },
                {
                    "name": "to",
                    "type": "address"
                },
                {
                    "name": "value",
                    "type": "uint256"
                },
                {
                    "name": "gas",
                    "type": "uint256"
                },
                {
                    "name": "nonce",
                    "type": "uint256"
                },
                {
                    "name": "data",
                    "type": "bytes"
                },
                {
                    "name": "validUntilTime",
                    "type": "uint256"
                }
            ]
        },
        "primaryType": "Message",
        "message": {
            "data": "d09de08a",
            "from": "0xc886c5a4939c8835bf7bf643f3dbcadc6eb242d1",
            "gas": "0x1d0f6",
            "nonce": "0x0",
            "to": "0x5db9a7629912ebf95876228c24a848de0bfb43a9",
            "validUntilTime": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "value": "0x0"
        }
    },
    "metadata": {
        "signature": "914b460ab5dda9bbfd0675913b19c3c0e55a0886698dfc07f0f6dd4c28e449363826e26c4fa67bde3ee8a832e0a2a1f47a471ce2ac00b0856e81b2acc61af0dc1b"
    }
}
```

In Rust:

```rust
// TODO: rust
```

In Javacript:

```javascript
// TODO: javascript
```

### Step 7. confirm "increment" result from the counter contract

Once we send the message with signature, the counter should have incremented. To check, run the following:

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# ???
```

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getLast()"
# ???
```

## What's next?

Now that you tried gasless transaction with the Ava Labs gas relayer, you can try writing your own EIP-2771 compliant contract. Please check out the following links:

- [Writing GSN-capable contracts](https://docs.opengsn.org/contracts/#receiving-a-relayed-call)
