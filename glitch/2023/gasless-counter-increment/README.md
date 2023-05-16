
# Gasless counter increment

Rust command-line example for gasless transaction:

```bash
cd ./avalanche-hackathon/glitch/2023/gasless-counter-increment
```

```bash
EVM_CHAIN_RPC_URL=http://aops-custom-202305-2crvsg-nlb-1d600174371701f9.elb.ap-northeast-2.amazonaws.com:9650/ext/bc/XpX1yGquejU5cma1qERzkHKDh4fsPKs4NttnS1tErigPzugx5/rpc \
GAS_RELAYER_RPC_URL=http://gasrelay-202305-MvNkRv-nlb-71ce073c82f4c5a7.elb.ap-northeast-2.amazonaws.com:9876/rpc-sync \
TRUSTED_FORWARDER_CONTRACT_ADDRESS=0x52C84043CD9c865236f11d9Fc9F56aa003c1f922 \
DOMAIN_NAME="my domain name" \
DOMAIN_VERSION="my domain version" \
TYPE_NAME="my type name" \
TYPE_SUFFIX_DATA="bytes8 typeSuffixDatadatadatada)" \
GASLESS_COUNTER_RECIPIENT_CONTRACT_ADDRESS=0x5DB9A7629912EBF95876228C24A848de0bfB43A9 \
cargo run gasless-counter-increment
```
