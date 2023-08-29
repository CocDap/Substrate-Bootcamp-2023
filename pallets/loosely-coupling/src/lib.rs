#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use scale_info::prelude::vec::Vec;
use pallet_template::DoSomething;
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	



	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config   {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type IncreaseValue: DoSomething;
		type Amount: Get<u32>;
	}





	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingIncrease { amount: u32, who: T::AccountId },
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {


	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn increase_on_chain_pallet_template(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Increase value  on storage from pallet template.
			let amount = T::Amount::get();
			// 
			<<T as Config>::IncreaseValue>::increase_value(amount)?;
			//pallet_loosely_coupling::Config::IncreaseValue
			// Emit an event.
			Self::deposit_event(Event::SomethingIncrease { amount, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


	}
}


impl<T: Config> Pallet<T> {
		
	fn convert_str_to_slice(_str: &Vec<u8>) -> [u8; 32] {
		let bytes = _str;
		let mut array:  [u8; 32] = [0; 32];
		frame_support::log::info!("called by {:?}", bytes);
		 
		let mut length = 32;
		if bytes.len() < 32 {
			length = bytes.len();
		}
		
		for i in 0..length {
			array[i] = bytes[i];
		}


		return array;
	}
}

