use crate::add_to_list;
use bitflags::bitflags;

bitflags! {
  pub struct Flags: u16
  {
    const INODE_UNINIT = 0x1;
    const BLOCK_UNINIT = 0x2;
    const INODE_ZEROED = 0x4;
  }
}

impl Flags
{
  pub fn from_raw(raw: u16) -> Self
  {
    unsafe { Self::from_bits_unchecked(raw) }
  }

  pub fn flags_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    add_to_list!(self, output, "inode_uninitialized", INODE_UNINIT);
    add_to_list!(self, output, "block_uninitialized", BLOCK_UNINIT);
    add_to_list!(self, output, "inode_zeroed", INODE_ZEROED);
    if !self.intersects(Self::all()) {
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
