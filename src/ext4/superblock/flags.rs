#[derive(Debug)]
pub struct Flags
{
  signed_directory_hash: bool,
  unsigned_directory_hash: bool,
  test_filesystem: bool,
  unknown_bits: bool,
}

impl Flags
{
  const SIGNED_DIRECTORY_HASH: u32 = 0x1;
  const UNSIGNED_DIRECTORY_HASH: u32 = 0x2;
  const TEST_FILESYSTEM: u32 = 0x4;

  pub fn from_raw(flags: u32) -> Self
  {
    Self {
      signed_directory_hash: flags & Self::SIGNED_DIRECTORY_HASH != 0,
      unsigned_directory_hash: flags & Self::UNSIGNED_DIRECTORY_HASH != 0,
      test_filesystem: flags & Self::TEST_FILESYSTEM != 0,
      unknown_bits: flags
        & !(Self::SIGNED_DIRECTORY_HASH | Self::UNSIGNED_DIRECTORY_HASH | Self::TEST_FILESYSTEM)
        != 0,
    }
  }

  pub fn flags_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    if self.signed_directory_hash {
      output.push("signed_directory_hash");
    }
    if self.unsigned_directory_hash {
      output.push("unsigned_directory_hash");
    }
    if self.test_filesystem {
      output.push("test_filesystem");
    }
    if self.unknown_bits {
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
