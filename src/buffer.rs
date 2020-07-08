use std::collections::VecDeque;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;

pub struct Buffer<T> {
    data: VecDeque<u8>,
    phantom: PhantomData<T>,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            phantom: PhantomData,
        }
    }

    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.data.extend(slice.into_iter());
    }
}

impl<T: DeserializeOwned> Iterator for Buffer<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item_size = std::mem::size_of::<T>();

        if self.data.len() < item_size {
            return None;
        }

        let bytes = self.data.drain(..item_size).collect::<Vec<_>>();
        let decoded: T = bincode::deserialize(&bytes[..]).unwrap();
        Some(decoded)
    }
}
