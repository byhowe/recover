use super::Mode;

#[derive(Debug)]
pub enum FileType
{
  Fifo,
  CharacterDevice,
  Directory,
  BlockDevice,
  RegularFile,
  SymbolicLink,
  Socket,
  Special,
}

impl FileType
{
  pub fn from_mode(mode: Mode) -> Self
  {
    match mode.file_type_flags() {
      Mode::FIFO => Self::Fifo,
      Mode::CHAR_DEV => Self::CharacterDevice,
      Mode::DIR => Self::Directory,
      Mode::BLOCK_DEV => Self::BlockDevice,
      Mode::REGULAR => Self::RegularFile,
      Mode::SYMLINK => Self::SymbolicLink,
      Mode::SOCKET => Self::Socket,
      _ => Self::Special,
    }
  }

  pub fn from_code(code: u8) -> Self
  {
    match code {
      0x1 => Self::RegularFile,
      0x2 => Self::Directory,
      0x3 => Self::CharacterDevice,
      0x4 => Self::BlockDevice,
      0x5 => Self::Fifo,
      0x6 => Self::Socket,
      0x7 => Self::SymbolicLink,
      _ => Self::Special,
    }
  }
}
