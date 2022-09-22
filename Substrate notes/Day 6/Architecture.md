### Architecture
#### High level overview
At a high level, a Substrate node provides a layered environmnet with two main elements:
- An outer node that handles network activity such as peer discovery, managing transaction requests, reaching consensus with peers, and responding to RPC calls.
- A runtime that contains all of the business logic for executing the state transition function of the blockchain.

##### Outer node
- Storage: The outer node persists the evolving state of a Substrate blockchain using a simple and highly efficient key-value storage layer.
- Peer-to-peer networking: The outer node uses the Rust implementation of the libp2p network stack to communicate with other network participants.
- Consensus: The ourer node communicates with other network participants to ensure they agree on the state of the blockchain.
- Remote procedure call (RPC) API: The outer node accepts inbound HTTP and WebSocket requests to allow blockchain users to interact with the network.
- Telemetry: The outer node collects and provides access to node metrics through an embedded Prometheus server.
- Execution environment: The outer node is responsible for selecting the execution environment-WebAssembly or native Rust-for the runtime to use then dispatching calls to the runtime selected.

##### Runtime
The Substrate runtime is designed to compile to WebAssembly (Wasm) byte code. This design decision enables:
- Support for forkless upgrades
- Multi-platform compatibility
- Runtime validity checking
- Validation proofs for relay chain consensus mechanisms

#### Light client nodes
It is a simplified version of a Substrate node that only provides the runtime and current state.
