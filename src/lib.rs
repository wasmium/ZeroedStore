// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.
// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

mod errors;
pub use errors::*;

mod store;
pub use store::*;

#[cfg(test)]

mod sanity_tests {
    use crate::{ZeroedStore, MARKER_LEN};
    use borsh::{BorshDeserialize, BorshSerialize};

    #[derive(Debug, BorshSerialize, BorshDeserialize, Default, PartialEq, Eq)]
    pub struct Foo {
        bar: Vec<[u8; 32]>,
    }

    #[test]
    fn mem_equals() {
        assert_eq!(
            ZeroedStore::<Foo>::size_of(),
            core::mem::size_of::<Foo>() + MARKER_LEN
        );
    }

    #[test]
    fn custom_data() {
        let baz = Foo {
            bar: vec![[1u8; 32], [3u8; 32], [2u8; 32]],
        };
        let mut foo = ZeroedStore::<Foo>::new();
        foo.data = baz;

        let mut buffer = [0u8; 108];
        assert!(foo.pack(&mut buffer).is_ok());

        let bar = ZeroedStore::<Foo>::unpack(&buffer);
        assert!(bar.is_ok());
        assert_eq!(foo, bar.unwrap());
    }
}
