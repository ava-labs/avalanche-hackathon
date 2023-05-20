#![allow(deprecated)]

use std::{env, str::FromStr, sync::Arc};

use avalanche_types::{
    evm::{abi, eip712::gsn::Tx},
    jsonrpc::client::evm as json_client_evm,
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

/// cargo run gasless-counter-increment
#[tokio::main]
async fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let evm_chain_rpc_url = env::var("EVM_CHAIN_RPC_URL").unwrap();
    let gas_relayer_rpc_url = env::var("GAS_RELAYER_RPC_URL").unwrap();

    let s = env::var("TRUSTED_FORWARDER_CONTRACT_ADDRESS").unwrap();
    let trusted_forwarder_contract_address = H160::from_str(&s).unwrap();

    let domain_name = env::var("DOMAIN_NAME").unwrap();
    let domain_version = env::var("DOMAIN_VERSION").unwrap();

    let type_name = env::var("TYPE_NAME").unwrap();
    let type_suffix_data = env::var("TYPE_SUFFIX_DATA").unwrap();

    let s = env::var("GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS").unwrap();
    let gasless_counter_recipient_contract_address = H160::from_str(&s).unwrap();

    log::info!("evm_chain_rpc_url: {evm_chain_rpc_url}");
    log::info!("gas_relayer_rpc_url: {gas_relayer_rpc_url}");
    log::info!(
        "trusted_forwarder_contract_address: 0x{:x}",
        trusted_forwarder_contract_address
    );

    log::info!("domain_name: {domain_name}");
    log::info!("domain_version: {domain_version}");

    log::info!("type_name: {type_name}");
    log::info!("type_suffix_data: {type_suffix_data}");

    log::info!(
        "gasless_counter_recipient_contract_address: 0x{:x}",
        gasless_counter_recipient_contract_address
    );

    let no_gas_key = Key::generate().unwrap();
    log::info!(
        "generated key {}",
        no_gas_key.to_public_key().to_eth_address()
    );
    let no_gas_key_signer: ethers_signers::LocalWallet =
        no_gas_key.to_ethers_core_signing_key().into();

    let chain_id = json_client_evm::chain_id(&evm_chain_rpc_url).await.unwrap();
    log::info!("fetched chain_id {chain_id}");

    let relay_server_provider = wallet_evm::new_provider(
        &gas_relayer_rpc_url,
        Duration::from_secs(15),
        Duration::from_secs(30),
        10,
        Duration::from_secs(3),
    )
    .unwrap();
    log::info!("created gas relay server provider for {gas_relayer_rpc_url}");

    let chain_rpc_provider = wallet_evm::new_provider(
        &evm_chain_rpc_url,
        Duration::from_secs(15),
        Duration::from_secs(30),
        10,
        Duration::from_secs(3),
    )
    .unwrap();
    log::info!("created chain rpc server provider for {evm_chain_rpc_url}");

    let tx = Eip1559TransactionRequest::new()
        .chain_id(chain_id.as_u64())
        .to(ethers::prelude::H160::from(
            trusted_forwarder_contract_address.as_fixed_bytes(),
        ))
        .data(get_nonce_calldata(no_gas_key.to_public_key().to_h160()));
    let tx: TypedTransaction = tx.into();
    let output = chain_rpc_provider.call(&tx, None).await.unwrap();
    let forwarder_nonce_no_gas_key = U256::from_big_endian(&output);
    log::info!(
        "forwarder_nonce_no_gas_key: {} {}",
        no_gas_key.to_public_key().to_h160(),
        forwarder_nonce_no_gas_key
    );

    let no_gas_recipient_contract_calldata = increment_calldata();
    log::info!(
        "no gas recipient contract calldata: 0x{}",
        hex::encode(no_gas_recipient_contract_calldata.clone())
    );

    let mut relay_tx = Tx::new()
        //
        // make sure this matches with "registerDomainSeparator" call
        .domain_name(domain_name)
        //
        .domain_version(domain_version)
        //
        // local network
        .domain_chain_id(chain_id)
        //
        // trusted forwarder contract address
        .domain_verifying_contract(trusted_forwarder_contract_address)
        .from(no_gas_key.to_public_key().to_h160())
        //
        // contract address that this gasless transaction will interact with
        .to(gasless_counter_recipient_contract_address)
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
        .type_name(type_name)
        //
        .type_suffix_data(type_suffix_data);

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

    let req_for_debug = serde_json::to_string_pretty(&relay_tx_request).unwrap();
    log::info!("sending this EIP-712 signed message:\n{req_for_debug}\n");

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
}

fn increment_calldata() -> Vec<u8> {
    // parsed function of "increment()"
    let func = Function {
        name: "increment".to_string(),
        inputs: vec![],
        outputs: Vec::new(),
        constant: None,
        state_mutability: StateMutability::NonPayable,
    };
    let arg_tokens = vec![];
    abi::encode_calldata(func, &arg_tokens).unwrap()
}

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
