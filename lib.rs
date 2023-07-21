#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(incomplete_features)]
#![feature(specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use ink::env::hash;
    use ink::prelude::vec::Vec;
    use openbrush::traits::Storage;
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        random_number: u128,
        salt: u128,
        max_value: u128,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(max_value: u128) -> Self {
            let mut instance = Self::default();
            instance.max_value = max_value;
            instance
        }

        #[ink(message)]
        pub fn generate_random_number(&mut self) -> u128 {
            let seed = self.env().block_timestamp();
            let mut input: Vec<u8> = Vec::new();
            input.extend_from_slice(&seed.to_be_bytes());
            input.extend_from_slice(&self.salt.to_be_bytes());
            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Keccak256>(&input, &mut output);
            self.salt += 1;
            let number = output[0] as u128 % (self.max_value + 1);
            self.random_number = number;
            number
        }

        #[ink(message)]
        pub fn get_random_number(&self) -> u128 {
            self.random_number
        }

        #[ink(message)]
        pub fn get_max_value(&self) -> u128 {
            self.max_value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_pseudo_random() {
            let max_value: u128 = 15000;
            let mut contract = Contract::new(max_value);
            let result = contract.generate_random_number();
            assert!(result <= max_value);
        }
    }
}
