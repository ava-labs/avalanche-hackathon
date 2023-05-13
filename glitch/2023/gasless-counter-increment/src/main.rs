use std::env;

fn main() {
    println!(
        "EVM_CHAIN_RPC_URL: {}",
        env::var("EVM_CHAIN_RPC_URL").unwrap()
    );
}
