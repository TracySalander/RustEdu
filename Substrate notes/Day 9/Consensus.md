### Consensus

#### Consensus in two phases
- **Block authoring** is the process nodes use to create new blocks (Aura, BABE, Proof of work)
- **Block finalization** is the process used to handle forks and choose the canonical chain (GRANDPA)

#### Consensus models
**Aura**
Aura provides a slot-based block authoring mechanism. In Aura a known set of authorities take turns producing blocks.

**BABE**
BABE provides slot-based block authoring with a known set of validators and is typically used in proof-of-stake blockchains. Unlike Aura, slot assignment is based on the evaluation of a Verifiable Random Function(VRF). Each validator is assigned a weight for an epoch. This epoch is broken up into slots and the validator evaluates its VRF at each slot. For each slot that the validator's VRF output is below its weight, it is allowed to author a block.

Because multiple validators might be able to produce a block during the same slot, forks are more common in BABE than they are in Aura, even in good network conditions.

Substrate's implementation of BABE also has a fallback mechanism for when no authorities are chosen in a given slot. These secondary slot assignments allow BABE to achieve a constant block time.

**Proof of work**
Proof-of-work block authoring is not slot-based and does not require a known authority set. In proof of work, anyone can produce a block at any time, so long as they can solve a computationally challenging problem (typically a hash preimage search).

**GRANDPA**
GRANDPA provides block finalization. After two-thirds of the GRANDPA authorities have voted for a particular block, it is considered final.
