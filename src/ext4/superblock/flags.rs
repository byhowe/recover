use crate::add_to_list;
use bitflags::bitflags;

bitflags! {
  pub struct Flags: u32
  {
    const SIGNED_DIRECTORY_HASH = 0x1;
    const UNSIGNED_DIRECTORY_HASH = 0x2;
    const TEST_FILESYSTEM = 0x4;
  }
}

impl Flags
{
  pub fn from_raw(raw: u32) -> Self
  {
    unsafe { Self::from_bits_unchecked(raw) }
  }

  pub fn flags_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    add_to_list!(self, output, "signed_directory_hash", SIGNED_DIRECTORY_HASH);
    add_to_list!(self, output, "unsigned_directory_hash", UNSIGNED_DIRECTORY_HASH);
    add_to_list!(self, output, "test_filesystem", TEST_FILESYSTEM);
    if !self.intersects(Self::all()) {
      output.push("(unknown_bits)")
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
