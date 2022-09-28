### Transactions and block basics

#### Transactions
There are three distinct transaction types in Substrate:
- Signed transactions (most common, usually pay a fee)
- Unsigned transactions (no fee, hard to prevent spam or replay attacks, require custom validation, consumes more resources than a signed transaction)
- Inherent transactions (special unsigned transaction, it is not gossiped to other nodes or stored in the transaction queuse and is assumed to be valid without requiring specific validation)

### Blocks
A block consists of a header and an array of transactions. The header contains the followin properties:
- Block height
- Parent hash
- Transaction root 
- State root 
- Digest

```
pub struct Header<Number: Copy + Into<U256> + TryFrom<U256>, Hash: HashT> {
    /// The parent hash.
    pub parent_hash: Hash::Output,
    /// The block number. (Block hight)
    #[cfg_attr(feature = "std", serde(
        serialize_with = "serialize_number",
        deserialize_with = "deserialize_number"))]
    pub number: Number, 
    /// The state trie merkle root (State root) The root will link to the old unchanged values and add new link to the changed new values.
    pub state_root: Hash::Output,
    /// The merkle root of the extrinsics. (Transaction root) A abstract of a senquence of transactions.
    pub extrinsics_root: Hash::Output,
    /// A chain-specific digest of data useful for light clients or referencing auxiliary data. It is a set of additional data, each chain is different. Usually put consensus relating info. 
    pub digest: Digest<Hash::Output>,
}
```
