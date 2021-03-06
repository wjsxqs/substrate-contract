#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{fmt::Debug, prelude::*};
use sp_runtime::{
	RuntimeDebug,
};
use frame_support::{
	ensure,
	dispatch::DispatchError,
};
use frame_support::debug;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use super::*;

	pub const MAX_CLAIM_SIZE: usize = 1024;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransfered(T::AccountId, T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ClaimTooLong,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// ensure!(claim.len() <= MAX_CLAIM_SIZE, Error::<T>::ClaimTooLong);
			//
			// ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
			//
			// Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number()));
			//
			// Self::deposit_event(Event::ClaimCreated(sender, claim));

			Self::do_create_claim(sender, claim)?;

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, to: T::AccountId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Proofs::<T>::insert(&claim, (to.clone(), block_number));

			Self::deposit_event(Event::ClaimTransfered(sender, to, claim));

			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_claim(who: T::AccountId, claim: Vec<u8>) -> Result<(), DispatchError> {

		debug::info!("run do_create_claim");
		debug::info!("who: {:?}", who);
		debug::info!("claim: {:?}", claim);

		ensure!(claim.len() <= MAX_CLAIM_SIZE, Error::<T>::ClaimTooLong);
		ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

		Proofs::<T>::insert(&claim, (who.clone(), frame_system::Module::<T>::block_number()));

		Self::deposit_event(Event::ClaimCreated(who, claim));

		Ok(())
	}

	pub fn do_transfer_claim(from: T::AccountId, to: T::AccountId, claim: Vec<u8>) -> Result<(), DispatchError> {
		debug::info!("run do_transfer_claim");
		debug::info!("from: {:?}", from);
		debug::info!("to: {:?}", to);
		debug::info!("claim: {:?}", claim);

		ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

		let (owner, block_number) = Proofs::<T>::get(&claim);

		ensure!(owner == from, Error::<T>::NotClaimOwner);

		Proofs::<T>::remove(&claim);

		Proofs::<T>::insert(&claim, (to.clone(), block_number));

		Self::deposit_event(Event::ClaimTransfered(from, to, claim));

		Ok(())
	}
}


// /// A module for proof of existence

// use frame_support::{
// 	decl_module, decl_storage, decl_event, decl_error, ensure,
// 	dispatch::DispatchResult,
// };
// use frame_system::{self as system, ensure_signed};
// use sp_std::vec::Vec;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// const MAX_CLAIM_SIZE: usize = 1024;

// /// The pallet's configuration trait.
// pub trait Trait: system::Trait {
// 	/// The overarching event type.
// 	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
// }

// // This pallet's storage items.
// decl_storage! {
// 	trait Store for Module<T: Trait> as PoeModule {
// 		Proofs get(fn proof): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
// 	}
// }

// // The pallet's events
// decl_event!(
// 	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
// 		ClaimCreated(AccountId, Vec<u8>),
// 		ClaimRevoked(AccountId, Vec<u8>),
// 		ClaimTransfered(AccountId, AccountId, Vec<u8>),
// 	}
// );

// // The pallet's errors
// decl_error! {
// 	pub enum Error for Module<T: Trait> {
// 		ProofAlreadyExist,
// 		ClaimNotExist,
// 		NotClaimOwner,
// 		ClaimTooLong,
// 	}
// }

// // The pallet's dispatchable functions.
// decl_module! {
// 	/// The module declaration.
// 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
// 		// Initializing errors
// 		// this includes information about your errors in the node's metadata.
// 		// it is needed only if you are using errors in your pallet
// 		type Error = Error<T>;

// 		// Initializing events
// 		// this is needed only if you are using events in your pallet
// 		fn deposit_event() = default;

// 		#[weight = 10_000]
// 		pub fn create_claim(origin, claim: Vec<u8>) -> DispatchResult {
// 			let sender = ensure_signed(origin)?;

// 			ensure!(claim.len() <= MAX_CLAIM_SIZE, Error::<T>::ClaimTooLong);

// 			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

// 			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

// 			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

// 			Ok(())
// 		}

// 		#[weight = 10_000]
// 		pub fn revoke_claim(origin, claim: Vec<u8>) -> DispatchResult {
// 			let sender = ensure_signed(origin)?;

// 			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

// 			let (owner, _) = Proofs::<T>::get(&claim);

// 			ensure!(owner == sender, Error::<T>::NotClaimOwner);

// 			Proofs::<T>::remove(&claim);

// 			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

// 			Ok(())
// 		}

// 		#[weight = 10_000]
// 		pub fn transfer_claim(origin, claim: Vec<u8>, to: T::AccountId, ) -> DispatchResult {
// 			let sender = ensure_signed(origin)?;

// 			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

// 			let (owner, block_number) = Proofs::<T>::get(&claim);

// 			ensure!(owner == sender, Error::<T>::NotClaimOwner);

// 			Proofs::<T>::remove(&claim);

// 			Proofs::<T>::insert(&claim, (to.clone(), block_number));

// 			Self::deposit_event(RawEvent::ClaimTransfered(sender, to, claim));

// 			Ok(())
// 		}

// 	}
// }
