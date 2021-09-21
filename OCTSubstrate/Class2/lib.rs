// if the feature is not std, then it must be no_std. The no_std feature is required for all pallets! This
// is because we are building a runtime module that must compile to WASM, and therefore cannot depend on rust's
// std dependencies.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet{
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	// Notice here that we are using the Vec<u8> type, which is normally included in the std Rust library. We cannot use
	// std! So instead, we have included use sp_std::vec::Vec; in our mod pallet. The sp-std crate includes many common
	// things that we desire from std, but are no_std compatible. To use this though, we must update our pallet's dependencies:
	// pallet/template/Cargo.toml
	use sp_std::vec::Vec;

	// Every pallet has a component called Config, that is used for configuration.
	// This component is a Rust "trait": For now, the only thing we will configure about our pallet is that
	// it will emit some Events.
	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// Now that we've configured our pallet to emit events, let's go ahead and define those events. Our pallet will only emit an
	// event in two circumstances:
	// 1. When a new proof is added to the blockchain
	// 2. When a proof is removed.
	// The events can contain some additional data, in this case, each event will also display who triggered the event
	// (AccountId), and the proof data (as Vec<u8>) that is being stored or removed. Note that convention is to include an array
	// with descriptive names for these parameters at the end of event documentation.
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>{
		// Event emitted when a proof has been claimed. [who, claim]
		ClaimCreated(T::AccountId, Vec<u8>),
		// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(T::AccountId, Vec<u8>),
	}

	// The events we defined previously indicate when calls to the pallet have completed successfully. Similarly, errors indicate when
	// a call has failed, and why it has failed.
	// The first of these errors can occur when attempting to claim a new proof. Of course a user cannot claim a proof that has already been
	// claimed. The latter two can occur when attempting to revoke a proof.
	#[pallet::error]
	pub enum Error<T>{
		// The proof has already been claimed.
		ProofAlreadyClaimed,
		// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,
	}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// To add a new proof to the blockchain, we will simply store that proof in our pallet's storage. To store that value, we will create a
	// hash map from the proof to the owner of that proof and the block number the proof was made. We'll be using FRAME's StorageMap to keep track of
	// this information.
	#[pallet::storage]
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>{}

	// As implied by our pallet's events and errors, we will have two "dispatchable functions"
	// the user can call in this FRAME pallet:
	// 1. create_claim(): Allow a user to claim the existence of a file with a proof
	// 2, revoke_claim(): Allow the owner of a claim to revoke their claim.
	// These functions will be based on using the StorageMap based on the following logic: if a proof has an owner and a block number,
	// then we know that it has been claimed. Otherwise, the proof is available to be claimed (and written to storage).
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics",
	// which are often compared to transactions. 
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T>{
		#[pallet::weight(1_000)]
		pub(super) fn create_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResultWithPostInfo{

			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has not already been claimed.
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			// Get the block number from the FRAME System module.
			let current_block = <frame_system::Module<T>>::block_number();

			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&proof, (&sender, current_block));

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(sender, proof));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		fn revoke_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResultWithPostInfo{
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

			// Get owner of the claim
			let (owner, _) = Proofs::<T>::get(&proof);

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			// Remove claim from storage.
			Proofs::<T>::remove(&proof);

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender, proof));

			Ok(().into())
		}
	}
}