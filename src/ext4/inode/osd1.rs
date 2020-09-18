use crate::ext4::superblock::Creator;

#[derive(Debug, Copy, Clone)]
pub enum Osd1
{
  Linux
  {
    /// Inode version. However, if the EA_INODE inode flag is set, this inode
    /// stores an extended attribute value and this field contains the upper 32
    /// bits of the attribute value’s reference count.
    version: u32,
  },
  Hurd
  {
    /// ??
    translator: u32,
  },
  Masix
  {
    /// ??
    reserved: u32,
  },
  Unknown(u32),
}

impl Osd1
{
  pub fn from_raw(raw: u32, os: &Creator) -> Self
  {
    match os {
      Creator::Linux => Self::Linux { version: raw },
      Creator::Hurd => Self::Hurd { translator: raw },
      Creator::Masix => Self::Masix { reserved: raw },
      _ => Self::Unknown(raw),
    }
  }
}
