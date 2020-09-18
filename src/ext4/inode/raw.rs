use super::{Inode, Osd2Raw};

#[repr(C)]
#[derive(Debug)]
pub(crate) struct InodeRaw
{
  /// File mode. See the table i_mode below.
  pub(crate) i_mode: u16, // 0 - 2
  /// Lower 16-bits of Owner UID.
  pub(crate) i_uid: u16, // 2 - 4
  /// Lower 32-bits of size in bytes.
  pub(crate) i_size_lo: u32, // 4 - 8
  /// Last access time, in seconds since the epoch. However, if the EA_INODE
  /// inode flag is set, this inode stores an extended attribute value and this
  /// field contains the checksum of the value.
  pub(crate) i_atime: u32, // 8 - 12
  /// Last inode change time, in seconds since the epoch. However, if the
  /// EA_INODE inode flag is set, this inode stores an extended attribute value
  /// and this field contains the lower 32 bits of the attribute value’s
  /// reference count.
  pub(crate) i_ctime: u32, // 12 - 16
  /// Last data modification time, in seconds since the epoch. However, if the
  /// EA_INODE inode flag is set, this inode stores an extended attribute value
  /// and this field contains the number of the inode that owns the extended
  /// attribute.
  pub(crate) i_mtime: u32, // 16 - 20
  /// Deletion Time, in seconds since the epoch.
  pub(crate) i_dtime: u32, // 20 - 24
  /// Lower 16-bits of GID.
  pub(crate) i_gid: u16, // 24 - 26
  /// Hard link count. Normally, ext4 does not permit an inode to have more than
  /// 65,000 hard links. This applies to files as well as directories, which
  /// means that there cannot be more than 64,998 subdirectories in a directory
  /// (each subdirectory’s ‘..’ entry counts as a hard link, as does the ‘.’
  /// entry in the directory itself). With the DIR_NLINK feature enabled, ext4
  /// supports more than 64,998 subdirectories by setting this field to 1 to
  /// indicate that the number of hard links is not known.
  pub(crate) i_links_count: u16, // 26 - 28
  /// Lower 32-bits of “block” count. If the huge_file feature flag is not set
  /// on the filesystem, the file consumes i_blocks_lo 512-byte blocks on disk.
  /// If huge_file is set and EXT4_HUGE_FILE_FL is NOT set in inode.i_flags,
  /// then the file consumes i_blocks_lo + (i_blocks_hi << 32) 512-byte blocks
  /// on disk. If huge_file is set and EXT4_HUGE_FILE_FL IS set in
  /// inode.i_flags, then this file consumes (i_blocks_lo + i_blocks_hi << 32)
  /// filesystem blocks on disk.
  pub(crate) i_blocks_lo: u32, // 28 - 32
  /// Inode flags. See the table i_flags below.
  pub(crate) i_flags: u32, // 32 - 36
  /// See the table i_osd1 for more details.
  pub(crate) i_osd1: u32, // 36 - 40
  /// Block map or extent tree. See the section “The Contents of inode.i_block”.
  pub(crate) i_block: [u32; Inode::N_BLOCKS], // 40 - 100
  /// File version (for NFS).
  pub(crate) i_generation: u32, // 100 - 104
  /// Lower 32-bits of extended attribute block. ACLs are of course one of many
  /// possible extended attributes; I think the name of this field is a result
  /// of the first use of extended attributes being for ACLs.
  pub(crate) i_file_acl_lo: u32, // 104 - 108
  /// Upper 32-bits of file/directory size. In ext2/3 this field was named
  /// i_dir_acl, though it was usually set to zero and never used.
  pub(crate) i_size_high: u32, // 108 - 112
  /// (Obsolete) fragment address.
  pub(crate) i_obso_faddr: u32, // 112 - 116
  /// See the table i_osd2 for more details.
  pub(crate) i_osd2: Osd2Raw, // 116 - 128
}

impl InodeRaw
{
  pub(crate) const WIDTH: usize = 128;
}

impl From<&[u8; Self::WIDTH]> for InodeRaw
{
  #[cfg(target_endian = "little")]
  fn from(block: &[u8; Self::WIDTH]) -> Self
  {
    unsafe { std::mem::transmute::<[u8; Self::WIDTH], Self>(*block) }
  }

