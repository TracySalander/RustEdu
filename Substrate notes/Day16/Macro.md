`#[pallet::config]` is mandatory to import and it must defined as following:
```
#[pallet::config]
pub trait Config: frame_system::Config + $optionally_some_other_traits{}
```
Inside this trait, you can define a type or constant.

If you want to define a constant, you need to add `#[constant]`. 

If you want to bound Get trait, you have to use `#[pallet::constant]`.

In order to make constants and their values appear in the runtime metadata, it is necessary to declare them with the `const` syntax.

The actual typse will be given in runtime code.

The type in Config can be used in the pallet by `T::type_name/constant_name`.

If the type is constant then you can use `T::constant_name::get()`.

Event must be bonded with `From<Event>` and `IsType<<Self as frame_system::Config>::Event>`

```
#[pallet::pallet]
#[pallet::generate_store(pub(super) trait Store)] // give the pallet right to access the storage
pub struct Pallet<T>(_);
```
This declaration of the pallet is a placeholder to implement traits and methods.

The macros will add the following macros
```
#[derive(
  frame_support::CloneNoBound,
  frame_support::EqNoBound,
  frame_support::PartialEqNoBound,
  frame_support::RuntimeDebugNoBound,
)]
```

It declare type alias for pallet, used by `construct_runtime`.

It implements `traits::PalletInfoAccess` on Pallet to easely access to pallet informations.

It implements `traits::StorageInfoTrait` and it can access to sotrage informations.

`#[pallet::storage]`
You can define abstract storage type and set them at the same time but the type must have generic T or T:Config.

The type has to be StorageValue, StorageMap or StorageDoubleMap.

You can add `#[pallet::getter(fn $my_getter_fn_name)]` so you can use get function in Pallet.
