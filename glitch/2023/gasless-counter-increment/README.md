
# Gasless counter increment

Rust command-line example for gasless transaction:

```bash
cd ./avalanche-hackathon/glitch/2023/gasless-counter-increment
```

```bash
EVM_CHAIN_RPC_URL=http://aops-custom-202305-2crvsg-nlb-1d600174371701f9.elb.ap-northeast-2.amazonaws.com:9650/ext/bc/XpX1yGquejU5cma1qERzkHKDh4fsPKs4NttnS1tErigPzugx5/rpc \
GAS_RELAYER_RPC_URL=http://gasrelay-202305-38hlg2-nlb-4deb54783e33031b.elb.ap-northeast-2.amazonaws.com:9876/rpc-sync \
TRUSTED_FORWARDER_CONTRACT_ADDRESS=0xC8d2c53fE31e4Ff2e3BDb33218E504836482D546 \
DOMAIN_NAME="AAAA Platform" \
DOMAIN_VERSION="1" \
TYPE_NAME="Message" \
TYPE_SUFFIX_DATA="bytes32 ABCDEFGHIJKLMNOPQRSTGSN)" \
GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS=0x175243D787c1555C84Fc4B9934d6f4E8662f7dE3 \
cargo run gasless-counter-increment
```
