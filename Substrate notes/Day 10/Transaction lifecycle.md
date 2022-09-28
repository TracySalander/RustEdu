#### Invalid transaction handling
If a transaction is invalid-for example, because it is too large or doesn't contain a valid signature-it is rejected and won't be added to a block. A transaction might be rejected for any of the following reasons:
- The transaction has already been included in a block so it is dropped from the verifying queue.
- The transaction's signature is invalid, so it is immediately be rejected.
- The transaction is too large to fit in the current block, so it is be put back in a queue for a new verification round.
