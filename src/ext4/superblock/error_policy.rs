#[derive(Debug)]
pub enum ErrorPolicy
{
  Continue,
  RemountReadOnly,
  Panic,
  Unknown(u16),
}

impl ErrorPolicy
{
  const CONTINUE: u16 = 1;
  const REMOUNT_READ_ONLY: u16 = 2;
  const PANIC: u16 = 3;

  pub fn from_raw(error: u16) -> Self
  {
    match error {
      Self::CONTINUE => Self::Continue,
      Self::REMOUNT_READ_ONLY => Self::RemountReadOnly,
      Self::PANIC => Self::Panic,
      _ => Self::Unknown(error),
    }
  }
}

impl std::fmt::Display for ErrorPolicy
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Continue => "Continue",
        Self::RemountReadOnly => "Remount read-only",
        Self::Panic => "Panic",
        Self::Unknown(err) => return write!(f, "Unknown {} (Continue)", err),
      }
    )
  }
}
