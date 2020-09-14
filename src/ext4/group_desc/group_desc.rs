use super::GroupDescRaw;
use std::convert::{TryFrom, TryInto};
use std::io;

#[derive(Debug)]
pub struct GroupDesc {}

impl GroupDesc
{
  pub const RAW_WIDTH: usize = GroupDescRaw::WIDTH;

  pub fn new<R>(inner: &mut R) -> Result<Self, Error>
  where
    R: io::Read,
  {
    let mut block: [u8; Self::RAW_WIDTH] = [0; Self::RAW_WIDTH];
    inner.read_exact(&mut block)?;
    Ok(GroupDescRaw::from(&block).try_into()?)
  }
}

impl TryFrom<GroupDescRaw> for GroupDesc
{
  type Error = Error;

  fn try_from(_raw: GroupDescRaw) -> Result<Self, Self::Error>
  {
    Ok(Self {})
  }
}

pub enum Error
{
  IO(io::Error),
}

impl From<io::Error> for Error
{
  fn from(error: io::Error) -> Self
  {
    Self::IO(error)
  }
}
