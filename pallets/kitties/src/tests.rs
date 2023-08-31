use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty_should_work() {
	new_test_ext().execute_with(|| {
		let dna1 = vec![1];
		System::set_block_number(1);
		// create a kitty by 1 
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(1),dna1.clone() ));

		System::assert_last_event(Event::Created { kitty: dna1.clone(), owner: 1 }.into());
		// check onchain storage có như mình mong muốn hay ko 

		let current_id =KittiesModule::kitty_id();
		assert_eq!(current_id, 1);
		assert!(KittiesModule::get_kitty(dna1.clone()).is_some());
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		assert_eq!(kitty.owner, 1);
		let kitties_owned = KittiesModule::kitty_owned(1);
		assert_eq!(kitties_owned, vec![dna1.clone()]);

		//create a kitty by 2
		let dna2 = vec![2];
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(2),dna2.clone() ));
		let current_id =KittiesModule::kitty_id();
		assert_eq!(current_id, 2);

		assert!(KittiesModule::get_kitty(dna2.clone()).is_some());
		let kitty = KittiesModule::get_kitty(dna2.clone()).unwrap();
		assert_eq!(kitty.owner, 2);

		//create a kitty by 1
		let dna3 = vec![3];
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(1),dna3.clone() ));
		let kitties_owned = KittiesModule::kitty_owned(1);
		assert_eq!(kitties_owned, vec![dna1, dna3]);

	});
}

#[test]
fn set_price_should_work() {
	new_test_ext().execute_with(|| {
		let dna1 = vec![1];
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(1),dna1.clone() ));
		assert!(KittiesModule::get_kitty(dna1.clone()).is_some());
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		assert_eq!(kitty.price, 0);

		
		assert_ok!(KittiesModule::set_price(RuntimeOrigin::signed(1),dna1.clone(), 100 ));
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		assert_eq!(kitty.price, 100);


	});


}
