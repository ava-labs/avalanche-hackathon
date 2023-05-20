
# Gasless counter increment

Rust command-line example for gasless transaction:

```bash
cd ./avalanche-hackathon/glitch/2023/gasless-counter-increment
```

```bash
EVM_CHAIN_RPC_URL=http://aops-custom-202305-2crvsg-nlb-1d600174371701f9.elb.ap-northeast-2.amazonaws.com:9650/ext/bc/XpX1yGquejU5cma1qERzkHKDh4fsPKs4NttnS1tErigPzugx5/rpc \
GAS_RELAYER_RPC_URL=http://gasrelay-202305-38hlg2-nlb-4deb54783e33031b.elb.ap-northeast-2.amazonaws.com:9876/rpc-sync \
TRUSTED_FORWARDER_CONTRACT_ADDRESS=0xFA2562F4A6581fECa7b19379894B50FbF95ba350 \
DOMAIN_NAME="my domain name" \
DOMAIN_VERSION="my domain version" \
TYPE_NAME="my type name" \
TYPE_SUFFIX_DATA="bytes32 ABCDEFGHIJKLMNOPQRSTGSN)" \
GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS=0xB79AeEfB71931397c7c1c785127Ba4e76D44eB8B \
cargo run gasless-counter-increment
```
