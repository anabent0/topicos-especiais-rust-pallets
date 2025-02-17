pub (crate) use crate as pallet_template;
use frame_support::{derive_impl, parameter_types};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		TemplateModule: pallet_template,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

// Definindo os parâmetros para os valores máximos de comprimento de cada campo
parameter_types! {
    pub const MaxTitleLength: u32 = 100;
    pub const MaxAuthorLength: u32 = 11;
}

impl pallet_template::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxTitleLength = MaxTitleLength;
    type MaxAuthorLength = MaxAuthorLength;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
