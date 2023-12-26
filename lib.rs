#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod crosscontract {
    /// import othercontract
    use othercontract::OthercontractRef;

    /// Declare a reference to be part of our storage struct
    #[ink(storage)]
    pub struct Crosscontract {
        othercontract: OthercontractRef,
    }

    impl Crosscontract {
        
       /// Instantiate othercontract
       #[ink(constructor)]
       pub fn new(othercontract_code_hash: Hash, version: u32) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            let othercontract = OthercontractRef::new(true)
                .code_hash(othercontract_code_hash)
                .endowment(total_balance / 4)
                .salt_bytes(salt)
                .instantiate();

            Self { othercontract }
       }

       /// Reference othercontract methods
       #[ink(message)]
       pub fn flip_and_get(&mut self) -> bool {
        self.othercontract.flip();
        self.othercontract.get()
       }
    }
}
