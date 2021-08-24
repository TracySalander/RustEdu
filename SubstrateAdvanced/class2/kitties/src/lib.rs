#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{Currency, Randomness, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded};

	#[derive(Encode, Decode)]
	pub struct Kitty(pub [u8; 16]);

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type KittyIndex: Parameter + AtLeast32BitUnsigned + Default + Copy + Bounded;
		type ReserveOfNewCreate: Get<BalanceOf<Self>>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T: Config> = StorageValue<_, T::KittyIndex>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_price)]
	pub type KittiesPrice<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, T::KittyIndex),
		KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
		KittyForSale(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
	}

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		InvalidKittyIndex,
		NotOwnerOfKitty,
		SameParentIndex,
		NotForSale,
		NotEnoughBalance,
		KittyAlreadyOwned,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				}
				None => 1u32.into(),
			};

			T::Currency::reserve(&who, T::ReserveOfNewCreate::get()).map_err(|_| Error::<T>::NotEnoughBalance)?;

			let dna = Self::random_value(&who);

			Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
			Owner::<T>::insert(kitty_id, Some(&who));
			KittiesCount::<T>::put(kitty_id + 1u32.into());

			Self::deposit_event(Event::KittyCreated(who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: T::KittyIndex,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let owner = Owner::<T>::get(&kitty_id).unwrap();
			ensure!(owner == sender, Error::<T>::NotOwnerOfKitty);

			Self::transfer_kitty(sender, to, kitty_id);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

			let owner1 = Self::owner(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let owner2 = Self::owner(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			ensure!(owner1 == who, Error::<T>::NotOwnerOfKitty);
			ensure!(owner2 == who, Error::<T>::NotOwnerOfKitty);

			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				}
				None => 1u32.into(),
			};

			let dna_1 = kitty1.0;
			let dna_2 = kitty2.0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8; 16];

			for i in 0..dna_1.len() {
				new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i])
			}

			Kitties::<T>::insert(kitty_id, Some(Kitty(new_dna)));
			Owner::<T>::insert(kitty_id, Some(&who));
			KittiesCount::<T>::put(kitty_id + 1u32.into());

			Self::deposit_event(Event::KittyCreated(who, kitty_id));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn sell(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Some(who.clone()) == Self::owner(kitty_id), Error::<T>::NotOwnerOfKitty);

			KittiesPrice::<T>::mutate_exists(kitty_id, |p| *p = Some(price));

			Self::deposit_event(Event::KittyForSale(who, kitty_id, price));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			let owner = Self::owner(kitty_id).unwrap();
			ensure!(owner != buyer.clone(), Error::<T>::KittyAlreadyOwned);

			let price = Self::kitties_price(kitty_id).ok_or(Error::<T>::NotForSale)?;

			let reserve = T::ReserveOfNewCreate::get();

			T::Currency::reserve(&buyer, reserve).map_err(|_| Error::<T>::NotEnoughBalance)?;

			T::Currency::unreserve(&owner, reserve);

			T::Currency::transfer(
				&buyer,
				&owner,
				price,
				frame_support::traits::ExistenceRequirement::KeepAlive,
			)?;

			KittiesPrice::<T>::remove(kitty_id);

			Self::transfer_kitty(owner, buyer, kitty_id);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(who: &T::AccountId) -> [u8; 16] {
			let payload =
				(T::Randomness::random_seed(), &who, <frame_system::Pallet<T>>::extrinsic_index());
			payload.using_encoded(blake2_128)
		}

		fn transfer_kitty(from: T::AccountId, to: T::AccountId, kitty_id: T::KittyIndex) {
			Owner::<T>::insert(kitty_id, Some(to.clone()));
			Self::deposit_event(Event::KittyTransferred(from, to, kitty_id));
		}
	}
}

