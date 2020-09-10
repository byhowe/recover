#[derive(Debug, Eq, PartialEq)]
pub enum HashVersion
{
  Legacy,
  HalfMD4,
  Tea,
  LegacyUnsigned,
  HalfMD4Unsigned,
  TeaUnsigned,
  Unknown(u8),
}

impl HashVersion
{
  const LEGACY: u8 = 0;
  const HALF_MD4: u8 = 1;
  const TEA: u8 = 2;
  const LEGACY_UNSIGNED: u8 = 3;
  const HALF_MD4_UNSIGNED: u8 = 4;
  const TEA_UNSIGNED: u8 = 5;

  pub fn from_raw(version: u8) -> Self
  {
    match version {
      Self::LEGACY => Self::Legacy,
      Self::HALF_MD4 => Self::HalfMD4,
      Self::TEA => Self::Tea,
      Self::LEGACY_UNSIGNED => Self::LegacyUnsigned,
      Self::HALF_MD4_UNSIGNED => Self::HalfMD4Unsigned,
      Self::TEA_UNSIGNED => Self::TeaUnsigned,
      _ => Self::Unknown(version),
    }
  }
}

impl std::fmt::Display for HashVersion
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Legacy => "Legacy",
        Self::HalfMD4 => "Half MD4",
        Self::Tea => "Tea",
        Self::LegacyUnsigned => "Legacy, unsigned",
        Self::HalfMD4Unsigned => "Half MD4, unsigned",
        Self::TeaUnsigned => "Tea, unsigned",
        Self::Unknown(ver) => return write!(f, "Unknown algorithm {}", ver),
      }
    )
  }
}
