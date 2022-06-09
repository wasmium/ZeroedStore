// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{StoreError, StoreResult};
use borsh::{BorshDeserialize, BorshSerialize};
use core::ops::RangeInclusive;

pub const MARKER: RangeInclusive<usize> = 0..=7;
pub const MARKER_LEN: usize = 8;

#[derive(Debug, PartialEq, Eq)]
pub struct ZeroedStore<T> {
    pub data: T,
}

impl<T> ZeroedStore<T>
where
    T: BorshDeserialize + BorshSerialize + Default + Sized,
{
    pub fn new() -> Self {
        ZeroedStore { data: T::default() }
    }

    pub fn pack(&self, buffer: &mut [u8]) -> StoreResult<usize> {
        let buffer_length = buffer.len();
        if buffer_length < MARKER_LEN {
            return Err(StoreError::BufferTooSmallForMarker);
        }
        let data = self.data.try_to_vec()?;
        let data_length = data.len();

        if buffer_length < data_length + MARKER_LEN {
            return Err(StoreError::BufferTooSmallForData {
                buffer_length,
                data_length,
            });
        }
        buffer[MARKER].copy_from_slice(&data_length.to_le_bytes());

        buffer[8..=data_length + 7].copy_from_slice(&data);

        Ok(data_length + 8usize)
    }

    pub fn unpack(buffer: &[u8]) -> StoreResult<ZeroedStore<T>> {
        let buffer_length = buffer.len();
        if buffer_length < MARKER_LEN {
            return Err(StoreError::BufferTooSmallForMarker);
        }

        let marker: [u8; 8] = match buffer[MARKER].try_into() {
            Ok(value) => value,
            Err(_) => return Err(StoreError::CorruptedMarker),
        };
        let byte_length = usize::from_le_bytes(marker);

        if byte_length > buffer_length {
            return Err(StoreError::CorruptedStorage {
                marker: byte_length,
                buffer_length,
            });
        }

        let data = buffer
            .iter()
            .skip(8)
            .take(byte_length)
            .map(|byte| *byte)
            .collect::<Vec<u8>>();

        if byte_length != 0 {
            let data = T::try_from_slice(&data)?;
            Ok(ZeroedStore { data })
        } else {
            Ok(ZeroedStore::<T>::new())
        }
    }

    #[cfg(feature = "non_constant_sizeof")]
    pub fn size_of() -> usize {
        core::mem::size_of::<T>() + MARKER_LEN
    }

    #[cfg(feature = "constant_sizeof")]
    pub const fn const_size_of() -> usize {
        core::mem::size_of::<T>() + MARKER_LEN
    }
}

impl<T> Default for ZeroedStore<T>
where
    T: BorshDeserialize + BorshSerialize + Default,
{
    fn default() -> Self {
        ZeroedStore::new()
    }
}
