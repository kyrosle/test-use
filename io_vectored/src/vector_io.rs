use std::{
  fs::File,
  io::{IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write},
};

pub fn pwritev(
  mut file: &File,
  vecs: &[IoSlice],
  offset: usize,
) -> std::io::Result<usize> {
  file.seek(SeekFrom::Start(offset as u64))?;
  file.write_vectored(vecs)
}

pub fn preadv(
  mut file: &File,
  vecs: &mut [IoSliceMut],
  offset: usize,
) -> std::io::Result<usize> {
  file.seek(SeekFrom::Start(offset as u64))?;
  file.read_vectored(vecs)
}

pub fn advance<'a>(
  step: usize,
  io_slice: &'a mut [IoSlice<'a>],
) -> &'a mut [IoSlice<'a>] {
  let mut current_start = None;
  let mut step_len = step;
  for (index, slice) in io_slice.iter().enumerate() {
    let len = slice.len();
    if len <= step_len {
      step_len -= len;
    } else {
      current_start = Some(index);
      break;
    }
  }
  // println!("[Write] index: {current_start}, step_len: {step_len}");
  // just exhausted the slice, else the slice should be splitted.
  if let Some(current_start) = current_start {
    if step_len == 0 {
      &mut io_slice[current_start..]
    } else {
      let slice = unsafe {
        let ptr = io_slice[current_start].as_ptr();
        let len = io_slice[current_start].len();
        let slice =
          std::slice::from_raw_parts(ptr.add(step_len), len - step_len);
        slice
      };
      let old =
        std::mem::replace(&mut io_slice[current_start], IoSlice::new(slice));
      println!("[Write] old: {old:?}, current: {current_start}");
      &mut io_slice[current_start..]
    }
  } else {
    &mut []
  }
}

pub fn advance_mut<'a>(
  step: usize,
  io_mut_slice: &'a mut [IoSliceMut<'a>],
) -> &'a mut [IoSliceMut<'a>] {
  let mut current_start = None;
  let mut step_len = step;
  for (index, slice) in io_mut_slice.iter().enumerate() {
    let len = slice.len();
    if len <= step_len {
      step_len -= len;
    } else {
      current_start = Some(index);
      break;
    }
  }
  println!("[Read] index: {current_start:?}, step_len: {step_len}");
  // just exhausted the slice, else the slice should be splitted.

  if let Some(current_start) = current_start {
    if step_len == 0 {
      &mut io_mut_slice[current_start..]
    } else {
      let slice = unsafe {
        let ptr = io_mut_slice[current_start].as_ptr();
        let len = io_mut_slice[current_start].len();
        let slice = std::slice::from_raw_parts_mut(
          ptr.add(step_len) as *mut u8,
          len - step_len,
        );
        slice
      };
      let old = std::mem::replace(
        &mut io_mut_slice[current_start],
        IoSliceMut::new(slice),
      );
      println!("[Read] old: {old:?}, current: {current_start}");
      &mut io_mut_slice[current_start..]
    }
  } else {
    &mut []
  }
}
