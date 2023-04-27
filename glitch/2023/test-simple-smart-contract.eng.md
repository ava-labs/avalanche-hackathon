# Test simple smart contracts

We will use [foundry](https://github.com/foundry-rs/foundry) to test simple smart contracts against C-chain

Create a simple counter contract 

```sh
cd /tmp

mkdir -p /tmp/contracts/counter
cat << EOF > /tmp/contracts/counter/Counter.sol
// SPDX-License-Identifier: MIT

pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;
    address public last;

    event DebugAddress(address indexed _addr);

    function setNumber(uint256 newNumber) public {
        number = newNumber;

        emit DebugAddress(msg.sender);
        last = msg.sender;
    }

    function increment() public {
        number++;

        emit DebugAddress(msg.sender);
        last = msg.sender;
    }

    function decrement() public {
        require(number > 0, "Counter: decrement overflow");
        number--;

        emit DebugAddress(msg.sender);
        last = msg.sender;
    }

    function getNumber() public view returns (uint256) {
        return number;
    }

    function getLast() public view returns (address) {
        return last;
    }
}
EOF
cat /tmp/contracts/counter/Counter.sol
```
```sh
cd /tmp/
forge create \
--gas-price 700000000000 \
--priority-gas-price 10000000000 \
--private-key=56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027 \
--rpc-url=http://127.0.0.1:9650/ext/bc/C/rpc \
./contracts/counter/Counter.sol:Counter
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


