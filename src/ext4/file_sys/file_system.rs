use crate::ext4::{superblock, Superblock};
use std::io::{self, SeekFrom};

pub struct FileSystem<R>
where
  R: io::Read + io::Seek,
{
  inner: R,
  pub sb: Superblock,
}

impl<R> FileSystem<R>
where
  R: io::Read + io::Seek,
{
  pub fn new(mut inner: R) -> Result<Self, Error>
  {
    inner.seek(SeekFrom::Current(1024))?;
    let sb = Superblock::new(&mut inner)?;
    Ok(Self { inner, sb })
  }
}

pub enum Error
{
  IO(io::Error),
  Superblock(superblock::Error),
}

impl From<io::Error> for Error
{
  fn from(error: io::Error) -> Self
  {
    Self::IO(error)
  }
}

impl From<superblock::Error> for Error
{
  fn from(error: superblock::Error) -> Self
  {
    Self::Superblock(error)
  }
}

impl std::fmt::Display for Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::IO(error) => format!(
          "An IO error occurred while reading the file system: {}",
          error
        ),
        Self::Superblock(err) => err.to_string(),
      }
    )
  }
}
