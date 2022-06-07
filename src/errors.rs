// © Wasmium Network Developers

// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at https://mozilla.org/MPL/2.0/.

use borsh::maybestd::io::{Error as BorshIoError, ErrorKind as BorshErrorKind};

pub type StoreResult<T> = Result<T, StoreError>;

#[derive(Debug)]
pub enum StoreError {
    CorruptedMarker,
    /// The `unpack`er expected more data as indicated by the value of the `MARKER`
    CorruptedStorage {
        marker: usize,
        buffer_length: usize,
    },
    Borsh(BorshErrorKind),
    BufferTooSmallForMarker,
    BufferTooSmallForData {
        buffer_length: usize,
        data_length: usize,
    },
}

impl From<BorshIoError> for StoreError {
    fn from(error: BorshIoError) -> Self {
        StoreError::Borsh(error.kind())
    }
}
