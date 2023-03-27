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
}