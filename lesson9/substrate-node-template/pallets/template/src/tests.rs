// Tests to be written here

use crate::{mock::*};

#[test]
fn test_offchain() {
	let (mut t, _pool_state, _offchain_state) = ExtBuilder::build();
	t.execute_with(|| {
		let r = TemplateModule::fetch(true);
		assert!(r.is_ok());
		let r = TemplateModule::fetch(true);
		assert!(r.is_ok());
	});
}
