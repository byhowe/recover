use chrono::{DateTime, TimeZone, Utc};
use std::convert::TryInto;
use std::io;

const EXT4_MAGIC_SIGNATURE: u16 = 0xEF53;

#[derive(Debug)]
#[repr(C)]
pub struct Superblock
{
  /// Total inode count.
  pub inodes_count: u32, // 0x0
  /// Total block count.
  pub blocks_count_lo: u32, // 0x4
  /// This number of blocks can only be allocated by the super-user.
  pub r_blocks_count_lo: u32, // 0x8
  /// Free block count.
  pub free_blocks_count_lo: u32, // 0xC
  /// Free inode count.
  pub free_inodes_count: u32, // 0x10
  /// First data block. This must be at least 1 for 1k-block filesystems and is
  /// typically 0 for all other block sizes.
  pub first_data_block: u32, // 0x14
  /// Block size is 2 ^ (10 + log_block_size).
  pub log_block_size: u32, // 0x18
  /// Cluster size is 2 ^ (10 + log_cluster_size) blocks if bigallox is enabled.
  /// Otherwise log_cluster_size must equal log_block_size.
  pub log_cluster_size: u32, // 0x1C
  /// Blocks per group.
  pub blocks_per_group: u32, // 0x20
  /// Clusters per group, if bigalloc is enabled. Otherwise clusters_per_group
  /// must equal blocks_per_group.
  pub clusters_per_group: u32, // 0x24
  /// Inodes per group.
  pub inodes_per_group: u32, // 0x28
  /// Mount time.
  pub mtime: DateTime<Utc>, // 0x2C
  /// Write time.
  pub wtime: DateTime<Utc>, // 0x30
  /// Number of mounts since the last fsck.
  pub mnt_count: u16, // 0x34
  /// Number of mounts beyond which a fsck is needed.
  pub max_mnt_count: u16, // 0x36
  /// Magic signature, 0xEF53
  magic: u16, // 0x38
  /// File system state. See super_state for more info.
  pub state: State, // 0x3A
  /// Behaviour when detecting errors. See super_errors for more info.
  pub errors: ErrorPolicy, // 0x3C
  /// Minor revision level.
  pub minor_rev_level: u16, // 0x3E
  /// Time of last check.
  pub lastcheck: DateTime<Utc>, // 0x40
  /// Maximum time between checks, in seconds.
  pub checkinterval: u32, // 0x44
  /// Creator OS.
  pub creator_os: Creator, // 0x48
  /// Revision level.
  pub rev_level: RevisionLevel, // 0x4C
  /// Default uid for reserved blocks.
  pub def_resuid: u16, // 0x50
  /// Default gid for reserved blocks.
  pub def_resgid: u16, // 0x52
}

