#[derive(Debug)]
pub enum Creator
{
  Linux,
  Hurd,
  Masix,
  FreeBSD,
  Lites,
  Unknown(u32),
}

impl Creator
{
  const LINUX: u32 = 0;
  const HURD: u32 = 1;
  const MASIX: u32 = 2;
  const FREE_BSD: u32 = 3;
  const LITES: u32 = 4;

  pub fn from_raw(creator: u32) -> Self
  {
    match creator {
      Self::LINUX => Self::Linux,
      Self::HURD => Self::Hurd,
      Self::MASIX => Self::Masix,
      Self::FREE_BSD => Self::FreeBSD,
      Self::LITES => Self::Lites,
      _ => Self::Unknown(creator),
    }
  }
}

impl std::fmt::Display for Creator
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{:?}", self)
  }
}
