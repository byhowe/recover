use byteorder::{ByteOrder, LittleEndian};
use chrono::{DateTime, TimeZone, Utc};
use std::convert::TryInto;
use std::io;

#[derive(Debug)]
#[repr(C)]
pub struct Superblock
{
  /// Total inode count.
  pub inodes_count: u32, // 0 - 4
  /// Total block count.
  pub blocks_count_lo: u32, // 4 - 8
  /// This number of blocks can only be allocated by the super-user.
  pub r_blocks_count_lo: u32, // 8 - 12
  /// Free block count.
  pub free_blocks_count_lo: u32, // 12 - 16
  /// Free inode count.
  pub free_inodes_count: u32, // 16 - 20
  /// First data block. This must be at least 1 for 1k-block filesystems and is
  /// typically 0 for all other block sizes.
  pub first_data_block: u32, // 20 - 24
  /// Block size is 2 ^ (10 + log_block_size).
  pub log_block_size: u32, // 24 - 28
  /// Cluster size is 2 ^ (10 + log_cluster_size) blocks if bigalloc is enabled.
  /// Otherwise log_cluster_size must equal log_block_size.
  pub log_cluster_size: u32, // 28 - 32
  /// Blocks per group.
  pub blocks_per_group: u32, // 32 - 36
  /// Clusters per group, if bigalloc is enabled. Otherwise clusters_per_group
  /// must equal blocks_per_group.
  pub clusters_per_group: u32, // 36 - 40
  /// Inodes per group.
  pub inodes_per_group: u32, // 40 - 44
  /// Mount time, in seconds since the epoch.
  pub mtime: DateTime<Utc>, // 44 - 48
  /// Write time, in seconds since the epoch.
  pub wtime: DateTime<Utc>, // 48 - 52
  /// Number of mounts since the last fsck.
  pub mnt_count: u16, // 52 - 54
  /// Number of mounts beyond which a fsck is needed.
  pub max_mnt_count: u16, // 54 - 56
  /// Magic signature, 0xEF53
  magic: u16, // 56 - 58
  /// File system state.
  pub state: State, // 58 - 60
  /// Behaviour when detecting errors.
  pub errors: ErrorPolicy, // 60 - 62
  /// Minor revision level.
  pub minor_rev_level: u16, // 62 - 64
  /// Time of last check, in seconds since the epoch.
  pub lastcheck: DateTime<Utc>, // 64 - 68
  /// Maximum time between checks, in seconds.
  pub checkinterval: u32, // 68 - 72
  /// Creator OS.
  pub creator_os: Creator, // 72 - 76
  /// Revision level.
  pub rev_level: RevisionLevel, // 76 - 80
  /// Default uid for reserved blocks.
  pub def_resuid: u16, // 80 - 82
  /// Default gid for reserved blocks.
  pub def_resgid: u16, // 82 - 84
  /// First non-reserved inode.
  pub first_ino: u32, // 84 - 88
  /// Size of inode structure, in bytes.
  pub inode_size: u16, // 88 - 90
  /// Block group # of this superblock.
  pub block_group_nr: u16, // 90 - 92
  /// Compatible feature set flags. Kernel can still read/write this fs even if
  /// it doesn’t understand a flag; fsck should not do that.
  pub feature_compat: u32, // 92 - 96
  /// Incompatible feature set. If the kernel or fsck doesn’t understand one of
  /// these bits, it should stop.
  pub feature_incompat: u32, // 96 - 100
  /// Readonly-compatible feature set. If the kernel doesn’t understand one of
  /// these bits, it can still mount read-only.
  pub feature_ro_compat: u32, // 100 - 104
  /// 128-bit UUID for volume.
  pub uuid: [u8; 16], // 104 - 120
  /// Volume label.
  pub volume_name: String, // 120 - 136
  /// Directory where filesystem was last mounted.
  pub last_mounted: String, // 136 - 200
  /// For compression (Not used in e2fsprogs/Linux)
  pub algorithm_usage_bitmap: u32, // 200 - 204
  /// #. of blocks to try to preallocate for ... files? (Not used in
  /// e2fsprogs/Linux)
  pub prealloc_blocks: u8, // 204 - 205
  /// #. of blocks to preallocate for directories. (Not used in e2fsprogs/Linux)
  pub prealloc_dir_blocks: u8, // 205 - 206
  /// Number of reserved GDT entries for future filesystem expansion.
  pub reserved_gdt_blocks: u16, // 206 - 208
  /// UUID of journal superblock.
  pub journal_uuid: [u8; 16], // 208 - 224
  /// inode number of journal file.
  pub journal_inum: u32, // 224 - 228
  /// Device number of journal file, if the external journal feature flag is
  /// set.
  pub journal_dev: u32, // 228 - 232
  /// Start of list of orphaned inodes to delete.
  pub last_orphan: u32, // 232 - 236
  /// HTREE hash seed.
  pub hash_seed: [u32; 4], // 236 - 252
  /// Default hash algorithm to use for directory hashes.
  pub def_hash_version: u8, // 252 - 253
  /// If this value is 0 or EXT3_JNL_BACKUP_BLOCKS (1), then the jnl_blocks
  /// field contains a duplicate copy of the inode’s i_block[] array and i_size.
  pub jnl_backup_type: u8, // 253 - 254
  /// Size of group descriptors, in bytes, if the 64bit incompat feature flag is
  /// set.
  pub desc_size: u16, // 254 - 256
  /// Default mount options.
  pub default_mount_opts: u32, // 256 - 260
  /// First metablock block group, if the meta_bg feature is enabled.
  pub first_meta_bg: u32, // 260 - 264
  /// When the filesystem was created, in seconds since the epoch.
  pub mkfs_time: u32, // 264 - 268
  /// Backup copy of the journal inode’s i_block[] array in the first 15
  /// elements and i_size_high and i_size in the 16th and 17th elements,
  /// respectively.
  pub jnl_blocks: [u32; 17], // 268 - 336
  /// High 32-bits of the block count.
  pub blocks_count_hi: u32, // 336 - 340
  /// High 32-bits of the reserved block count.
  pub r_blocks_count_hi: u32, // 340 - 344
  /// High 32-bits of the free block count.
  pub free_blocks_count_hi: u32, // 344 - 348
  /// All inodes have at least # bytes.
  pub min_extra_isize: u16, // 348 - 350
  /// New inodes should reserve # bytes.
  pub want_extra_isize: u16, // 350 - 352
  /// Miscellaneous flags.
  pub flags: u32, // 352 - 356
  /// RAID stride. This is the number of logical blocks read from or written to
  /// the disk before moving to the next disk. This affects the placement of
  /// filesystem metadata, which will hopefully make RAID storage faster.
  pub raid_stride: u16, // 356 - 358
  /// #. seconds to wait in multi-mount prevention (MMP) checking. In theory,
  /// MMP is a mechanism to record in the superblock which host and device have
  /// mounted the filesystem, in order to prevent multiple mounts. This feature
  /// does not seem to be implemented...
  pub mmp_interval: u16, // 358 - 360
  /// Block # for multi-mount protection data.
  pub mmp_block: u64, // 360 - 368
  /// RAID stripe width. This is the number of logical blocks read from or
  /// written to the disk before coming back to the current disk. This is used
  /// by the block allocator to try to reduce the number of read-modify-write
  /// operations in a RAID5/6.
  pub raid_stripe_width: u32, // 368 - 372
  /// Size of a flexible block group is 2 ^ log_groups_per_flex.
  pub log_groups_per_flex: u8, // 372 - 373
  /// Metadata checksum algorithm type. The only valid value is 1 (crc32c).
  pub checksum_type: u8, // 373 - 374
  pub reserved_pad: u16, // 374 - 376
  /// Number of KiB written to this filesystem over its lifetime.
  pub kbytes_written: u64, // 376 - 384
  /// inode number of active snapshot. (Not used in e2fsprogs/Linux.)
  pub snapshot_inum: u32, // 384 - 388
  /// Sequential ID of active snapshot. (Not used in e2fsprogs/Linux.)
  pub snapshot_id: u32, // 388 - 392
  /// Number of blocks reserved for active snapshot’s future use. (Not used in
  /// e2fsprogs/Linux.)
  pub snapshot_r_blocks_count: u64, // 392 - 400
  /// inode number of the head of the on-disk snapshot list. (Not used in
  /// e2fsprogs/Linux.)
  pub snapshot_list: u32, // 400 - 404
  /// Number of errors seen.
  pub error_count: u32, // 404 - 408
  /// First time an error happened, in seconds since the epoch.
  pub first_error_time: u32, // 408 - 412
  /// inode involved in first error.
  pub first_error_ino: u32, // 412 - 416
  /// Number of block involved of first error.
  pub first_error_block: u64, // 416 - 424
  /// Name of function where the error happened.
  pub first_error_func: [u8; 32], // 424 - 456
  /// Line number where error happened.
  pub first_error_line: u32, // 456 - 460
  /// Time of most recent error, in seconds since the epoch.
  pub last_error_time: u32, // 460 - 464
  /// inode involved in most recent error.
  pub last_error_ino: u32, // 464 - 468
  /// Line number where most recent error happened.
  pub last_error_line: u32, // 468 - 472
  /// Number of block involved in most recent error.
  pub last_error_block: u64, // 472 - 480
  /// Name of function where the most recent error happened.
  pub last_error_func: [u8; 32], // 480 - 512
  /// ASCIIZ string of mount options.
  pub mount_opts: [u8; 64], // 512 - 576
  /// Inode number of user quota file.
  pub usr_quota_inum: u32, // 576 - 580
  /// Inode number of group quota file.
  pub grp_quota_inum: u32, // 580 - 584
  /// Overhead blocks/clusters in fs. (Huh? This field is always zero, which
  /// means that the kernel calculates it dynamically.)
  pub overhead_blocks: u32, // 584 - 588
  /// Block groups containing superblock backups (if sparse_super2)
  pub backup_bgs: [u32; 2], // 588 - 596
  /// Encryption algorithms in use. There can be up to four algorithms in use at
  /// any time.
  pub encrypt_algos: [u8; 4], // 596 - 600
  /// Salt for the string2key algorithm for encryption.
  pub encrypt_pw_salt: [u8; 16], // 600 - 616
  /// Inode number of lost+found.
  pub lpf_ino: u32, // 616 - 620
  /// Inode that tracks project quotas.
  pub prj_quota_inum: u32, // 620 - 624
  /// Checksum seed used for metadata_csum calculations. This value is
  /// crc32c(~0, $orig_fs_uuid).
  pub checksum_seed: u32, // 624 - 628
  /// Upper 8 bits of the wtime field.
  pub wtime_hi: u8, // 628 - 629
  /// Upper 8 bits of the mtime field.
  pub mtime_hi: u8, // 629 - 630
  /// Upper 8 bits of the mkfs_time field.
  pub mkfs_time_hi: u8, // 630 - 631
  /// Upper 8 bits of the lastcheck_hi field.
  pub lastcheck_hi: u8, // 631 - 632
  /// Upper 8 bits of the first_error_time_hi field.
  pub first_error_time_hi: u8, // 632 - 633
  /// Upper 8 bits of the last_error_time_hi field.
  pub last_error_time_hi: u8, // 633 - 634
  /// Zero padding.
  pub pad: [u8; 2], // 634 - 636
  /// Filename charset encoding.
  pub encoding: u16, // 636 - 638
  /// Filename charset encoding flags.
  pub encoding_flags: u16, // 638 - 640
  /// Padding to the end of the block.
  pub reserved: [u32; 95], // 640 - 1020
  /// Superblock checksum.
  pub checksum: u32, // 1020 - 1024
}