impl Superblock
{
  pub fn new<R>(mut inner: R) -> Result<Self, SuperblockError>
  where
    R: io::Read,
  {
    let mut block: [u8; 84] = [0; 84];
    inner.read_exact(&mut block)?;

    let superblock = Self {
      inodes_count: u32::from_le_bytes(block[0..4].try_into().unwrap()),
      blocks_count_lo: u32::from_le_bytes(block[4..8].try_into().unwrap()),
      r_blocks_count_lo: u32::from_le_bytes(block[8..12].try_into().unwrap()),
      free_blocks_count_lo: u32::from_le_bytes(block[12..16].try_into().unwrap()),
      free_inodes_count: u32::from_le_bytes(block[16..20].try_into().unwrap()),
      first_data_block: u32::from_le_bytes(block[20..24].try_into().unwrap()),
      log_block_size: u32::from_le_bytes(block[24..28].try_into().unwrap()),
      log_cluster_size: u32::from_le_bytes(block[28..32].try_into().unwrap()),
      blocks_per_group: u32::from_le_bytes(block[32..36].try_into().unwrap()),
      clusters_per_group: u32::from_le_bytes(block[36..40].try_into().unwrap()),
      inodes_per_group: u32::from_le_bytes(block[40..44].try_into().unwrap()),
      mtime: Utc.timestamp(
        u32::from_le_bytes(block[44..48].try_into().unwrap()).into(),
        0,
      ),
      wtime: Utc.timestamp(
        u32::from_le_bytes(block[48..52].try_into().unwrap()).into(),
        0,
      ),
      mnt_count: u16::from_le_bytes(block[52..54].try_into().unwrap()),
      max_mnt_count: u16::from_le_bytes(block[54..56].try_into().unwrap()),
      magic: u16::from_le_bytes(block[56..58].try_into().unwrap()),
      state: State::from(u16::from_le_bytes(block[58..60].try_into().unwrap())),
      errors: ErrorPolicy::from(u16::from_le_bytes(block[60..62].try_into().unwrap())),
      minor_rev_level: u16::from_le_bytes(block[62..64].try_into().unwrap()),
      lastcheck: Utc.timestamp(
        u32::from_le_bytes(block[64..68].try_into().unwrap()).into(),
        0,
      ),
      checkinterval: u32::from_le_bytes(block[68..72].try_into().unwrap()),
      creator_os: Creator::from(u32::from_le_bytes(block[72..76].try_into().unwrap())),
      rev_level: RevisionLevel::from(u32::from_le_bytes(block[76..80].try_into().unwrap())),
      def_resuid: u16::from_le_bytes(block[80..82].try_into().unwrap()),
      def_resgid: u16::from_le_bytes(block[82..84].try_into().unwrap()),
    };

    if superblock.magic != EXT4_MAGIC_SIGNATURE {
      return Err(SuperblockError::Signature(superblock.magic));
    } else {
      Ok(superblock)
    }
  }

  pub fn block_size(&self) -> u32
  {
    (2 as u32).pow(10 + self.log_block_size)
  }

  pub fn cluster_size(&self) -> u32
  {
    (2 as u32).pow(10 + self.log_cluster_size)
  }
}

#[derive(Debug)]
pub struct State
{
  pub cleanly_unmounted: bool,
  pub errors_detected: bool,
  pub orphans_being_recovered: bool,
}

impl From<u16> for State
{
  fn from(state: u16) -> Self
  {
    Self {
      cleanly_unmounted: state & 0x0001 != 0,
      errors_detected: state & 0x0002 != 0,
      orphans_being_recovered: state & 0x0004 != 0,
    }
  }
}

impl std::fmt::Display for State
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub enum ErrorPolicy
{
  Continue,
  RemountReadOnly,
  Panic,
  Unknown,
}

impl From<u16> for ErrorPolicy
{
  fn from(error: u16) -> Self
  {
    match error {
      1 => Self::Continue,
      2 => Self::RemountReadOnly,
      3 => Self::Panic,
      _ => Self::Unknown,
    }
  }
}

impl std::fmt::Display for ErrorPolicy
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub enum Creator
{
  Linux,
  Hurd,
  Masix,
  FreeBSD,
  Lites,
  Unknown,
}

impl From<u32> for Creator
{
  fn from(creator: u32) -> Self
  {
    match creator {
      0 => Self::Linux,
      1 => Self::Hurd,
      2 => Self::Masix,
      3 => Self::FreeBSD,
      4 => Self::Lites,
      _ => Self::Unknown,
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

#[derive(Debug)]
pub enum RevisionLevel
{
  Original,
  Dynamic,
  Unknown,
}

impl From<u32> for RevisionLevel
{
  fn from(rev: u32) -> Self
  {
    match rev {
      0 => Self::Original,
      1 => Self::Dynamic,
      _ => Self::Unknown,
    }
  }
}

impl std::fmt::Display for RevisionLevel
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub enum SuperblockError
{
  IOError(io::Error),
  Signature(u16),
}

impl From<io::Error> for SuperblockError
{
  fn from(e: io::Error) -> Self
  {
    Self::IOError(e)
  }
}

impl std::fmt::Display for SuperblockError
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::IOError(io_err) => format!(
          "An IO error occurred while reading the superblock: {}",
          io_err
        ),
        Self::Signature(sig) => format!(
          "Expected magic number was {:#06x} but found {:#06x}.",
          EXT4_MAGIC_SIGNATURE, sig
        ),
      }
    )
  }
}
