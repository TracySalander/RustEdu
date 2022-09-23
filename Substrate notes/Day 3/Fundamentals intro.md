#### Fundamentals
- Polkadot is a layer-0 protocol and multichain network laying the foundation for the future of Web3. Substrate is the primary blockchain SDK used by developers to create the parachains that make up the Polkadot network.

Layer 0 is the initial stage of blockchain that allows various networks to function, such as Bitcoin, Ethereum, and many more. Layer 0 also provides blockchain with a facility of cross-chain interoperability communication from top to different layers. Layer 0 provides the underlying infrastructure for blockchain.
(Polkadot protocol, you can run other public chains on top which are called parachains in Polkadot)

Layer 1 is the implementation layer, like Bitcoin, Ethereum, Solana, Avalanche, NEAR. They are individual chains and they take responsibility for their own security.

Layer 2 is the apps on top of Layer 1, for example Unisawp, Compound are apps on top of Ethereum. Layer 2's security is based on Layer 1. It's a way to release the burden of Layer 1. It will deal with some transactions on Layer 2 and give the results back to Layer 1. Lower transaction fee, better UX. 

Sidechain is like an individual Layer 2 and has its own nodes, consensus, architecture. It's a second way to make Layer 1 better. Use bridges to communicate with Layer 1.
For example, you want to use Sidechain, you need to put your tokens in a specific account in Layer 1 and it will be locked. Then, side chain will generate tokens by a rule and you can use these tokens in side chain. Once you finish your operations, the tokens in side chain will burned and the Layer 1's token will be released.
(Ronin, Octopus Network)

- Building a custom blockchain with Substrate offers greater freedom, flexibility, and optimization than building on top of a general-purpose smart-contract blockchain.