  #[cfg(target_endian = "big")]
  fn from(block: &[u8; Self::WIDTH]) -> Self
  {
    let mut raw = unsafe { std::mem::transmute::<[u8; Self::WIDTH], Self>(*block) };
    raw.i_mode = u16::from_le(raw.i_mode);
    raw.i_uid = u16::from_le(raw.i_uid);
    raw.i_size_lo = u32::from_le(raw.i_size_lo);
    raw.i_atime = u32::from_le(raw.i_atime);
    raw.i_ctime = u32::from_le(raw.i_ctime);
    raw.i_mtime = u32::from_le(raw.i_mtime);
    raw.i_dtime = u32::from_le(raw.i_dtime);
    raw.i_gid = u16::from_le(raw.i_gid);
    raw.i_links_count = u16::from_le(raw.i_links_count);
    raw.i_blocks_lo = u32::from_le(raw.i_blocks_lo);
    raw.i_flags = u32::from_le(raw.i_flags);
    raw.i_generation = u32::from_le(raw.i_generation);
    raw.i_file_acl_lo = u32::from_le(raw.i_file_acl_lo);
    raw.i_size_high = u32::from_le(raw.i_size_high);
    raw.i_obso_faddr = u32::from_le(raw.i_obso_faddr);
    raw
  }
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct InodeRawLarge
{
  /// File mode. See the table i_mode below.
  pub(crate) i_mode: u16, // 0 - 2
  /// Lower 16-bits of Owner UID.
  pub(crate) i_uid: u16, // 2 - 4
  /// Lower 32-bits of size in bytes.
  pub(crate) i_size_lo: u32, // 4 - 8
  /// Last access time, in seconds since the epoch. However, if the EA_INODE
  /// inode flag is set, this inode stores an extended attribute value and this
  /// field contains the checksum of the value.
  pub(crate) i_atime: u32, // 8 - 12
  /// Last inode change time, in seconds since the epoch. However, if the
  /// EA_INODE inode flag is set, this inode stores an extended attribute value
  /// and this field contains the lower 32 bits of the attribute value’s
  /// reference count.
  pub(crate) i_ctime: u32, // 12 - 16
  /// Last data modification time, in seconds since the epoch. However, if the
  /// EA_INODE inode flag is set, this inode stores an extended attribute value
  /// and this field contains the number of the inode that owns the extended
  /// attribute.
  pub(crate) i_mtime: u32, // 16 - 20
  /// Deletion Time, in seconds since the epoch.
  pub(crate) i_dtime: u32, // 20 - 24
  /// Lower 16-bits of GID.
  pub(crate) i_gid: u16, // 24 - 26
  /// Hard link count. Normally, ext4 does not permit an inode to have more than
  /// 65,000 hard links. This applies to files as well as directories, which
  /// means that there cannot be more than 64,998 subdirectories in a directory
  /// (each subdirectory’s ‘..’ entry counts as a hard link, as does the ‘.’
  /// entry in the directory itself). With the DIR_NLINK feature enabled, ext4
  /// supports more than 64,998 subdirectories by setting this field to 1 to
  /// indicate that the number of hard links is not known.
  pub(crate) i_links_count: u16, // 26 - 28
  /// Lower 32-bits of “block” count. If the huge_file feature flag is not set
  /// on the filesystem, the file consumes i_blocks_lo 512-byte blocks on disk.
  /// If huge_file is set and EXT4_HUGE_FILE_FL is NOT set in inode.i_flags,
  /// then the file consumes i_blocks_lo + (i_blocks_hi << 32) 512-byte blocks
  /// on disk. If huge_file is set and EXT4_HUGE_FILE_FL IS set in
  /// inode.i_flags, then this file consumes (i_blocks_lo + i_blocks_hi << 32)
  /// filesystem blocks on disk.
  pub(crate) i_blocks_lo: u32, // 28 - 32
  /// Inode flags. See the table i_flags below.
  pub(crate) i_flags: u32, // 32 - 36
  /// See the table i_osd1 for more details.
  pub(crate) i_osd1: u32, // 36 - 40
  /// Block map or extent tree. See the section “The Contents of inode.i_block”.
  pub(crate) i_block: [u32; Inode::N_BLOCKS], // 40 - 100
  /// File version (for NFS).
  pub(crate) i_generation: u32, // 100 - 104
  /// Lower 32-bits of extended attribute block. ACLs are of course one of many
  /// possible extended attributes; I think the name of this field is a result
  /// of the first use of extended attributes being for ACLs.
  pub(crate) i_file_acl_lo: u32, // 104 - 108
  /// Upper 32-bits of file/directory size. In ext2/3 this field was named
  /// i_dir_acl, though it was usually set to zero and never used.
  pub(crate) i_size_high: u32, // 108 - 112
  /// (Obsolete) fragment address.
  pub(crate) i_obso_faddr: u32, // 112 - 116
  /// See the table i_osd2 for more details.
  pub(crate) i_osd2: Osd2Raw, // 116 - 128
  /// Size of this inode - 128. Alternately, the size of the extended inode
  /// fields beyond the original ext2 inode, including this field.
  pub(crate) i_extra_isize: u16, // 128 - 130
  /// Upper 16-bits of the inode checksum.
  pub(crate) i_checksum_hi: u16, // 130 - 132
  /// Extra change time bits. This provides sub-second precision. See Inode
  /// Timestamps section.
  pub(crate) i_ctime_extra: u32, // 132 - 136
  /// Extra modification time bits. This provides sub-second precision.
  pub(crate) i_mtime_extra: u32, // 136 - 140
  /// Extra access time bits. This provides sub-second precision.
  pub(crate) i_atime_extra: u32, // 140 - 144
  /// File creation time, in seconds since the epoch.
  pub(crate) i_crtime: u32, // 144 - 148
  /// Extra file creation time bits. This provides sub-second precision.
  pub(crate) i_crtime_extra: u32, // 148 - 152
  /// Upper 32-bits for version number.
  pub(crate) i_version_hi: u32, // 152 - 156
  /// Project ID.
  pub(crate) i_projid: u32, // 156 - 160
}

impl InodeRawLarge
{
  pub(crate) const WIDTH: usize = 160;
}

impl From<&[u8; Self::WIDTH]> for InodeRawLarge
{
  #[cfg(target_endian = "little")]
  fn from(block: &[u8; Self::WIDTH]) -> Self
  {
    unsafe { std::mem::transmute::<[u8; Self::WIDTH], Self>(*block) }
  }

