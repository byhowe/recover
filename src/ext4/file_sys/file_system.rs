use super::iters;
use crate::ext4::{superblock, Superblock};
use std::io::{self, SeekFrom};

pub struct FileSystem<R>
{
  pub(crate) inner: R,
  offset: u64,
  pub sb: Superblock,
}

impl<R> FileSystem<R>
{
  pub const START_OFFSET: u64 = 1024;

  pub fn new(mut inner: R, offset: u64) -> Result<Self, Error>
  where
    R: io::Read + io::Seek,
  {
    inner.seek(SeekFrom::Start(Self::START_OFFSET + offset))?;
    let sb = Superblock::new(&mut inner)?;
    Ok(Self { inner, offset, sb })
  }

  pub fn iter_group_descriptors<'fs>(&'fs mut self) -> iters::GroupDescIter<'fs, R>
  {
    iters::GroupDescIter::new(self)
  }
}

impl<R> io::Seek for FileSystem<R>
where
  R: io::Seek,
{
  fn seek(&mut self, mut pos: SeekFrom) -> io::Result<u64>
  {
    if let SeekFrom::Start(i) = pos {
      pos = SeekFrom::Start(i + self.offset)
    }
    self.inner.seek(pos)
  }
}

#[derive(Debug)]
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
      "Filesystem error: {}",
      match self {
        Self::IO(error) => format!("An IO error occurred: {}", error),
        Self::Superblock(err) => err.to_string(),
      }
    )
  }
}
