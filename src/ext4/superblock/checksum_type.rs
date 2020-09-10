#[derive(Debug)]
pub enum ChecksumType
{
  Crc32c,
  Unknown(u8),
}

impl ChecksumType
{
  const CRC32C: u8 = 1;

  pub fn from_raw(csum: u8) -> Self
  {
    match csum {
      Self::CRC32C => Self::Crc32c,
      _ => Self::Unknown(csum),
    }
  }
}

impl std::fmt::Display for ChecksumType
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Crc32c => "crc32c",
        Self::Unknown(csum) => return write!(f, "Unknown type {}", csum),
      }
    )
  }
}
