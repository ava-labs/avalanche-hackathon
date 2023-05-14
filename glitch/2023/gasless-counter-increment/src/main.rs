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

    let chain_id = json_client_evm::chain_id(&evm_chain_rpc_url).await.unwrap();
    log::info!("fetched chain_id {chain_id}");

    // TODO
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
