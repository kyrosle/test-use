#[cfg(test)]
mod tests {
  use crate::{iovec::*, utils::create_metadata, vector_io::pwritev};
  use tempfile::tempfile;

  #[test]
  fn one_write_one_read() -> std::io::Result<()> {
    let metadata = create_metadata(5);
    let check_metadata = metadata.clone();

    let file = tempfile().unwrap();

    let offset = 0;

    {
      let mut metadata = metadata.clone();
      let metadata = metadata.as_mut_slice();
      let vecs = IoVecs::new_immutable(metadata);
    }

    Ok(())
  }
}
