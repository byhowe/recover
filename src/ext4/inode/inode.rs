use super::{Flags, InodeRaw, InodeRawLarge, Mode, Osd1, Osd2};
use crate::{concat_lo_hi, ext4::superblock::Creator};
use chrono::{DateTime, TimeZone, Utc};
use std::io;

#[derive(Debug)]
pub struct Inode
{
  /// File mode.
  pub mode: Mode,
  /// Owner UID.
  pub uid: u32,
  /// Size.
  pub size: u64,
  /// Last access time.
  pub atime: DateTime<Utc>,
  /// Last inode change time.
  pub ctime: DateTime<Utc>,
  /// Last data modification time.
  pub mtime: DateTime<Utc>,
  /// Deletion time.
  pub dtime: DateTime<Utc>,
  /// GID.
  pub gid: u32,
  /// Hard link count. Normally, ext4 does not permit an inode to have more than
  /// 65,000 hard links. This applies to files as well as directories, which
  /// means that there cannot be more than 64,998 subdirectories in a directory
  /// (each subdirectory’s ‘..’ entry counts as a hard link, as does the ‘.’
  /// entry in the directory itself). With the DIR_NLINK feature enabled, ext4
  /// supports more than 64,998 subdirectories by setting this field to 1 to
  /// indicate that the number of hard links is not known.
  pub links_count: u16,
  /// Lower 32-bits of “block” count. If the huge_file feature flag is not set
  /// on the filesystem, the file consumes i_blocks_lo 512-byte blocks on disk.
  /// If huge_file is set and EXT4_HUGE_FILE_FL is NOT set in inode.i_flags,
  /// then the file consumes i_blocks_lo + (i_blocks_hi << 32) 512-byte blocks
  /// on disk. If huge_file is set and EXT4_HUGE_FILE_FL IS set in
  /// inode.i_flags, then this file consumes (i_blocks_lo + i_blocks_hi << 32)
  /// filesystem blocks on disk.
  blocks_lo: u32,
  /// Inode flags. See the table i_flags below.
  pub flags: Flags,
  /// See the table i_osd1 for more details.
  pub osd1: Osd1,
  /// Block map or extent tree. See the section “The Contents of inode.i_block”.
  pub block: [u32; Self::N_BLOCKS],
  /// File version (for NFS).
  pub generation: u32,
  /// Lower 32-bits of extended attribute block. ACLs are of course one of many
  /// possible extended attributes; I think the name of this field is a result
  /// of the first use of extended attributes being for ACLs.
  pub file_acl: u32,
  /// (Obsolete) fragment address.
  pub obso_faddr: u32,
  /// See the table i_osd2 for more details.
  pub osd2: Osd2,
  /// Size of this inode - 128. Alternately, the size of the extended inode
  /// fields beyond the original ext2 inode, including this field.
  pub extra_isize: u16,
  /// Inode checksum.
  pub checksum: u32,
  /// Extra change time bits. This provides sub-second precision. See Inode
  /// Timestamps section.
  pub ctime_extra: u32,
  /// Extra modification time bits. This provides sub-second precision.
  pub mtime_extra: u32,
  /// Extra access time bits. This provides sub-second precision.
  pub atime_extra: u32,
  /// File creation time, in seconds since the epoch.
  pub crtime: DateTime<Utc>,
  /// Extra file creation time bits. This provides sub-second precision.
  pub crtime_extra: u32,
  /// Project ID.
  pub projid: u32,
}

impl Inode
{
  pub const RAW_WIDTH: usize = InodeRaw::WIDTH;
  pub const RAW_WIDTH_LARGE: usize = InodeRawLarge::WIDTH;

  // Special inode numbers
  /// Bad blocks inode.
  pub const BAD_INO: usize = 1;
  /// Root inode.
  pub const ROOT_INO: usize = 2;
  /// User quota inode.
  pub const USR_QUOTA_INO: usize = 3;
  /// Group quota inode.
  pub const GRP_QUOTA_INO: usize = 4;
  /// Boot loader inode.
  pub const BOOT_LOADER_INO: usize = 5;
  /// Undelete directory inode.
  pub const UNDEL_DIR_INO: usize = 6;
  /// Reserved group descriptors inode.
  pub const RESIZE_INO: usize = 7;
  /// Journal inode.
  pub const JOURNAL_INO: usize = 8;
  /// The "exclude" inode, for snapshots.
  pub const EXCLUDE_INO: usize = 9;
  /// Used by non-upstream feature.
  pub const REPLICA_INO: usize = 10;
  /// First non-reserved inode for old ext2 filesystems.
  pub const GOOD_OLD_FIRST_INO: u32 = 11;

  /// Maximal count of links to a file.
  pub const LINK_MAX: u16 = 65000;

  pub const GOOD_OLD_INODE_SIZE: u16 = 128;

  // Constants relative to the data blocks.
  pub const NDIR_BLOCKS: usize = 12;
  pub const IND_BLOCK: usize = Self::NDIR_BLOCKS;
  pub const DIND_BLOCK: usize = Self::IND_BLOCK + 1;
  pub const TIND_BLOCK: usize = Self::DIND_BLOCK + 1;
  pub const N_BLOCKS: usize = Self::TIND_BLOCK + 1;

