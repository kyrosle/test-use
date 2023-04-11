use std::{
  fs::File,
  io::{self, IoSlice, IoSliceMut},
};

use crate::vector_io::pwritev;

pub enum IoVecsType<'a> {
  Immutable(&'a mut [IoSlice<'a>]),
  Mutable(&'a mut [IoSliceMut<'a>]),
}

impl<'a> IoVecsType<'a> {
  fn is_empty(&self) -> bool {
    match self {
      IoVecsType::Immutable(iovecs) => !iovecs.iter().any(|i| i.len() > 0),
      IoVecsType::Mutable(iovecs) => !iovecs.iter().any(|i| i.len() > 0),
    }
  }

  fn as_write(&'a self) -> Option<&'a [IoSlice]> {
    match self {
      IoVecsType::Immutable(iovecs) => Some(iovecs),
      IoVecsType::Mutable(_) => None,
    }
  }

  fn as_read(&'a mut self) -> Option<&'a mut [IoSliceMut]> {
    match self {
      IoVecsType::Immutable(_) => None,
      IoVecsType::Mutable(iovecs) => Some(iovecs),
    }
  }
}
pub struct IoVecs<'a> {
  pub iovecs_cnt: usize,
  pub iovecs: IoVecsType<'a>,
}

impl<'a> IoVecs<'a> {
  pub fn new_immutable(iovecs: &'a mut [&'a [u8]]) -> Self {
    let mut cnt = 0;
    let iovecs = Box::new(
      iovecs
        .iter()
        .map(|i| {
          cnt += i.len();
          IoSlice::new(i)
        })
        .collect::<Vec<_>>(),
    )
    .leak();

    IoVecs {
      iovecs_cnt: cnt,
      iovecs: IoVecsType::Immutable(iovecs),
    }
  }

  pub fn new_mutable(iovecs: &'a mut [&'a mut [u8]]) -> Self {
    let mut cnt = 0;
    let iovecs = Box::new(
      iovecs
        .iter_mut()
        .map(|i| {
          cnt += i.len();
          IoSliceMut::new(i)
        })
        .collect::<Vec<_>>(),
    )
    .leak();

    IoVecs {
      iovecs_cnt: cnt,
      iovecs: IoVecsType::Mutable(iovecs),
    }
  }

  pub fn advance(&mut self, step: usize) {
    let iovecs = match self.iovecs {
      IoVecsType::Immutable(iovecs) => {
        IoVecsType::Immutable(crate::advance(step, iovecs))
      }
      IoVecsType::Mutable(iovecs) => {
        IoVecsType::Mutable(crate::advance_mut(step, iovecs))
      }
    };
    self.iovecs_cnt -= step;
    self.iovecs = iovecs;
  }

  pub fn as_slice(&'a self) -> &'a [IoSlice<'a>] {
    self.iovecs.as_write().unwrap()
  }

  pub fn write_vectored(
    &'a mut self,
    file: &File,
    offset: usize,
  ) -> io::Result<()> {
    loop {
      let iovecs = self.as_slice();
      let write_in = pwritev(file, iovecs, offset)?;
      self.advance(write_in);
      if self.is_empty() {
        return Ok(());
      }
    }
  }

  pub fn is_empty(&self) -> bool {
    self.iovecs_cnt == 0 || self.iovecs.is_empty()
  }
}
