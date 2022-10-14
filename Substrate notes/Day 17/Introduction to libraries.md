### Introduction to libraries
#### Core node libraries

The libraries that enable a Substrate node to handle its network respinsibilities: (You can check the Cargo.html)

`sc_xxx`: consensus and block execution.

`sp_xxx`: communication layer between the outer node and the runtime.

`frame_xxx` (optional): build the runtime logic and to encode and decode the information passed into and out of the runtime.

`pallet_xxx` (optional): a single FRAME module.