  pub fn new<R>(inner: &mut R, large: bool, os: &Creator) -> Result<Self, Error>
  where
    R: io::Read,
  {
    if large {
      let mut block: [u8; Self::RAW_WIDTH_LARGE] = [0; Self::RAW_WIDTH_LARGE];
      inner.read_exact(&mut block)?;
      Ok(Self::from_raw_large(InodeRawLarge::from(&block), os))
    } else {
      let mut block: [u8; Self::RAW_WIDTH] = [0; Self::RAW_WIDTH];
      inner.read_exact(&mut block)?;
      Ok(Self::from_raw(InodeRaw::from(&block), os))
    }
  }

  fn from_raw(raw: InodeRaw, os: &Creator) -> Self
  {
    let osd2 = Osd2::from_raw(raw.i_osd2, &os);
    Self {
      mode: Mode::from_raw(raw.i_mode),
      uid: concat_lo_hi!(
        u32,
        raw.i_uid,
        match osd2 {
          Osd2::Linux { uid_high, .. } => uid_high,
          Osd2::Hurd { uid_high, .. } => uid_high,
          _ => 0,
        }
      ),
      size: concat_lo_hi!(u64, raw.i_size_lo, raw.i_size_high),
      atime: Utc.timestamp(raw.i_atime as i64, 0),
      ctime: Utc.timestamp(raw.i_ctime as i64, 0),
      mtime: Utc.timestamp(raw.i_mtime as i64, 0),
      dtime: Utc.timestamp(raw.i_dtime as i64, 0),
      gid: concat_lo_hi!(
        u32,
        raw.i_gid,
        match osd2 {
          Osd2::Linux { gid_high, .. } => gid_high,
          Osd2::Hurd { gid_high, .. } => gid_high,
          _ => 0,
        }
      ),
      links_count: raw.i_links_count,
      blocks_lo: raw.i_blocks_lo,
      flags: Flags::from_raw(raw.i_flags),
      osd1: Osd1::from_raw(raw.i_osd1, &os),
      block: raw.i_block,
      generation: raw.i_generation,
      file_acl: concat_lo_hi!(
        u32,
        raw.i_file_acl_lo,
        match osd2 {
          Osd2::Linux { file_acl_high, .. } => file_acl_high,
          Osd2::Masix { file_acl_high, .. } => file_acl_high,
          _ => 0,
        }
      ),
      obso_faddr: raw.i_obso_faddr,
      osd2,
      extra_isize: 0,
      checksum: 0,
      ctime_extra: 0,
      mtime_extra: 0,
      atime_extra: 0,
      crtime: Utc.timestamp(0, 0),
      crtime_extra: 0,
      projid: 0,
    }
  }

  fn from_raw_large(raw: InodeRawLarge, os: &Creator) -> Self
  {
    let osd2 = Osd2::from_raw(raw.i_osd2, &os);
    Self {
      mode: Mode::from_raw(raw.i_mode),
      uid: concat_lo_hi!(
        u32,
        raw.i_uid,
        match osd2 {
          Osd2::Linux { uid_high, .. } => uid_high,
          Osd2::Hurd { uid_high, .. } => uid_high,
          _ => 0,
        }
      ),
      size: concat_lo_hi!(u64, raw.i_size_lo, raw.i_size_high),
      atime: Utc.timestamp(raw.i_atime as i64, 0),
      ctime: Utc.timestamp(raw.i_ctime as i64, 0),
      mtime: Utc.timestamp(raw.i_mtime as i64, 0),
      dtime: Utc.timestamp(raw.i_dtime as i64, 0),
      gid: concat_lo_hi!(
        u32,
        raw.i_gid,
        match osd2 {
          Osd2::Linux { gid_high, .. } => gid_high,
          Osd2::Hurd { gid_high, .. } => gid_high,
          _ => 0,
        }
      ),
      links_count: raw.i_links_count,
      blocks_lo: raw.i_blocks_lo,
      flags: Flags::from_raw(raw.i_flags),
      osd1: Osd1::from_raw(raw.i_osd1, &os),
      block: raw.i_block,
      generation: raw.i_generation,
      file_acl: concat_lo_hi!(
        u32,
        raw.i_file_acl_lo,
        match osd2 {
          Osd2::Linux { file_acl_high, .. } => file_acl_high,
          Osd2::Masix { file_acl_high, .. } => file_acl_high,
          _ => 0,
        }
      ),
      obso_faddr: raw.i_obso_faddr,
      osd2,
      extra_isize: raw.i_extra_isize,
      checksum: match osd2 {
        Osd2::Linux { checksum_lo, .. } => concat_lo_hi!(u32, checksum_lo, raw.i_checksum_hi),
        _ => 0,
      },
      ctime_extra: raw.i_ctime_extra,
      mtime_extra: raw.i_mtime_extra,
      atime_extra: raw.i_atime_extra,
      crtime: Utc.timestamp(raw.i_crtime as i64, 0),
      crtime_extra: raw.i_crtime_extra,
      projid: raw.i_projid,
    }
  }
}

#[derive(Debug)]
pub enum Error
{
  IO(io::Error),
}

impl From<io::Error> for Error
{
  fn from(error: io::Error) -> Self
  {
    Self::IO(error)
  }
}

impl std::fmt::Display for Error
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "Inode error: {}",
      match self {
        Self::IO(error) => format!("An IO error occurred: {}", error),
      }
    )
  }
}
