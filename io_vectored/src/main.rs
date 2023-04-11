use std::fs::File;

pub mod iovec;
pub mod test;
pub mod utils;
pub mod vector_io;

use utils::*;
use vector_io::*;

const DATA_LEN: usize = 8;
const DATA_GROUP_SIZE: usize = 200;

fn main() -> std::io::Result<()> {
  let mut io_slice = create_write_slices(DATA_GROUP_SIZE);
  let mut io_slice = io_slice.as_mut_slice();

  let buffer = File::create("foo.txt")?;

  println!("[Begin] {io_slice:?}");

  // Writes some prefix of the byte string, not necessarily all of it.
  let mut total_written = 0;
  let offset = 2;
  loop {
    // let result = buffer.write_vectored(io_slice)?;
    let result = pwritev(&buffer, io_slice, offset + total_written)?;
    io_slice = advance(result, io_slice);

    println!("[Write Loop] write in {result}; advance io_slice: {io_slice:?}");
    total_written += result;

    // TODO: optimize the judgement after redesigning the VectorIO structure.
    if io_slice.iter().fold(0, |mut c, s| {
      c += s.len();
      c
    }) == 0
    {
      break;
    }
    // std::thread::sleep(Duration::from_secs(1));
  }

  println!("written {total_written} bytes");

  // ----------------------------------------------------------------------------
  drop(buffer);
  // ----------------------------------------------------------------------------

  let buffer = File::open("foo.txt")?;

  let mut total_read = 0;

  let mut read_slice = create_mut_slices(DATA_GROUP_SIZE);

  {
    let mut read_slice = read_slice
      .iter_mut()
      .map(|s| s.as_mut_slice())
      .collect::<Vec<_>>();
    let mut io_mut_slice = create_io_mut_slice(&mut read_slice);

    let mut io_mut_slice = io_mut_slice.as_mut_slice();

    loop {
      let result = preadv(&buffer, io_mut_slice, offset + total_read)?;
      io_mut_slice = advance_mut(result, io_mut_slice);
      println!("[Write Loop] write in {result};");
      total_read += result;
      if result == 0 {
        break;
      }
      // std::thread::sleep(Duration::from_secs(1));
    }
  }
  println!("read {total_read} bytes");
  for slice in read_slice {
    println!("{slice:?}");
  }
  Ok(())
}
