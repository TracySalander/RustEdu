### Networks and blockchains
#### Node types
Full ndoes: by default. full nodes are configured to store only the most recent 256 blocks and to discard state older than that.

Although older blocks are discarded, full nodes retain all of the block headers from the genesis block to the most recent block to validate that the state is correct.

A full node requires much less disk space than an archive node. However, a full node requires far more computational resources to query and retrieve information about some previous state. If you need to query historical blocks, you should purge the full node then restart it as an archive node.

Use cases: receving and verifying transactions, authoring and validating blocks

#### Archive nodes
Archive nodes store all past blocks with complete state available for every block. 

Use cases: Block explorers, discussion forums

#### Light client nodes
Light client nodes provide a runtime and access to the current state through RPC endpoints.

Light client nodes don't participate in blockchain or network operations.
Use cases: browser extensions, mobile device applications, reading block headers, submitting transactions, viewing the results of transactions.
