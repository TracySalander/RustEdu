### Rust for Substrate
- Rust is fast: it's statically typed at compile time, making it possible for the compiler to optimize the code for speed and for developers to optimize for a specific compilation target.
- Rust is portable: it's designed to run on embedded devices with support for any type of operating system.
- Rust is memory safe: it has no garbage collector and it checks each and every variable you use and every memory address you reference to avoid any memory leaks.
- Rust is Wasm first: it has first class support for compiling to WebAssembly.

`#![no_std]`is a crate-level attribute that indicates that the crate will link to the core-crate intead of the std-crate. The libcore crate in turn is a platform-agnostic subset of the std crate which makes no assumptions about the system the program will run on. As such, it provides APIs for language primitives like floats, strings and slices, as well as APIs that expose processor features like atomic operations and SIMD(Single instruction, multiple data) instructions. However it lacks APIs for anything that involves platform integration. Because of these properties no_std and libcore code can be used for any kind of bootstrapping code like bootloaders, firmware or kernels.

#### Generics and configuration traits
`Config` trait allows you to define the types and interfaces a pallet depends on.

This trait itself inherits a number of core runtime types from the `frame_system::pallet::Config` trait, making it easy to access common types when writing runtime logic. In addtion, in any FRAME pallet the `Config` trait is generic over `T` (more on generics in the next section). Some common examples of these core runtime types could be `T::AccountId`, the common type for identifying user accounts in the runtime or `T::BlockNumber`, the block number type used by the runtime.
