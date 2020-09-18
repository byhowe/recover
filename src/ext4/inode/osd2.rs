use crate::ext4::superblock::Creator;

#[derive(Debug, Copy, Clone)]
pub enum Osd2
{
  Linux
  {
    /// Upper 16-bits of the block count. Please see the note attached to
    /// i_blocks_lo.
    blocks_high: u16, // 0 - 2
    /// Upper 16-bits of the extended attribute block (historically, the file
    /// ACL location). See the Extended Attributes section below.
    file_acl_high: u16, // 2 - 4
    /// Upper 16-bits of the Owner UID.
    uid_high: u16, // 4 - 6
    /// Upper 16-bits of the GID.
    gid_high: u16, // 6 - 8
    /// Lower 16-bits of the inode checksum.
    checksum_lo: u16, // 8 - 10
  },
  Hurd
  {
    /// Upper 16-bits of the file mode.
    mode_high: u16, // 2 - 4
    /// Upper 16-bits of the Owner UID.
    uid_high: u16, // 4 - 6
    /// Upper 16-bits of the GID.
    gid_high: u16, // 6 - 8
    /// Author code?
    author: u32, // 8 - 12
  },
  Masix
  {
    /// Upper 16-bits of the extended attribute block (historically, the file
    /// ACL location).
    file_acl_high: u16, // 2 - 4
  },
  Unknown([u8; 12]),
}

impl Osd2
{
  #[cfg(target_endian = "little")]
  pub(crate) fn from_raw(raw: Osd2Raw, os: &Creator) -> Self
  {
    match os {
      Creator::Linux => Self::Linux {
        blocks_high: unsafe { raw.linux.l_i_blocks_high },
        file_acl_high: unsafe { raw.linux.l_i_file_acl_high },
        uid_high: unsafe { raw.linux.l_i_uid_high },
        gid_high: unsafe { raw.linux.l_i_gid_high },
        checksum_lo: unsafe { raw.linux.l_i_checksum_lo },
      },
      Creator::Hurd => Self::Hurd {
        mode_high: unsafe { raw.hurd.h_i_mode_high },
        uid_high: unsafe { raw.hurd.h_i_uid_high },
        gid_high: unsafe { raw.hurd.h_i_gid_high },
        author: unsafe { raw.hurd.h_i_author },
      },
      Creator::Masix => Self::Masix {
        file_acl_high: unsafe { raw.masix.m_i_file_acl_high },
      },
      _ => Self::Unknown(unsafe { raw.unknown }),
    }
  }

  #[cfg(target_endian = "big")]
  pub(crate) fn from_raw(raw: Osd2Raw, os: &Creator) -> Self
  {
    match os {
      Creator::Linux => Self::Linux {
        blocks_high: u16::from_le(unsafe { raw.linux.l_i_blocks_high }),
        file_acl_high: u16::from_le(unsafe { raw.linux.l_i_file_acl_high }),
        uid_high: u16::from_le(unsafe { raw.linux.l_i_uid_high }),
        gid_high: u16::from_le(unsafe { raw.linux.l_i_gid_high }),
        checksum_lo: u16::from_le(unsafe { raw.linux.l_i_checksum_lo }),
      },
      Creator::Hurd => Self::Hurd {
        mode_high: u16::from_le(unsafe { raw.hurd.h_i_mode_high }),
        uid_high: u16::from_le(unsafe { raw.hurd.h_i_uid_high }),
        gid_high: u16::from_le(unsafe { raw.hurd.h_i_gid_high }),
        author: u32::from_le(unsafe { raw.hurd.h_i_author }),
      },
      Creator::Masix => Self::Masix {
        file_acl_high: u16::from_le(unsafe { raw.masix.m_i_file_acl_high }),
      },
      _ => Self::Unknown(unsafe { raw.unknown }),
    }
  }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Osd2LinuxRaw
{
  /// Upper 16-bits of the block count. Please see the note attached to
  /// i_blocks_lo.
  l_i_blocks_high: u16, // 0 - 2
  /// Upper 16-bits of the extended attribute block (historically, the file ACL
  /// location). See the Extended Attributes section below.
  l_i_file_acl_high: u16, // 2 - 4
  /// Upper 16-bits of the Owner UID.
  l_i_uid_high: u16, // 4 - 6
  /// Upper 16-bits of the GID.
  l_i_gid_high: u16, // 6 - 8
  /// Lower 16-bits of the inode checksum.
  l_i_checksum_lo: u16, // 8 - 10
  /// Unused.
  l_i_reserved: u16, // 10 - 12
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Osd2HurdRaw
{
  /// ??
  h_i_reserved1: u16, // 0 - 2
  /// Upper 16-bits of the file mode.
  h_i_mode_high: u16, // 2 - 4
  /// Upper 16-bits of the Owner UID.
  h_i_uid_high: u16, // 4 - 6
  /// Upper 16-bits of the GID.
  h_i_gid_high: u16, // 6 - 8
  /// Author code?
  h_i_author: u32, // 8 - 12
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Osd2MasixRaw
{
  /// ??
  h_i_reserved1: u16, // 0 - 2
  /// Upper 16-bits of the extended attribute block (historically, the file ACL
  /// location).
  m_i_file_acl_high: u16, // 2 - 4
  /// ??
  m_i_reserved2: [u32; 2], // 4 - 12
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) union Osd2Raw
{
  linux: Osd2LinuxRaw,
  hurd: Osd2HurdRaw,
  masix: Osd2MasixRaw,
  unknown: [u8; 12],
}

impl std::fmt::Debug for Osd2Raw
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{:?}", unsafe { self.unknown })
  }
}
