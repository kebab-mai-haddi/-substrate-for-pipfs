use frame_support::{decl_module, decl_storage, dispatch::result::Result, ensure, StorageMap};
use frame_system::ensure_signed;
use sp_runtime::DispatchError;

// pub trait Trait: balances::Trait {}
pub trait Trait: pallet_balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Value: map T::Hash => Option<T::AccountId>;
        // TODO: check whether this is the appropriate datatype(hash).
        Value: map hasher(blake2_256) T::Hash => Option<T::AccountId>;
        // Balances: map hasher(blake2_256) (T::AssetId, T::AccountId) => T::Balance;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_value(origin, value: T::Hash) -> Result<(), DispatchError> {
            let sender = ensure_signed(origin)?;
            ensure!(!<Value<T>>::contains_key(value), "key already exists");
            <Value<T>>::insert(value, sender);
            Ok(())
        }
    }
}
