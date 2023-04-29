# Test simple smart contracts

We will use [foundry](https://github.com/foundry-rs/foundry) to test simple smart contracts using the C-Chain API.

And if you are testing against other networks than the local DEVNET (e.g., Glitch Hackathon DEVNET, Fuji public testnet), *please make sure to use the correct RPC URL (e.g., `--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc` only works for the local network)*


make sure to write this contract file locally
use this link `https://github.com/ava-labs/avalanche-hackathon/blob/main/src/Counter.sol.`


```sh
# make sure you have access to the simple counter contract file
# https://github.com/ava-labs/avalanche-hackathon/blob/main/src/Counter.sol

cd ./avalanche-hackathon
forge create \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
./src/Counter.sol:Counter
```
```sh
Deployer: 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
Deployed to: 0x17aB05351fC94a1a67Bf3f56DdbB941aE6c63E25
Transaction hash: 0x7ff975ee51ef2dcec54d4e5801377079579dc0b697654bd7897f05fab317326a
```

**To increment the counter:**

```sh
# use the "ewoq" key
cast send \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x17aB05351fC94a1a67Bf3f56DdbB941aE6c63E25 \
"increment()"

# to see the debug events
# use the transaction hash "0x8117b66ce18217f5b679596ebb2b01b395ae511917baa17a98dd597a2183a9a4"
cast receipt \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x8117b66ce18217f5b679596ebb2b01b395ae511917baa17a98dd597a2183a9a4

cast call \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x17aB05351fC94a1a67Bf3f56DdbB941aE6c63E25 \
"getNumber()" | sed -r '/^\s*$/d' | tail -1

cast --to-dec 0x0000000000000000000000000000000000000000000000000000000000000001
# 1

# set to "ewoq" key address
cast call \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
0x17aB05351fC94a1a67Bf3f56DdbB941aE6c63E25 \
"getLast()"
# 0x0000000000000000000000008db97c7cece249c2b98bdc0226cc4c2a57bf52fc
```