impl Superblock
{
  const MAGIC_SIGNATURE: u16 = 0xEF53;

  pub fn new<R>(mut inner: R) -> Result<Self, SuperblockError>
  where
    R: io::Read,
  {
    let mut block: [u8; 1024] = [0; 1024];
    inner.read_exact(&mut block)?;

    let superblock = Self {
      inodes_count: LittleEndian::read_u32(&block[0..4]),
      blocks_count_lo: LittleEndian::read_u32(&block[4..8]),
      r_blocks_count_lo: LittleEndian::read_u32(&block[8..12]),
      free_blocks_count_lo: LittleEndian::read_u32(&block[12..16]),
      free_inodes_count: LittleEndian::read_u32(&block[16..20]),
      first_data_block: LittleEndian::read_u32(&block[20..24]),
      log_block_size: LittleEndian::read_u32(&block[24..28]),
      log_cluster_size: LittleEndian::read_u32(&block[28..32]),
      blocks_per_group: LittleEndian::read_u32(&block[32..36]),
      clusters_per_group: LittleEndian::read_u32(&block[36..40]),
      inodes_per_group: LittleEndian::read_u32(&block[40..44]),
      mtime: Utc.timestamp(LittleEndian::read_u32(&block[44..48]).into(), 0),
      wtime: Utc.timestamp(LittleEndian::read_u32(&block[48..52]).into(), 0),
      mnt_count: LittleEndian::read_u16(&block[52..54]),
      max_mnt_count: LittleEndian::read_u16(&block[54..56]),
      magic: LittleEndian::read_u16(&block[56..58]),
      state: State::from_raw(LittleEndian::read_u16(&block[58..60]))?,
      errors: ErrorPolicy::from_raw(LittleEndian::read_u16(&block[60..62]))?,
      minor_rev_level: LittleEndian::read_u16(&block[62..64]),
      lastcheck: Utc.timestamp(LittleEndian::read_u32(&block[64..68]).into(), 0),
      checkinterval: LittleEndian::read_u32(&block[68..72]),
      creator_os: Creator::from_raw(LittleEndian::read_u32(&block[72..76]))?,
      rev_level: RevisionLevel::from_raw(LittleEndian::read_u32(&block[76..80]))?,
      def_resuid: LittleEndian::read_u16(&block[80..82]),
      def_resgid: LittleEndian::read_u16(&block[82..84]),
      first_ino: LittleEndian::read_u32(&block[84..88]),
      inode_size: LittleEndian::read_u16(&block[88..90]),
      block_group_nr: LittleEndian::read_u16(&block[90..92]),
      feature_compat: LittleEndian::read_u32(&block[92..96]),
      feature_incompat: LittleEndian::read_u32(&block[96..100]),
      feature_ro_compat: LittleEndian::read_u32(&block[100..104]),
      uuid: from_le_bytes_to_u8_array(&block[104..120])
        .as_slice()
        .try_into()
        .unwrap(),
      volume_name: String::from_utf8(block[120..136].to_vec())?,
      last_mounted: String::from_utf8(block[136..200].to_vec())?,
      algorithm_usage_bitmap: LittleEndian::read_u32(&block[200..204]),
      prealloc_blocks: u8::from_le(block[204]),
      prealloc_dir_blocks: u8::from_le(block[205]),
      reserved_gdt_blocks: LittleEndian::read_u16(&block[206..208]),
      journal_uuid: from_le_bytes_to_u8_array(&block[208..224])
        .as_slice()
        .try_into()
        .unwrap(),
      journal_inum: LittleEndian::read_u32(&block[224..228]),
      journal_dev: LittleEndian::read_u32(&block[228..232]),
      last_orphan: LittleEndian::read_u32(&block[232..236]),
      hash_seed: from_le_bytes_to_u32_array(&block[236..252])
        .as_slice()
        .try_into()
        .unwrap(),
      def_hash_version: u8::from_le(block[252]),
      jnl_backup_type: u8::from_le(block[253]),
      desc_size: LittleEndian::read_u16(&block[254..256]),
      default_mount_opts: LittleEndian::read_u32(&block[256..260]),
      first_meta_bg: LittleEndian::read_u32(&block[260..264]),
      mkfs_time: LittleEndian::read_u32(&block[264..268]),
      jnl_blocks: from_le_bytes_to_u32_array(&block[268..336])
        .as_slice()
        .try_into()
        .unwrap(),
      blocks_count_hi: LittleEndian::read_u32(&block[336..340]),
      r_blocks_count_hi: LittleEndian::read_u32(&block[340..344]),
      free_blocks_count_hi: LittleEndian::read_u32(&block[344..348]),
      min_extra_isize: LittleEndian::read_u16(&block[348..350]),
      want_extra_isize: LittleEndian::read_u16(&block[350..352]),
      flags: LittleEndian::read_u32(&block[352..356]),
      raid_stride: LittleEndian::read_u16(&block[356..358]),
      mmp_interval: LittleEndian::read_u16(&block[358..360]),
      mmp_block: LittleEndian::read_u64(&block[360..368]),
      raid_stripe_width: LittleEndian::read_u32(&block[368..372]),
      log_groups_per_flex: u8::from_le(block[372]),
      checksum_type: u8::from_le(block[373]),
      reserved_pad: LittleEndian::read_u16(&block[374..376]),
      kbytes_written: LittleEndian::read_u64(&block[376..384]),
      snapshot_inum: LittleEndian::read_u32(&block[384..388]),
      snapshot_id: LittleEndian::read_u32(&block[388..392]),
      snapshot_r_blocks_count: LittleEndian::read_u64(&block[392..400]),
      snapshot_list: LittleEndian::read_u32(&block[400..404]),
      error_count: LittleEndian::read_u32(&block[404..408]),
      first_error_time: LittleEndian::read_u32(&block[408..412]),
      first_error_ino: LittleEndian::read_u32(&block[412..416]),
      first_error_block: LittleEndian::read_u64(&block[416..424]),
      first_error_func: from_le_bytes_to_u8_array(&block[424..456])
        .as_slice()
        .try_into()
        .unwrap(),
      first_error_line: LittleEndian::read_u32(&block[456..460]),
      last_error_time: LittleEndian::read_u32(&block[460..464]),
      last_error_ino: LittleEndian::read_u32(&block[464..468]),
      last_error_line: LittleEndian::read_u32(&block[468..472]),
      last_error_block: LittleEndian::read_u64(&block[472..480]),
      last_error_func: from_le_bytes_to_u8_array(&block[480..512])
        .as_slice()
        .try_into()
        .unwrap(),
      mount_opts: from_le_bytes_to_u8_array(&block[512..576])
        .as_slice()
        .try_into()
        .unwrap(),
      usr_quota_inum: LittleEndian::read_u32(&block[576..580]),
      grp_quota_inum: LittleEndian::read_u32(&block[580..584]),
      overhead_blocks: LittleEndian::read_u32(&block[584..588]),
      backup_bgs: from_le_bytes_to_u32_array(&block[588..596])
        .as_slice()
        .try_into()
        .unwrap(),
      encrypt_algos: from_le_bytes_to_u8_array(&block[596..600])
        .as_slice()
        .try_into()
        .unwrap(),
      encrypt_pw_salt: from_le_bytes_to_u8_array(&block[600..616])
        .as_slice()
        .try_into()
        .unwrap(),
      lpf_ino: LittleEndian::read_u32(&block[616..620]),
      prj_quota_inum: LittleEndian::read_u32(&block[620..624]),
      checksum_seed: LittleEndian::read_u32(&block[624..628]),
      wtime_hi: u8::from_le(block[628]),
      mtime_hi: u8::from_le(block[629]),
      mkfs_time_hi: u8::from_le(block[630]),
      lastcheck_hi: u8::from_le(block[631]),
      first_error_time_hi: u8::from_le(block[632]),
      last_error_time_hi: u8::from_le(block[633]),
      pad: from_le_bytes_to_u8_array(&block[634..636])
        .as_slice()
        .try_into()
        .unwrap(),
      encoding: LittleEndian::read_u16(&block[636..638]),
      encoding_flags: LittleEndian::read_u16(&block[638..640]),
      reserved: from_le_bytes_to_u32_array(&block[640..1020])
        .as_slice()
        .try_into()
        .unwrap(),
      checksum: LittleEndian::read_u32(&block[1020..1024]),
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

fn from_le_bytes_to_u8_array(array: &[u8]) -> Vec<u8>
{
  array.iter().map(|&e| u8::from_le(e)).collect::<Vec<u8>>()
}

fn from_le_bytes_to_u32_array(array: &[u8]) -> Vec<u32>
{
  let u32_length = array.len() / 4;
  let mut u32_vec = Vec::with_capacity(u32_length);
  for i in 0..u32_length {
    u32_vec.push(LittleEndian::read_u32(&array[i * 4..i * 4 + 4]))
  }
  u32_vec
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
  StringError(std::string::FromUtf8Error),
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

impl From<std::string::FromUtf8Error> for SuperblockError
{
  fn from(error: std::string::FromUtf8Error) -> Self
  {
    Self::StringError(error)
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
        Self::StringError(str_err) => format!("A string error occurred: {}", str_err),
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