  #[cfg(target_endian = "big")]
  fn from(block: &[u8; Self::WIDTH]) -> Self
  {
    let mut raw = unsafe { std::mem::transmute::<[u8; Self::WIDTH], Self>(*block) };
    raw.i_mode = u16::from_le(raw.i_mode);
    raw.i_uid = u16::from_le(raw.i_uid);
    raw.i_size_lo = u32::from_le(raw.i_size_lo);
    raw.i_atime = u32::from_le(raw.i_atime);
    raw.i_ctime = u32::from_le(raw.i_ctime);
    raw.i_mtime = u32::from_le(raw.i_mtime);
    raw.i_dtime = u32::from_le(raw.i_dtime);
    raw.i_gid = u16::from_le(raw.i_gid);
    raw.i_links_count = u16::from_le(raw.i_links_count);
    raw.i_blocks_lo = u32::from_le(raw.i_blocks_lo);
    raw.i_flags = u32::from_le(raw.i_flags);
    raw.i_generation = u32::from_le(raw.i_generation);
    raw.i_file_acl_lo = u32::from_le(raw.i_file_acl_lo);
    raw.i_size_high = u32::from_le(raw.i_size_high);
    raw.i_obso_faddr = u32::from_le(raw.i_obso_faddr);
    raw.i_extra_isize = u16::from_le(raw.i_extra_isize);
    raw.i_checksum_hi = u16::from_le(raw.i_checksum_hi);
    raw.i_ctime_extra = u32::from_le(raw.i_ctime_extra);
    raw.i_mtime_extra = u32::from_le(raw.i_mtime_extra);
    raw.i_atime_extra = u32::from_le(raw.i_atime_extra);
    raw.i_crtime = u32::from_le(raw.i_crtime);
    raw.i_crtime_extra = u32::from_le(raw.i_crtime_extra);
    raw.i_version_hi = u32::from_le(raw.i_version_hi);
    raw.i_projid = u32::from_le(raw.i_projid);
    raw
  }
}
