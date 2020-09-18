#[derive(Debug, Eq, PartialEq)]
pub enum RevisionLevel
{
  Original,
  Dynamic,
  Unknown(u32),
}

impl RevisionLevel
{
  pub const GOOD_OLD_REV: Self = Self::Original;

  const ORIGINAL_FORMAT: u32 = 0;
  const V2_FORMAT: u32 = 1;

  pub fn from_raw(rev: u32) -> Self
  {
    match rev {
      Self::ORIGINAL_FORMAT => Self::Original,
      Self::V2_FORMAT => Self::Dynamic,
      _ => Self::Unknown(rev),
    }
  }
}

impl std::fmt::Display for RevisionLevel
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Original => "0 (Original format)",
        Self::Dynamic => "1 (v2 format w/ dynamic inode sizes)",
        Self::Unknown(rev) => return write!(f, "{} (Unknown)", rev),
      }
    )
  }
}
