#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(PSP37, PSP37Burnable)]
#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PerpToken {
    	#[storage_field]
		psp37: psp37::Data,
    }
    
    impl PerpToken {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance
        }
    }
}