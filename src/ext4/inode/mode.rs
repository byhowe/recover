use bitflags::bitflags;

bitflags! {
  pub struct Mode: u16
  {
    /// Others may execute
    const OTHER_X = 0x1;
    /// Others may write
    const OTHER_W = 0x2;
    /// Others may read
    const OTHER_R = 0x4;
    /// Group members may execute
    const GROUP_X = 0x8;
    /// Group members may write
    const GROUP_W = 0x10;
    /// Group members may read
    const GROUP_R = 0x20;
    /// Owner may execute
    const OWNER_X = 0x40;
    /// Owner may write
    const OWNER_W = 0x80;
    /// Owner may read
    const OWNER_R = 0x100;
    /// Sticky bit
    const STICKY_B = 0x200;
    /// Set GID
    const SGID = 0x400;
    /// Set UID
    const SUID = 0x800;

    // These are mutually-exclusive file types
    /// FIFO
    const FIFO = 0x1000;
    /// Character device
    const CHAR_DEV = 0x2000;
    /// Directory
    const DIR = 0x4000;
    /// Block device
    const BLOCK_DEV = 0x6000;
    /// Regular file
    const REGULAR = 0x8000;
    /// Symbolic link
    const SYMLINK = 0xA000;
    /// Socket
    const SOCKET = 0xC000;
  }
}

impl Mode
{
  pub fn from_raw(raw: u16) -> Self
  {
    unsafe { Self::from_bits_unchecked(raw) }
  }

  pub fn file_type_flags(&self) -> Self
  {
    unsafe { Mode::from_bits_unchecked(self.bits & 0xF000) }
  }
}

impl std::fmt::Display for Mode
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let dev_char: char = match self.file_type_flags() {
      Self::FIFO => '|',
      Self::CHAR_DEV => 'c',
      Self::DIR => 'd',
      Self::BLOCK_DEV => 'b',
      Self::REGULAR => '.',
      Self::SYMLINK => 'l',
      Self::SOCKET => 's',
      _ => '?',
    };
    write!(
      f,
      "{file_type}{ur}{uw}{ux}{gr}{gw}{gx}{or}{ow}{ox}",
      file_type = dev_char,
      ur = self.bit('r', Self::OWNER_R),
      uw = self.bit('w', Self::OWNER_W),
      ux = self.user_execute_bit(),
      gr = self.bit('r', Self::GROUP_R),
      gw = self.bit('w', Self::GROUP_W),
      gx = self.group_execute_bit(),
      or = self.bit('r', Self::OTHER_R),
      ow = self.bit('w', Self::OTHER_W),
      ox = self.other_execute_bit()
    )
  }
}

impl Mode
{
  #[inline(always)]
  fn bit(&self, chr: char, flag: Self) -> char
  {
    if self.contains(flag) {
      chr
    } else {
      '-'
    }
  }

  #[inline(always)]
  fn user_execute_bit(&self) -> char
  {
    match (
      self.contains(Self::OWNER_X),
      self.contains(Self::SUID),
      self.contains(Self::REGULAR),
    ) {
      (false, false, _) => '-',
      (true, false, false) => 'x',
      (true, false, true) => 'x',
      (false, true, _) => 'S',
      (true, true, false) => 's',
      (true, true, true) => 's',
    }
  }

  #[inline(always)]
  fn group_execute_bit(&self) -> char
  {
    match (self.contains(Self::GROUP_X), self.contains(Self::SGID)) {
      (false, false) => '-',
      (true, false) => 'x',
      (false, true) => 'S',
      (true, true) => 's',
    }
  }

  #[inline(always)]
  fn other_execute_bit(&self) -> char
  {
    match (self.contains(Self::OTHER_X), self.contains(Self::STICKY_B)) {
      (false, false) => '-',
      (true, false) => 'x',
      (false, true) => 'T',
      (true, true) => 't',
    }
  }
}
