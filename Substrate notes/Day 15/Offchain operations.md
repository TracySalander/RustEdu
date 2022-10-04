### Offchain operations
**Offchain workers** are a subsystem of components that enable the execution of long-running and possibly non-deterministic tasks, such as:
- website service requests
- encryption, decryption and signing of data
- random number generation
- CPU-intensive computations
- enumeration or aggregation of on-chain data Offchain workers enable you to move tasks that might require more time to execute than allowed out of the block processing pipeline. Any task that might take longer than the maximum block execution time allowed is a reasonable candidate for offchain processing.

**Offchain storage** is storage that is local to a Substrate node and can be accessed by both offchain workers and on-chain logic:
- Offchain workers have both read and write access to offchain storage and are spawned during each block import.
- On-chain logic has write access through offchain indexing but doesn't have read access. The offchain storage allows different worker threads to communicate with each other and to store user-specific or node-specific data that does not require consensus over the whole network.
- It can be read using remote procedure calls (RPC)

**Offchain indexing** is an optional service that allows the runtime to write directly to the offchain storage independently from offchain workers. The offchain index provides temporary storage for on-chain logic and complements the on-chain state.
