#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_prelude::vec::Vec;

#[ink::contract]
mod my_psp34 {
    use ink_prelude::string::String;
    use ink_prelude::collections::BTreeMap;
    use ink_prelude::vec::Vec;
    use ink_psp34::{PSP34, Psp34Error, Psp34Result};

    #[derive(scale::Encode, scale::Decode, Default)]
    pub struct Item {
        name: String,
        quantity: u64,
        owner: AccountId,
        previous_owner: Option<AccountId>,
        location: String,
    }

    #[ink(storage)]
    pub struct SimpleSupplyChain {
        items: ink_storage::collections::HashMap<u64, Item>,
        next_item_id: u64,
    }

    impl PSP34 for SimpleSupplyChain {
        type Item = Item;

        fn create_item(&mut self, name: String, quantity: u64, owner: AccountId) -> Psp34Result<Self::Item> {
            let item = Item {
                name,
                quantity,
                owner,
                previous_owner: None,
                location: String::new(),
            };

            let id = self.next_item_id;
            self.items.insert(id, item);

            self.next_item_id += 1;

            Ok(self.items.get(&id).unwrap().to_owned())
        }

        fn get_item(&self, id: u64) -> Psp34Result<Self::Item> {
            self.items.get(&id)
                .map(|item| item.to_owned())
                .ok_or(Psp34Error::ItemNotFound)
        }
    }

    impl SimpleSupplyChain {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                items: ink_storage::collections::HashMap::new(),
                next_item_id: 0,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::test;

        #[test]
        fn create_item_works() {
            // given
            let mut contract = SimpleSupplyChain::new();
            let owner = test::get_caller();

            // when
            let item_result = contract.create_item("Item 1".into(), 10, owner);

            // then
            assert!(item_result.is_ok());

            let item = item_result.unwrap();
            assert_eq!(item.name, "Item 1");
            assert_eq!(item.quantity, 10);
            assert_eq!(item.owner, owner);

            let retrieved_item_result = contract.get_item(0);
            assert!(retrieved_item_result.is_ok());

            let retrieved_item = retrieved_item_result.unwrap();
            assert_eq!(retrieved_item, item);
        }

        #[test]
        fn get_item_works() {
            // given
            let mut contract = SimpleSupplyChain::new();
            let owner = test::get_caller();

            let item = Item {
                name: "Item 2".into(),
                quantity: 5,
                owner,
                previous_owner: None,
                location: "Location 1".into(),
            };

            contract.items.insert(0, item.clone());

            // when
            let retrieved_item_result = contract.get_item(0);

            // then
            assert!(retrieved_item_result.is_ok());

            let retrieved_item = retrieved_item_result.unwrap();
            assert_eq!(retrieved_item, item);
        }
    }

}