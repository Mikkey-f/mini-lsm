// Copyright (c) 2022-2025 Alex Chi Z
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::Bytes;
pub use iterator::BlockIterator;

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted key-value pairs.
pub struct Block {
    pub(crate) data: Vec<u8>,
    pub(crate) offsets: Vec<u16>,
}

impl Block {
    /// Encode the internal data to the data layout illustrated in the course
    /// Note: You may want to recheck if any of the expected field is missing from your output
    pub fn encode(&self) -> Bytes {
        let mut buffer = Vec::with_capacity(self.data.len() + self.offsets.len() * 2 + 2);

        buffer.extend_from_slice(&self.data);

        for offset in &self.offsets {
            buffer.extend_from_slice(&offset.to_le_bytes())
        }

        buffer.extend_from_slice(&(self.offsets.len() as u16).to_le_bytes());

        Bytes::from(buffer)
    }

    /// Decode from the data layout, transform the input `data` to a single `Block`
    pub fn decode(data: &[u8]) -> Self {
        let num_of_elements_start = &data[data.len() - 2..];
        let num_of_elements =
            u16::from_le_bytes([num_of_elements_start[0], num_of_elements_start[1]]);

        let offset_start = data.len() - ((num_of_elements * 2) as usize) - 2;
        let mut offsets = Vec::with_capacity(num_of_elements as usize);

        for i in 0..num_of_elements {
            let pos = offset_start + ((i * 2) as usize);
            offsets.push(u16::from_le_bytes([data[pos], data[pos + 1]]));
        }

        let data_ = data[0..offset_start].to_vec();
        Self {
            data: data_,
            offsets,
        }
    }
}
