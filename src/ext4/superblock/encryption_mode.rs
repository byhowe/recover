#[derive(Debug)]
pub enum EncryptionMode
{
  Invalid,
  AES256XTS,
  AES256GCM,
  AES256CBC,
}

impl EncryptionMode
{
  const INVALID: u8 = 0;
  const AES_256_XTS: u8 = 1;
  const AES_256_GCM: u8 = 2;
  const AES_256_CBC: u8 = 3;

  pub fn from_raw(mode: u8) -> Self
  {
    match mode {
      Self::INVALID => Self::Invalid,
      Self::AES_256_XTS => Self::AES256XTS,
      Self::AES_256_GCM => Self::AES256GCM,
      Self::AES_256_CBC => Self::AES256CBC,
      _ => Self::Invalid,
    }
  }

  pub fn from_modes(modes: &[u8]) -> Vec<EncryptionMode>
  {
    modes
      .iter()
      .map(|&m| EncryptionMode::from_raw(m))
      .collect::<Vec<EncryptionMode>>()
  }
}

impl std::fmt::Display for EncryptionMode
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Invalid => "Invalid",
        Self::AES256XTS => "Aes 256 XTS",
        Self::AES256GCM => "Aes 256 GCM",
        Self::AES256CBC => "Aes 256 CBC",
      }
    )
  }
}
