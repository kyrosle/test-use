use std::io::{IoSlice, IoSliceMut};

use rand::random;

use crate::DATA_LEN;

pub fn create_write_slices<'a>(n: usize) -> Vec<IoSlice<'a>> {
  let mut vec = Vec::with_capacity(n);
  for _ in 0..n {
    let num = random::<u8>();
    let slice = Box::new([num; DATA_LEN]);
    let slice: &'a [u8] = Box::leak(slice);
    vec.push(IoSlice::new(slice));
  }
  vec
}

#[allow(dead_code)]
pub fn create_io_slice(data: Vec<&[u8]>) -> Vec<IoSlice> {
  data.into_iter().map(IoSlice::new).collect::<Vec<_>>()
}

pub fn create_mut_slices(n: usize) -> Vec<[u8; DATA_LEN]> {
  (0..n).map(|_| [0; DATA_LEN]).collect()
}

pub fn create_io_mut_slice<'a>(
  data: &'a mut [&'a mut [u8]],
) -> Vec<IoSliceMut<'a>> {
  data.iter_mut().map(|s| IoSliceMut::new(s)).collect()
}

pub fn create_metadata<'a>(n: usize) -> Vec<&'a [u8]> {
  let mut vec = Vec::with_capacity(n);
  for _ in 0..n {
    let num = random::<u8>();
    let slice = Box::new([num; DATA_LEN]);
    let slice: &'a [u8] = Box::leak(slice);
    vec.push(slice);
  }
  vec
}
