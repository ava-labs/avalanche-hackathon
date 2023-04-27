TODO# Test local Avalanche network

### Install Go 

AvalancheGo compilation requires Go. So, make sure your environment has Go installed.
```sh
go version
```

### Install Foundry 
We will use [Foundry](https://github.com/foundry-rs/foundry) to deploy and interact with smart contracts. Install [Foundry](https://github.com/foundry-rs/foundry)
Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.
```sh
forge --version
cast --version
```

### Download AvalancheGo code base

First, Make directory to git clone 
```sh
mkdir -p ${HOME}/go/src/github.com/ava-labs
cd ${HOME}/go/src/github.com/ava-labs
```
Second, Git clone the repository from [ava-labs/avalanchego](https://github.com/ava-labs/avalanchego)
```sh
git clone git@github.com:ava-labs/avalanchego.git
cd ./avalanchego
```

### Compile AvalancheGo code base
```sh
cd ${HOME}/go/src/github.com/ava-labs/avalanchego
./scripts/build.sh
find ./build
./build/avalanchego --version
```

### Start the local AvalancheGo network
```sh
cd ${HOME}/go/src/github.com/ava-labs/avalanchego
./build/avalanchego \
--network-id=local \
--staking-enabled=false \
--db-type=memdb \
--log-level=info
```
or to expose the 9650 port to all traffic
```sh
cd ${HOME}/go/src/github.com/ava-labs/avalanchego
./build/avalanchego \
--network-id=local \
--http-host=0.0.0.0 \
--staking-enabled=false \
--db-type=memdb \
--log-level=info
```

### Test the local AvalancheGo network endpoints
Now the local Avalanche network has the following endpoints
```sh
http_rpc: http://localhost:9650
http_rpc_x: http://localhost:9650/ext/bc/X
http_rpc_p: http://localhost:9650/ext/bc/P
http_rpc_c: http://localhost:9650/ext/bc/C/rpc
metrics: http://localhost:9650/ext/metrics
health: http://localhost:9650/ext/health
liveness: http://localhost:9650/ext/health/liveness
metamask_rpc_c: http://localhost:9650/ext/bc/C/rpc
websocket_rpc_c: ws://localhost:9650/ext/bc/C/ws
```

Next Codes are some examples about local Avalanche network

**To get the currenet node ID:**
```sh
curl -X POST --data '{
    "jsonrpc":"2.0",
    "id"     :1,
    "method" :"info.getNodeID"
}' -H 'content-type:application/json;' http://localhost:9650/ext/info
```

Then, you can get the current node ID as follows

`{"jsonrpc":"2.0","result":{"nodeID":"NodeID-6NeWXVweQgfHgxY4WLn1XKg5niPfUGsZn"...`

**To get the network ID:**
```sh
curl -X POST --data '{
    "jsonrpc":"2.0",
    "id"     :1,
    "method" :"info.getNetworkID"
}' -H 'content-type:application/json;' http://localhost:9650/ext/info
```

Then, you can get the network ID as follows

`{"jsonrpc":"2.0","result":{"networkID":"12345"},"id":1}`

See [avalanchego/apis](https://docs.avax.network/apis/avalanchego/apis) for more


# Test the local AvalancheGo network with web wallets

We will use the following test keys to integrate the Avalanche local network with Metamask. Note that the key that starts with ewoq is the canonical test key used in our local network, pre-funded through the genesis block:

```sh
[
    {
        "key_type": "hot",
        "private_key_cb58": "PrivateKey-ewoqjP7PxY4yr3iLTpLisriqt94hdyDFNgchSxGGztUrTXtNN",
        "private_key_hex": "0x56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027",
        "addresses": {
            "12345": {
                "x_address": "X-local18jma8ppw3nhx5r4ap8clazz0dps7rv5u00z96u",
                "p_address": "P-local18jma8ppw3nhx5r4ap8clazz0dps7rv5u00z96u"
            }
        },
        "short_address": "6Y3kysjF9jnHnYkdS9yGAuoHyae2eNmeV",
        "eth_address": "0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC",
        "h160_address": "0x8db97c7cece249c2b98bdc0226cc4c2a57bf52fc"
    },
    {
        "key_type": "hot",
        "private_key_cb58": "PrivateKey-if94hLX5xK14jChGZUsh7PSBsBT8KkRNAwS88NoHeTxZNbJh2",
        "private_key_hex": "0x5e96aefcb014670192ad0a5a95bf5dfe8f62537b99327a13a942c127913c0281",
        "addresses": {
            "12345": {
                "x_address": "X-local1myazn9837mrey0zxmqrzjfgt605e72fey4n9mp",
                "p_address": "P-local1myazn9837mrey0zxmqrzjfgt605e72fey4n9mp"
            }
        },
        "short_address": "LobJpmdncFrFRYtUj4oiUKKWzg9eR4KSX",
        "eth_address": "0x53C62F5d19f94556c4e9E9Ee97CeE274AB053399",
        "h160_address": "0x53c62f5d19f94556c4e9e9ee97cee274ab053399"
    }
]
```

### Core wallet

Download the Core wallet extension [here](https://core.app)

**Step 1. Add the Avalanche C-chain local network to the Core using**

`http://localhost:9650/ext/bc/C/rpc:`

**Step 2. Import the test `ewoq` key using its hex-encoded private key**
`56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 :`

**Step 3. Make sure Core shows the same balance as the following commands:**

```sh
# ewoq key address is "0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC"
curl http://localhost:9650/ext/bc/C/rpc \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC", "latest"],"id":0}'

# check P-chain balance (this is not shown in Core yet)
curl --location --request POST 'http://localhost:9650/ext/bc/P' \
--header 'Content-Type: application/json' \
--data-raw '{
    "jsonrpc":"2.0",
    "id"     :1,
    "method" :"platform.getBalance",
    "params" :{
      "addresses":["P-local18jma8ppw3nhx5r4ap8clazz0dps7rv5u00z96u"]
    }
}'
# {"jsonrpc":"2.0","result":{"balance":"30000000000000000",...
```

**Step 4. Transfer some tokens to another test key of zero balance. We will use the second key in the above JSON:**

```sh
# another key address is "0x53C62F5d19f94556c4e9E9Ee97CeE274AB053399"
curl http://localhost:9650/ext/bc/C/rpc \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0x53C62F5d19f94556c4e9E9Ee97CeE274AB053399", "latest"],"id":0}'
```

**Step 5. Make sure the tokens got transfered by checking the balance of two accounts:**

```sh
# ewoq key address is "0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC"
curl http://localhost:9650/ext/bc/C/rpc \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC", "latest"],"id":0}'
# {"jsonrpc":"2.0","id":0,"result":"0x2946bc6b88d726f8f3a400"}

cast --to-dec 0x2946bc6b88d726f8f3a400
# 49899999994566250000000000
```

```sh
# another key address is "0x53C62F5d19f94556c4e9E9Ee97CeE274AB053399"
curl http://localhost:9650/ext/bc/C/rpc \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0x53C62F5d19f94556c4e9E9Ee97CeE274AB053399", "latest"],"id":0}'
# {"jsonrpc":"2.0","id":0,"result":"0x152d02c7e14af6800000"}

cast --to-dec 0x152d02c7e14af6800000
# 100000000000000000000000
```