# Welcome to Glitch Hackathon 2023

[글리치 해커톤 2023](https://glitch-hack.com) 이벤트에 오신 것을 환영합니다! 이 페이지는 [아발란체 트랙 미션](https://trinity-studio.notion.site/GLITCH-KOREA-TRACK-GUIDE-1509d049e1b84ef49f737af5398541c8)에 참가하실 때 필요한 모든 자료들을 포함하고 있습니다. 해커톤 용 인프라 이용을 위해서 다음 링크들을 순서대로 참조해주세요 (예: 테스트 토큰, 지갑 연동, 체인 RPC들).

Welcome to [Glitch Hackathon 2023](https://glitch-hack.com)! This page contains all the information you need to participate in the [Avalanche track mission](https://trinity-studio.notion.site/GLITCH-KOREA-TRACK-GUIDE-1509d049e1b84ef49f737af5398541c8). Please check out the following links to get access to our Hackathon infra (e.g., test tokens, wallet integration, chain RPCs).

## (Optional) Step 1. Connect to local DEVNET and fund the wallet

아발란체는 테스트용으로 셋업한, 단기간만 쓰도록 설계된 네트워크를 데브넷이라 칭합니다.

_In the world of Avalanche, we refer to short-lived, test Subnets as Devnets._ (source [HyperSDK](https://github.com/ava-labs/hypersdk/blob/0907bf7c016c3ab569952201270e37cdfb8592b1/examples/tokenvm/DEVNETS.md))

로컬 아발란체 네트워크를 셋업해서, 빠른 환경에서 EVM 컨트랙트들을 테스트 할 수 있습니다. 아래 문서는 로컬 데브넷에 연결 및 지갑 연동을 어떻게 하는지를 설명합니다. 퍼블릭 테스트(Fuji)/메인넷 또는 [글리치 해커톤 전용 데브넷](#step-2-connect-to-glitch-hackathon-devnet-and-fund-the-wallet)을 이용하시는 경우, 이 단계는 건너 뛰어 주세요. 이 단계에서는 개발용 지갑에 앱 개발 및 컨트랙 배포에 필요한 토큰을 어떻게 충전하는지도 설명합니다.

You can set up a local Avalanche network to quickly test your EVM contracts. The links below show how to connect to the local DEVNET and use a web wallet with the local DEVNET. If you are using public test(Fuji)/mainnet or [Glitch Hackathon DEVNET](#step-2-connect-to-glitch-hackathon-devnet-and-fund-the-wallet), you can skip this part. This step also explains how to fund your wallet that is needed for building apps and contract deployment.

- [영어 문서 (English version)](./docs/1-connect-to-local-devnet-and-fund-the-wallet.eng.md)

## Step 2. Connect to Glitch Hackathon DEVNET and fund the wallet

아발란체는 테스트용으로 셋업한, 단기간만 쓰도록 설계된 네트워크를 데브넷이라 칭합니다.

_In the world of Avalanche, we refer to short-lived, test Subnets as Devnets._ (source [HyperSDK](https://github.com/ava-labs/hypersdk/blob/0907bf7c016c3ab569952201270e37cdfb8592b1/examples/tokenvm/DEVNETS.md))

아바 랩스에서는 글리치 해커톤 전용 데브넷을 준비했습니다. 모든 참가자들한테는 무한정 테스트 토큰이 지원되어, 스마트 컨트랙트 및 커스텀 가상머신 개발 및 테스트에 용이합니다. 아래 문서는 글리치 데브넷에 연결 및 지갑 연동을 어떻게 하는지를 설명합니다. _위에 설명된 로컬 데브넷 또는 퍼블릭 테스트(Fuji)/메인넷을 사용하실 경우 이 단계는 건너 뛰어 주세요._ 이 단계에서는 개발용 지갑에 앱 개발 및 컨트랙 배포에 필요한 토큰을 어떻게 충전하는지도 설명합니다.

Ava Labs has set up a public DEVNET for Glitch Hackathon. All participants will get unlimited tokens, to easily test smart contracts and custom VM development. The links below show how to connect to the Glitch DEVNET and use a web wallet with the Glitch DEVNET. _If you are using the local DEVNET above or public test(Fuji)/mainnet, you can skip this part._ This step also explains how to fund your wallet that is needed for building apps and contract deployment.

*해당 데브넷은 `Tue May 23 23:00:00 UTC 2023` 경 내릴 예정입니다. 데모 보존 등을 위해서는 스크린 레코딩 등을 이용해 주세요.*

*This DEVNET will be shut down at `Tue May 23 23:00:00 UTC 2023`. To keep your demos, please use screen recording.*

- [영어 문서 (English version)](./docs/2-connect-to-glitch-devnet-and-fund-the-wallet.eng.md)

## Step 3. Test simple smart contract

간단한 EVM 스마트 컨트랙트를 자신만의 지갑으로 배포해보고, 배포된 컨트랙을 어떻게 실행하는지를 테스트 합니다.

Let's deploy a simple EVM smart contract with your wallet, and see how to execute that contract.

- [영어 문서 (English version)](./docs/3-test-simple-smart-contract.eng.md)

## Step 4. Test gasless transaction

(이 단계는 [글리치 해커톤 데브넷](#step-2-connect-to-glitch-hackathon-devnet-and-fund-the-wallet)을 필요로 합니다. 가스비 없는 메타 트랙잭션을 사용하지 않는다면 이 단계를 건너 뛰어 주세요.)

(This step requires [Glitch Hackathon DEVNET](#step-2-connect-to-glitch-hackathon-devnet-and-fund-the-wallet). If you do not plan to use no-gas-fee meta transaction, please skip this step.)

아바 랩스에서 배포한 가스 릴레이 서버를 사용해서, 가스비 없이도 유저들이 거래를 할 수 있는 컨트랙을 배포합니다. EVM 체인의 네이티브 토큰이 없이도 유저들이 스마트 컨트랙을 사용할 수 있기 때문에 (예, NFT 민팅), 웹2와 같은 유저 경험을 제공합니다.

Try the gas relay server from Ava Labs, to deploy contracts with gasless transactions. Even users with no EVM native token can interact with the smart contracts (e.g., NFT minting), which provides web2-like user experience.

- [영어 문서 (English version)](./docs/4-test-gasless-transaction.eng.md)
