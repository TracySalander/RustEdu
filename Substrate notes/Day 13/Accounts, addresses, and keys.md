### Accounts, addresses, and keys
#### Address encoding and chain-specific addresses**

Substrate enables you to use a single public key to derive multiple addresses so you can interact with multiple chains without creating separate public and private key pairs for each network.
You can look up the chain-specific address for a public key using Subscan https://polkadot.subscan.io/tools/format_transform

#### Specialized accounts

Substrate supports some specialized accounts to control how specific key pairs can be used. For example, you might have accounts that require custom cryptographic schemes, can only be used to perform specific functions, or can only access specific pallets.

**Staking accounts and keys**

**Stash account:** (stake balance)

The stash account represents the public/private key pair that defines a staking balance for validators.

You should keep stash account keys offline and in cold storage for security.

You should not use the stash account to make frequent transactions.

You can designate a controller account to make non-spending decisions or a keyless proxy account (an account that is detached from any owner so that it can be used to perform autonomous transactions) to vote in governance on its behalf.

**Controller account:** (purpose, need a stash account in advance)

The controller account represents the public/private key pair that signals your intent to validate or nominate, sets preferences like the rewards destination and, in the case of validators, sets the session keys.

A controller account only needs to pay transaction fees, so it only needs minimal funds.

It can never be used to spend funds from its stash account.

**Session account:** (sign, need a stash account and a controller account in advance)

The session account represents the public/private key pair used by validators to sign consensus-related messages.
