#[derive(Debug)]
pub struct Flags
{
  inode_uninitialized: bool,
  block_uninitialized: bool,
  inode_zeroed: bool,
  unknown_bits: bool,
}

impl Flags
{
  const INODE_UNINIT: u16 = 0x1;
  const BLOCK_UNINIT: u16 = 0x2;
  const INODE_ZEROED: u16 = 0x4;

  pub fn from_raw(flags: u16) -> Self
  {
    Self {
      inode_uninitialized: flags & Self::INODE_UNINIT != 0,
      block_uninitialized: flags & Self::BLOCK_UNINIT != 0,
      inode_zeroed: flags & Self::INODE_ZEROED != 0,
      unknown_bits: flags & !(Self::INODE_UNINIT | Self::BLOCK_UNINIT | Self::INODE_ZEROED) != 0,
    }
  }

  pub fn flags_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    if self.inode_uninitialized {
      output.push("inode_uninitialized");
    }
    if self.block_uninitialized {
      output.push("block_uninitialized");
    }
    if self.inode_zeroed {
      output.push("inode_zeroed");
    }
    if self.unknown_bits {
      output.push("(unknown_bits)");
    }
    output
  }
}

impl std::fmt::Display for Flags
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.flags_list()))
  }
}
