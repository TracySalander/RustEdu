```rust
/// 生产 Kitty
		/// 父母的编号不能相同
		/// ### Arguments
		/// * `origin` - 生产者
		/// * `kitty_id_1` - 父亲的编号
		/// * `kitty_id_2` - 母亲的编号
		#[pallet::weight(0)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyIndex,
			kitty_id_2: KittyIndex,
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
					ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				}
				None => 1
			};

			let dna_1 = kitty1.0;
			let dna_2 = kitty2.0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8; 16];

			// 按照父母生成新的DNA
			for i in 0..dna_1.len() {
				new_dna[i] = (selector[i] & dna_1[i]) | (!selector[i] & dna_2[i])
			}

			// 把这些数据都放到链上
			Kitties::<T>::insert(kitty_id, Some(Kitty(new_dna)));
			Owner::<T>::insert(kitty_id, Some(&who));
			KittiesCount::<T>::put(kitty_id);

			Self::deposit_event(Event::KittyCreated(who, kitty_id));

			Ok(())
		}
```

剩下runtime的lib.rs和Cargo.toml中有template的都复制改名为kitties
