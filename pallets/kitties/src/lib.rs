#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::dispatch::*;
use frame_support::{inherent::Vec, pallet_prelude::*, traits::ExistenceRequirement};
use frame_system::pallet_prelude::*;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
// use std::fmt::{Debug, Formatter, Result}; ko sử dụng dc

// Define Kitties
// #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
// #[scale_info(skip_type_params(T))]
// pub struct Kitty<AccountId> {
// 	pub dna: Vec<u8>,
// 	pub price: u64,
// 	pub gender: Gender,
// 	pub owner: AccountId
// }

use frame_support::traits::Currency;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::MyCurrency as Currency<AccountIdOf<T>>>::Balance;

#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
	pub dna: Vec<u8>,
	pub price: Option<BalanceOf<T>>,
	pub gender: Gender,
	pub owner: T::AccountId,
}

impl<T: Config> fmt::Debug for Kitty<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Kitty")
			.field("dna", &self.dna)
			.field("price", &self.price)
			.field("gender", &self.gender)
			.field("owner", &self.owner)
			.finish()
	}
}
// Define Gender
#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Gender {
	Male,
	Female,
}


#[frame_support::pallet]
pub mod pallet {

	use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type MyCurrency: Currency<Self::AccountId>;
	}

	// TODO : Define KittyId storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub(super) type KittyId<T> = StorageValue<_, u32, ValueQuery>;

	//TODO : Define Kitties storage + OptionQuery
	// dna => kitty
	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>>;

	//TODO : Define KittiesOwned storage + ValueQuery
	#[pallet::storage]
	#[pallet::getter(fn kitty_owned)]
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { kitty: Vec<u8>, owner: T::AccountId },
		// Transfer
		// Buy
		SetPrice { kitty: Vec<u8>, price: Option<BalanceOf<T>> },
		Sold { seller: T::AccountId, buyer: T::AccountId, kitty: Vec<u8>, price: BalanceOf<T> },
		Transferred { from: T::AccountId, to: T::AccountId, kitty: Vec<u8> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DuplicateKitty,
		OverFlow,
		NoKitty,
		NotOwner,
		TransferToSelf,
		BidPriceTooLow,
		NotForSale,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(5000)]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let owner = ensure_signed(origin)?;

			//TODO : generate gender
			let gender = Self::gen_gender(&dna)?;
			// TODO: define new kitty

			// TODO: Check if the kitty does not already exist in our storage map
			// using ensure!
			ensure!(!Kitties::<T>::contains_key(&dna), Error::<T>::DuplicateKitty);
			// return DuplicateKitty if error
			let new_kitty =
				Kitty::<T> { dna: dna.clone(), gender, price: None, owner: owner.clone() };
			log::info!("New kitty:{:?}", new_kitty);

			// TODO: Get current kitty id
			let current_id = Self::kitty_id();

			// TODO: Increase kitty Id by 1 (if overflow return OverFlow)
			let next_id = current_id.checked_add(1).ok_or(Error::<T>::OverFlow)?;
			// TODO: Append new kitty to KittiesOwned

			// let mut dnas = KittiesOwned::<T>::get(&owner);
			// dnas.push(dna.clone());
			// KittiesOwned::<T>::insert(&owner, dnas);

			KittiesOwned::<T>::append(&owner, dna.clone());
			// TODO: Write new kitty to storage
			Kitties::<T>::insert(&dna, new_kitty);
			// TODO: Write new kitty id
			KittyId::<T>::put(next_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn set_price(
			origin: OriginFor<T>,
			dna: Vec<u8>,
			amount: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;

			// get kitty
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			ensure!(kitty.owner == owner, Error::<T>::NotOwner);

			kitty.price = amount;
			Kitties::<T>::insert(&dna, kitty);

			Self::deposit_event(Event::SetPrice { kitty: dna, price: amount });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: Vec<u8>,
			price: BalanceOf<T>,
		) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let buyer = ensure_signed(origin)?;
			// Transfer the kitty from seller to buyer as a sale
			Self::do_transfer(kitty_id, buyer, Some(price))?;

			Ok(())
		}
	}
}

// helper function
impl<T: Config> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		if dna.len() % 2 == 0 {
			return Ok(Gender::Male);
		} else {
			return Ok(Gender::Female);
		}
	}

	pub fn do_transfer(
		kitty_id: Vec<u8>,
		to: T::AccountId,
		maybe_list_price: Option<BalanceOf<T>>,
	) -> DispatchResult {
		// Get the kitty
		let mut kitty = Kitties::<T>::get(&kitty_id).ok_or(Error::<T>::NoKitty)?;
		let from = kitty.owner;

		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut from_owned = KittiesOwned::<T>::get(&from);

		// Remove kitty from list of owned kitties.
		if let Some(ind) = from_owned.iter().position(|id| *id == kitty_id) {
			from_owned.swap_remove(ind);
		} else {
			return Err(Error::<T>::NoKitty.into());
		}

		// Add kitty to the list of owned kitties.
		let mut to_owned = KittiesOwned::<T>::get(&to);
		to_owned.push(kitty_id.clone());

		// Mutating state here via a balance transfer, so nothing is allowed to fail after this.
		// The buyer will always be charged the actual price. The limit_price parameter is just a
		// protection so the seller isn't able to front-run the transaction.
		if let Some(list_price) = maybe_list_price {
			// Current kitty price if for sale
			if let Some(price) = kitty.price {
				ensure!(list_price >= price, Error::<T>::BidPriceTooLow);
				// Transfer the amount from buyer to seller
				T::MyCurrency::transfer(&to, &from, price, ExistenceRequirement::KeepAlive)?;
				// Deposit sold event
				Self::deposit_event(Event::Sold {
					seller: from.clone(),
					buyer: to.clone(),
					kitty: kitty_id.clone(),
					price,
				});
			} else {
				// Kitty price is set to `None` and is not for sale
				return Err(Error::<T>::NotForSale.into());
			}
		}

		// Transfer succeeded, update the kitty owner and reset the price to `None`.
		kitty.owner = to.clone();
		kitty.price = None;

		// Write updates to storage
		Kitties::<T>::insert(&kitty_id, kitty);
		KittiesOwned::<T>::insert(&to, to_owned);
		KittiesOwned::<T>::insert(&from, from_owned);

		Self::deposit_event(Event::Transferred { from, to, kitty: kitty_id });

		Ok(())
	}
}
