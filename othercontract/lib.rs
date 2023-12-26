#![cfg_attr(not(feature = "std"), no_std, no_main)]

// re-export the contract reference
pub use self::othercontract::OthercontractRef;

#[ink::contract]
mod othercontract {

    #[ink(storage)]
    pub struct Othercontract {
        value: bool,
    }

    impl Othercontract {
        
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

    }
}
