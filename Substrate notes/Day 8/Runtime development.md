### Runtime development
#### State transitions and the runtime
**state transition function (STF)**
The logic of a blockchain that determines how the state changes when a block is processed. In Substrate, the state transition function is effectively equivalent to the runtime.

**state**
Cryptographically-secure data that persists between blocks and can be used to create new blocks as part of the state transition function. In Substrate-based blockchains, state is stored in a trie data structure that supports the efficient creation of incremental digests. This trie is exposed to the runtime as a simple key/value map where both keys and values can be arbitrary byte arrays.

**trie (Patricia Merkle Tree)**
A data structure that is used to represent sets of key-value pairs.

The Patricia Merkle trie data structure enables the items in the data set to be stored and retrieved using a cryptographic hash. Because incremental changes to the data set result in a new hash, retrieving data is efficient even if the data set is very large. With this data structure, you can also prove whether the data set includes any particular key-value pair without the access to the entire data set.

**key-value database**
Substrate implements its storage database with RocksDB, a persistent key-value store for fast storage environments. It also supports an experimental Parity DB.

The DB is used for all the components of Substrate that require persistent storage, such as:

- Substrate clients
- Substrate light-clients
- Off-chain workers

**RocksDB**
RocksDB is an embeddable persistent key-value store for fast storage.

RocksDB uses a log structured database engine, written entirely in C++, for maximum performance. Keys and values are just arbitrarily-seized byte streams.

RocksDB is optimized for fast, low latency storage such as flash drives and high-speed disk drives. RocksDB exploits the full potential of high read/write rates offered by flash or RAM.

RocksDB is adaptable to different workloads. From database storage engines such as MyRocks to application data caching to embedded workloads, RocksDB can be used for a variety of data needs.

RocksDB provides basic operations such as opening and closing a database, reading and writing to more advanced operations such as merging and compaction filters.

**ParityDB**
ParityDB is an embedded persistent key-value store optimized for blockchain applications.

Desgin considerations:

- The database is intended to be used for efficiently storing blockchain state encoded into the Patricia-Merkle trie. Which means most of the keys are fixed size and uniformly distributed. Most values are small. Values over 16 kbytes are rare. Trie nodes may be shared by multiple tries and/or branches, therefore reference counting is required.
- Read performance is more important than write performance for blockchain transaction throughput. Writes are typically performed in large batches, when the new block is imported. There's usually some time between subsequent writes when the blockchain client is idle or executes the block.

#### Runtime interface
The outer node is responsible for handling peer discovery, transaction pooling, block and transaction gossiping, consensus, and answering RPC calls from the outside world. These tasks frequently require the outer node to query the runtime for information or to provide information to the runtime. In Substrate, the `sp_api` crate provides an interface to implement a runtime API. It is designed to give you flexibility in defining your own custom interfaces using the `impl_runtime_apis` macro. However, evry runtime must implement the `Core` and `Metadata` interface.
