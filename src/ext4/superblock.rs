use chrono::{DateTime, TimeZone, Utc};
use std::convert::TryInto;
use std::io;

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
  const MAGIC_SIGNATURE: u16 = 0xEF53;

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
      state: State::from_raw(u16::from_le_bytes(block[58..60].try_into().unwrap()))?,
      errors: ErrorPolicy::from_raw(u16::from_le_bytes(block[60..62].try_into().unwrap()))?,
      minor_rev_level: u16::from_le_bytes(block[62..64].try_into().unwrap()),
      lastcheck: Utc.timestamp(
        u32::from_le_bytes(block[64..68].try_into().unwrap()).into(),
        0,
      ),
      checkinterval: u32::from_le_bytes(block[68..72].try_into().unwrap()),
      creator_os: Creator::from_raw(u32::from_le_bytes(block[72..76].try_into().unwrap()))?,
      rev_level: RevisionLevel::from_raw(u32::from_le_bytes(block[76..80].try_into().unwrap()))?,
      def_resuid: u16::from_le_bytes(block[80..82].try_into().unwrap()),
      def_resgid: u16::from_le_bytes(block[82..84].try_into().unwrap()),
    };

    if superblock.magic != Self::MAGIC_SIGNATURE {
      return Err(SuperblockError::Signature(superblock.magic));
    } else {
      Ok(superblock)
    }
  }

  pub fn block_size(&self) -> u32
  {
    2u32.pow(10 + self.log_block_size)
  }

  pub fn cluster_size(&self) -> u32
  {
    2u32.pow(10 + self.log_cluster_size)
  }
}

#[derive(Debug)]
pub struct State
{
  pub cleanly_unmounted: bool,
  pub errors_detected: bool,
  pub orphans_being_recovered: bool,
}

impl State
{
  const CLEANLY_UNMOUNTED: u16 = 0x0001;
  const ERRORS_DETECTED: u16 = 0x0002;
  const ORPHANS_BEING_RECOVERED: u16 = 0x0004;

  fn from_raw(state: u16) -> Result<Self, UnexpectedValue>
  {
    let state_construct = Self {
      cleanly_unmounted: state & Self::CLEANLY_UNMOUNTED != 0,
      errors_detected: state & Self::ERRORS_DETECTED != 0,
      orphans_being_recovered: state & Self::ORPHANS_BEING_RECOVERED != 0,
    };

    if state & !(Self::CLEANLY_UNMOUNTED | Self::ERRORS_DETECTED | Self::ORPHANS_BEING_RECOVERED)
      != 0
    {
      Err(UnexpectedValue::State(state))
    } else {
      Ok(state_construct)
    }
  }
}

#[derive(Debug)]
pub enum ErrorPolicy
{
  Continue,
  RemountReadOnly,
  Panic,
}

impl ErrorPolicy
{
  const CONTINUE: u16 = 1;
  const REMOUNT_READ_ONLY: u16 = 2;
  const PANIC: u16 = 3;

  fn from_raw(error: u16) -> Result<Self, UnexpectedValue>
  {
    match error {
      Self::CONTINUE => Ok(Self::Continue),
      Self::REMOUNT_READ_ONLY => Ok(Self::RemountReadOnly),
      Self::PANIC => Ok(Self::Panic),
      _ => Err(UnexpectedValue::ErrorPolicy(error)),
    }
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
}

impl Creator
{
  const LINUX: u32 = 0;
  const HURD: u32 = 1;
  const MASIX: u32 = 2;
  const FREE_BSD: u32 = 3;
  const LITES: u32 = 4;

  fn from_raw(creator: u32) -> Result<Self, UnexpectedValue>
  {
    match creator {
      Self::LINUX => Ok(Self::Linux),
      Self::HURD => Ok(Self::Hurd),
      Self::MASIX => Ok(Self::Masix),
      Self::FREE_BSD => Ok(Self::FreeBSD),
      Self::LITES => Ok(Self::Lites),
      _ => Err(UnexpectedValue::Creator(creator)),
    }
  }
}

#[derive(Debug)]
pub enum RevisionLevel
{
  Original,
  Dynamic,
}

impl RevisionLevel
{
  const ORIGINAL_FORMAT: u32 = 0;
  const V2_FORMAT: u32 = 1;

  fn from_raw(rev: u32) -> Result<Self, UnexpectedValue>
  {
    match rev {
      Self::ORIGINAL_FORMAT => Ok(Self::Original),
      Self::V2_FORMAT => Ok(Self::Dynamic),
      _ => Err(UnexpectedValue::RevisionLevel(rev)),
    }
  }
}

#[derive(Debug)]
pub enum UnexpectedValue
{
  State(u16),
  ErrorPolicy(u16),
  Creator(u32),
  RevisionLevel(u32),
}

impl std::fmt::Display for UnexpectedValue
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::State(state) => format!("Unknown state flag value: {:#018b}", state),
        Self::ErrorPolicy(error) => format!("Unknown error policy value: {}", error),
        Self::Creator(creator) => format!("Unknown creator OS value: {}", creator),
        Self::RevisionLevel(rev) => format!("Unknown revision level value: {}", rev),
      }
    )
  }
}

#[derive(Debug)]
pub enum SuperblockError
{
  IOError(io::Error),
  Signature(u16),
  UnexpectedValue(UnexpectedValue),
}

impl From<io::Error> for SuperblockError
{
  fn from(e: io::Error) -> Self
  {
    Self::IOError(e)
  }
}

impl From<UnexpectedValue> for SuperblockError
{
  fn from(unexpected: UnexpectedValue) -> Self
  {
    Self::UnexpectedValue(unexpected)
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
          Superblock::MAGIC_SIGNATURE,
          sig
        ),
        Self::UnexpectedValue(field) => format!("{}", field),
      }
    )
  }
}
