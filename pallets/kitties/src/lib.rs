#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{inherent::Vec, pallet_prelude::*};
use frame_system::pallet_prelude::*;

// Define Kitties
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
	pub dna: Vec<u8>,
	pub price: u64,
	pub gender: Gender,
	pub owner: T::AccountId,
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
	}

	// TODO : Define KittyId storage 
	// #[pallet::storage]
	// #[pallet::getter(fn kitty_id)]
	// pub(super) type KittyId<T: Config> =


	//TODO : Define Kitties storage + OptionQuery
	// #[pallet::storage]
	// #[pallet::getter(fn get_kitty)]
	// pub type Kitties<T: Config> =

	//TODO : Define KittiesOwned storage + ValueQuery
	// #[pallet::storage]
	// #[pallet::getter(fn kitty_owned)]
	// pub(super) type KittiesOwned<T: Config> =


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { kitty: Vec<u8>, owner: T::AccountId },

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DuplicateKitty,
		OverFlow,

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let owner = ensure_signed(origin)?;

			//TODO : generate gender 

			// TODO: define new kitty 
			

			// TODO: Check if the kitty does not already exist in our storage map
			// using ensure!
			// return DuplicateKitty if error


			// TODO: Get current kitty id 
			
			// TODO: Increase kitty Id by 1 (if overflow return OverFlow)

			// TODO: Append new kitty to KittiesOwned


			// TODO: Write new kitty to storage

			// TODO: Write new kitty id 


			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone()});

			Ok(())
		}
	}
}

impl<T> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender,Error<T>>{
		todo!()
	}
}