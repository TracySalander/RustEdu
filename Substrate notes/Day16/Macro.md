```#[pallet::config]``` is mandatory to import and it must defined as following:
```
#[pallet::config]
pub trait Config: frame_system::Config + $optionally_some_other_traits{}
```
Event must be bonded with `From<Event>`
