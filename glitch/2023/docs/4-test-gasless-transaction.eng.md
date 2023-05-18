# Test gasless transaction

This step shows how to make gasless transactions in Avalanche EVM chains, using Ava Labs provided gas relayers. The gas relay servers are provisioned with gas paying keys, which then pays the gas fees on behalf of the users.

## Requirements

You will need the following in this tutorial:

- [Foundry](https://github.com/foundry-rs/foundry) to deploy/test simple EIP-2771 compliant smart contracts
- Pre-funded wallet and its hex-encoded private key -- see ["generate and import test private key"](./2-connect-to-glitch-devnet-and-fund-the-wallet.eng.md#step-2-generate-and-import-test-private-key)
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
export GAS_RELAYER_RPC_URL=http://gasrelay-202305-MvNkRv-nlb-71ce073c82f4c5a7.elb.ap-northeast-2.amazonaws.com:9876/rpc-sync
```

Check the following links for more info:

- http://gasrelay-202305-MvNkRv-nlb-71ce073c82f4c5a7.elb.ap-northeast-2.amazonaws.com:9876/info
- http://gasrelay-202305-MvNkRv-nlb-71ce073c82f4c5a7.elb.ap-northeast-2.amazonaws.com:9876/health

## Set up a wallet

Make sure you have access to your wallet private key and the EVM chain RPC:

```bash
export MY_WALLET_EVM_ADDRESS=0xTODO
```

```bash
curl ${EVM_CHAIN_RPC_URL} \
-X POST \
-H "Content-Type: application/json" \
-d "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBalance\",\"params\":[\"${MY_WALLET_EVM_ADDRESS}\", \"latest\"],\"id\":0}"
# {"jsonrpc":"2.0","id":0,"result":"0x0"}
```

It is fine to have zero balance in your wallet to interact with gasless transactions. But, if you need to deploy your own contract in DEVNET, you will need request fund from the faucet. Go [here](./2-connect-to-glitch-devnet-and-fund-the-wallet.eng.md#step-2-fund-your-wallet-using-glitch-hackathon-devnet-faucet).

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
export GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS=0x5DB9A7629912EBF95876228C24A848de0bfB43A9
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
${GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"increment()"
# (code: -32000, message: gas required exceeds allowance (0), data: None)
```

## Call the contract without paying gas

Now let's see how to call such contract without paying any gas using our gas relayer servers.

### Step 1. check the current contract status

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# 0x0000000000000000000000000000000000000000000000000000000000000000
```

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getLast()"
# 0x0000000000000000000000000000000000000000000000000000000000000000
```

### Step 2. get the current forwarder contract "nonce" of your key

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${TRUSTED_FORWARDER_CONTRACT_ADDRESS} \
"getNonce(address)" \
${MY_WALLET_EVM_ADDRESS}
# 0x0000000000000000000000000000000000000000000000000000000000000000
```

In Rust:

```rust
use std::{
    io::{self, stdout},
    sync::Arc,
};
use avalanche_types::{
    evm::{abi, eip712::gsn::Tx},
    key::secp256k1::private_key::Key,
    wallet::evm as wallet_evm,
};
use ethers::prelude::Eip1559TransactionRequest;
use ethers_core::{
    abi::{Function, Param, ParamType, StateMutability, Token},
    types::transaction::eip2718::TypedTransaction,
    types::{H160, U256},
};
use ethers_providers::Middleware;
use tokio::time::Duration;

fn get_nonce_calldata(addr: H160) -> Vec<u8> {
    // parsed function of "getNonce(address from)"
    let func = Function {
        name: "getNonce".to_string(),
        inputs: vec![Param {
            name: "from".to_string(),
            kind: ParamType::Address,
            internal_type: None,
        }],
        outputs: vec![Param {
            name: "nonce".to_string(),
            kind: ParamType::Uint(256),
            internal_type: None,
        }],
        constant: None,
        state_mutability: StateMutability::NonPayable,
    };
    let arg_tokens = vec![Token::Address(addr)];
    abi::encode_calldata(func, &arg_tokens).unwrap()
}

let chain_rpc_provider = wallet_evm::new_provider(
  env::var("EVM_CHAIN_RPC_URL").unwrap(),
  Duration::from_secs(15),
  Duration::from_secs(30),
  10,
  Duration::from_secs(3),
)
.unwrap();
log::info!("created chain rpc server provider for {EVM_CHAIN_RPC_URL}");

let tx = Eip1559TransactionRequest::new()
    .chain_id(chain_id.as_u64())
    .to(ethers::prelude::H160::from(
        env::var("TRUSTED_FORWARDER_CONTRACT_ADDRESS").unwrap()
    ))
    .data(get_nonce_calldata(no_gas_key.to_public_key().to_h160()));
let tx: TypedTransaction = tx.into();
let output = chain_rpc_provider.call(&tx, None).await.unwrap();
let forwarder_nonce_no_gas_key = U256::from_big_endian(&output);
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

const web3 = new Web3(new Web3.providers.HttpProvider(process.env.EVM_CHAIN_RPC_URL)
const forwarderContract = new web3.eth.Contract(FORWARDER_ABI.abi, process.env.TRUSTED_FORWARDER_CONTRACT_ADDRESS);

ethUtil.bnToHex(Number(await forwarderContract.methods.getNonce(process.env.MY_WALLET_EVM_ADDRESS).call()))
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
use ethers_core::abi::{Function, StateMutability};

let func = Function {
    name: "increment".to_string(),
    inputs: vec![],
    outputs: Vec::new(),
    constant: None,
    state_mutability: StateMutability::NonPayable,
};
let arg_tokens = vec![];
let no_gas_recipient_contract_calldata = abi::encode_calldata(func, &arg_tokens).unwrap();
log::info!(
    "no gas recipient contract calldata: 0x{}",
    hex::encode(no_gas_recipient_contract_calldata.clone())
);
```

In Javacript:

```javascript
const COUNTER_ABI=[
    "function increment() public"
]

const web3 = new Web3(new Web3.providers.HttpProvider(process.env.EVM_CHAIN_RPC_URL))
const counterContract = new web3.eth.Contract(COUNTER_ABI, process.env.GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS)
const callData = counterContract.methods.increment().encodeABI()
```

### Step 4. create and sign EIP-712 message

Now we need construct a structed message in [EIP-712](https://eips.ethereum.org/EIPS/eip-712) format that is compliant with OpenGSN trusted forwarder contract and Ava Labs gas relayer server. Once we create the EIP-712 message, we need to sign the message:

In Rust:

```rust
use std::sync::Arc;
use ethers_core::types::U256;
use tokio::time::Duration;

let mut relay_tx = Tx::new()
    //
    // make sure this matches with "registerDomainSeparator" call
    .domain_name(env::var("DOMAIN_NAME").unwrap())
    //
    .domain_version(env::var("DOMAIN_VERSION").unwrap())
    //
    .domain_chain_id(chain_id)
    //
    // trusted forwarder contract address
    .domain_verifying_contract(env::var("TRUSTED_FORWARDER_CONTRACT_ADDRESS").unwrap())
    //
    .from(no_gas_key.to_public_key().to_h160())
    //
    // contract address that this gasless transaction will interact with
    .to(env::var("GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS").unwrap())
    //
    // just some random value, otherwise, estimate gas fails
    .gas(U256::from(30000))
    //
    // contract call needs no value
    .value(U256::zero())
    //
    .nonce(forwarder_nonce_no_gas_key)
    //
    // calldata for contract calls
    .data(no_gas_recipient_contract_calldata)
    //
    .valid_until_time(U256::MAX)
    //
    .type_name(env::var("TYPE_NAME").unwrap())
    //
    .type_suffix_data(env::var("TYPE_SUFFIX_DATA").unwrap());

let chain_rpc_provider_arc = Arc::new(chain_rpc_provider);
let relay_tx_request = relay_tx
    .sign_to_request_with_estimated_gas_with_retries(
        no_gas_key_signer,
        Arc::clone(&chain_rpc_provider_arc),
        Duration::from_secs(30),
        Duration::from_millis(100),
        U256::from(10000),
    )
    .await
    .unwrap();
log::info!("relay_tx_request: {:?}", relay_tx_request);
```

In Javacript:

```javascript
import Web3 from 'web3'
import * as ethUtil from 'ethereumjs-util'

const web3 = new Web3(new Web3.providers.HttpProvider(process.env.EVM_CHAIN_RPC_URL))

const domain = {
    name: DOMAIN_NAME,
    version: DOMAIN_VERSION,
    chainId: ethUtil.bnToHex(await web3.eth.getChainId()),
    verifyingContract: process.env.TRUSTED_FORWARDER_CONTRACT_ADDRESS,
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
    from: process.env.MY_WALLET_EVM_ADDRESS,
    gas: ethUtil.bnToHex(Number(estimateGas)),
    nonce: ethUtil.bnToHex(Number(await forwarderContract.methods.getNonce(process.env.MY_WALLET_EVM_ADDRESS).call())),
    to: process.env.GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS,
    validUntilTime: String('0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff'),
    value: String('0x0'),
};

const dataToSign =  {
    domain,
    types,
    primaryType,
    message: {
        ...message,
        typeSuffixDatadatadatada: Buffer.from(process.env.TYPE_SUFFIX_DATA, 'utf8'),
    },
};

const sig = ethSigUtil.signTypedData(
    {
        privateKey: Buffer.from(FROM_ADDRESS_PK, 'hex'),
        data: dataToSign,
        version: ethSigUtil.SignTypedDataVersion.V4,
    }
);

// optional, to double check the signature
const ecRecover = ethSigUtil.recoverTypedSignature(
    {
        data: dataToSign,
        signature: sig,
        version: ethSigUtil.SignTypedDataVersion.V4,
    }
);
```

### Step 5. send EIP-712 message and signature

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
let signed_bytes: ethers_core::types::Bytes =
    serde_json::to_vec(&relay_tx_request).unwrap().into();

let pending = relay_server_provider
    .send_raw_transaction(signed_bytes)
    .await
    .unwrap();
log::info!(
    "pending tx hash {} from 0x{:x}",
    pending.tx_hash(),
    no_gas_key.to_public_key().to_h160()
);
```

In Javacript:

```javascript
const forwardRequest =  {
    domain,
    types,
    primaryType,
    message,
};

const relayTx = {
    forwardRequest: forwardRequest,
    metadata: {
        signature : sig.substring(2)
    }
};

const hexRawTx = '0x' + Buffer.from(JSON.stringify(relayTx)).toString('hex');
console.log(hexRawTx)

var xmlhttp = new XMLHttpRequest();
xmlhttp.open("POST", process.env.GAS_RELAYER_RPC_URL);
xmlhttp.send(JSON.stringify({"jsonrpc": "2.0", "method": "eth_sendRawTransaction", "params": [hexRawTx], "id": 1}));
```

### Step 6. try again with Rust command-line example

Please check the [`gasless-counter-increment`](../gasless-counter-increment) command to automate all above and see how to call `increment` using Rust.

```bash
# make sure you have all the variables set via "export" command
# see above to get all the env vars for this DEVNET
export EVM_CHAIN_RPC_URL=...
export GAS_RELAYER_RPC_URL=...
export TRUSTED_FORWARDER_CONTRACT_ADDRESS=...
export DOMAIN_NAME=...
export DOMAIN_VERSION=...
export TYPE_NAME=...
export TYPE_SUFFIX_DATA=...
export GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS=...

# or pass env vars directly to the "cargo" command
cd ./avalanche-hackathon/glitch/2023/gasless-counter-increment
cargo run gasless-counter-increment
```

### Step 7. confirm "increment" result from the counter contract

Once we send the message with signature, the counter should have incremented. To check, run the following:

```bash
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# 0x0000000000000000000000000000000000000000000000000000000000000001

# after increment, the counter should go up by 1

cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getNumber()" | sed -r '/^\s*$/d' | tail -1
# 0x0000000000000000000000000000000000000000000000000000000000000002
```

```bash
# address of the message signer, which had no gas but still able to increment the counter
cast call \
--rpc-url=${EVM_CHAIN_RPC_URL} \
${GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS} \
"getLast()"
# 0x000000000000000000000000a5fc4b134c572e9aaf03cb1f56099ff717f3964a
```

## What's next?

Now that you tried gasless transaction with the Ava Labs gas relayer, you can try writing your own EIP-2771 compliant contract. Please check out the following links:

- [Writing GSN-capable contracts](https://docs.opengsn.org/contracts/#receiving-a-relayed-call)
