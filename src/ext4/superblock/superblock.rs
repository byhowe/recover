use super::{
  CharEncoding, ChecksumType, Creator, DefaultMountOptions, EncryptionMode, ErrorPolicy,
  FeatureCompat, FeatureIncompat, Flags, HashVersion, ReadOnlyFeatureCompat, RevisionLevel, State,
  SuperblockRaw,
};
use crate::uuid::Uuid;
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::convert::{TryFrom, TryInto};
use std::io;

#[derive(Debug)]
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
  pub magic: u16, // 56 - 58
  /// File system state.
  pub state: State, // 58 - 60
  /// Behaviour when detecting errors.
  pub errors: ErrorPolicy, // 60 - 62
  /// Minor revision level.
  pub minor_rev_level: u16, // 62 - 64
  /// Time of last check, in seconds since the epoch.
  pub lastcheck: DateTime<Utc>, // 64 - 68
  /// Maximum time between checks, in seconds.
  pub checkinterval: Duration, // 68 - 72
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
  pub feature_compat: FeatureCompat, // 92 - 96
  /// Incompatible feature set. If the kernel or fsck doesn’t understand one of
  /// these bits, it should stop.
  pub feature_incompat: FeatureIncompat, // 96 - 100
  /// Readonly-compatible feature set. If the kernel doesn’t understand one of
  /// these bits, it can still mount read-only.
  pub feature_ro_compat: ReadOnlyFeatureCompat, // 100 - 104
  /// 128-bit UUID for volume.
  pub uuid: Uuid, // 104 - 120
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
  pub journal_uuid: Uuid, // 208 - 224
  /// inode number of journal file.
  pub journal_inum: u32, // 224 - 228
  /// Device number of journal file, if the external journal feature flag is
  /// set.
  pub journal_dev: u32, // 228 - 232
  /// Start of list of orphaned inodes to delete.
  pub last_orphan: u32, // 232 - 236
  /// HTREE hash seed.
  pub hash_seed: Uuid, // 236 - 252
  /// Default hash algorithm to use for directory hashes.
  pub def_hash_version: HashVersion, // 252 - 253
  /// If this value is 0 or EXT3_JNL_BACKUP_BLOCKS (1), then the jnl_blocks
  /// field contains a duplicate copy of the inode’s i_block[] array and i_size.
  pub jnl_backup_type: u8, // 253 - 254
  /// Size of group descriptors, in bytes, if the 64bit incompat feature flag is
  /// set.
  pub desc_size: u16, // 254 - 256
  /// Default mount options.
  pub default_mount_opts: DefaultMountOptions, // 256 - 260
  /// First metablock block group, if the meta_bg feature is enabled.
  pub first_meta_bg: u32, // 260 - 264
  /// When the filesystem was created, in seconds since the epoch.
  pub mkfs_time: DateTime<Utc>, // 264 - 268
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
  pub flags: Flags, // 352 - 356
  /// RAID stride. This is the number of logical blocks read from or written to
  /// the disk before moving to the next disk. This affects the placement of
  /// filesystem metadata, which will hopefully make RAID storage faster.
  pub raid_stride: u16, // 356 - 358
  /// #. seconds to wait in multi-mount prevention (MMP) checking. In theory,
  /// MMP is a mechanism to record in the superblock which host and device have
  /// mounted the filesystem, in order to prevent multiple mounts. This feature
  /// does not seem to be implemented...
  pub mmp_interval: Duration, // 358 - 360
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
  pub checksum_type: ChecksumType, // 373 - 374
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
  pub first_error_time: DateTime<Utc>, // 408 - 412
  /// inode involved in first error.
  pub first_error_ino: u32, // 412 - 416
  /// Number of block involved of first error.
  pub first_error_block: u64, // 416 - 424
  /// Name of function where the error happened.
  pub first_error_func: String, // 424 - 456
  /// Line number where error happened.
  pub first_error_line: u32, // 456 - 460
  /// Time of most recent error, in seconds since the epoch.
  pub last_error_time: DateTime<Utc>, // 460 - 464
  /// inode involved in most recent error.
  pub last_error_ino: u32, // 464 - 468
  /// Line number where most recent error happened.
  pub last_error_line: u32, // 468 - 472
  /// Number of block involved in most recent error.
  pub last_error_block: u64, // 472 - 480
  /// Name of function where the most recent error happened.
  pub last_error_func: String, // 480 - 512
  /// ASCIIZ string of mount options.
  pub mount_opts: String, // 512 - 576
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
  pub encrypt_algos: Vec<EncryptionMode>, // 596 - 600
  /// Salt for the string2key algorithm for encryption.
  pub encrypt_pw_salt: Uuid, // 600 - 616
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
  /// Filename charset encoding.
  pub encoding: CharEncoding, // 636 - 638
  /// Filename charset encoding flags.
  pub encoding_flags: u16, // 638 - 640
  /// Superblock checksum.
  pub checksum: u32, // 1020 - 1024
}

impl Superblock
{
  pub const MAGIC_SIGNATURE: u16 = 0xEF53;

  pub const GOOD_OLD_FIRST_INO: u32 = 11;
  pub const GOOD_OLD_REV: RevisionLevel = RevisionLevel::Original;
  pub const GOOD_OLD_INODE_SIZE: u16 = 128;

  pub fn new<R>(mut inner: R) -> Result<Self, Error>
  where
    R: io::Read,
  {
    let mut block: [u8; 1024] = [0; 1024];
    inner.read_exact(&mut block)?;
    let raw: SuperblockRaw = block.into();
    Ok(raw.try_into()?)
  }

  pub fn check_signature(&self) -> Option<Error>
  {
    if self.magic == Self::MAGIC_SIGNATURE {
      None
    } else {
      Some(Error::Signature(self.magic))
    }
  }

  pub fn get_block_size(&self) -> u32
  {
    2u32.pow(10 + self.log_block_size)
  }

  pub fn get_cluster_size(&self) -> u32
  {
    2u32.pow(10 + self.log_cluster_size)
  }

  pub fn get_flexible_block_group(&self) -> u32
  {
    2u32.pow(self.log_groups_per_flex.into())
  }

  pub fn get_inode_size(&self) -> u16
  {
    if self.rev_level == Self::GOOD_OLD_REV {
      Self::GOOD_OLD_INODE_SIZE
    } else {
      self.inode_size
    }
  }

  pub fn get_blocks_count(&self) -> u64
  {
    self.blocks_count_lo as u64
      | if self.feature_incompat.bit64 {
        (self.blocks_count_hi as u64) << 32
      } else {
        0
      }
  }

  pub fn get_reserved_blocks_count(&self) -> u64
  {
    self.r_blocks_count_lo as u64
      | if self.feature_incompat.bit64 {
        (self.r_blocks_count_hi as u64) << 32
      } else {
        0
      }
  }

  pub fn get_free_blocks_count(&self) -> u64
  {
    self.free_blocks_count_lo as u64
      | if self.feature_incompat.bit64 {
        (self.free_blocks_count_hi as u64) << 32
      } else {
        0
      }
  }

  pub fn get_features(&self) -> Vec<&str>
  {
    let mut features = self.feature_compat.features_list();
    features.append(&mut self.feature_incompat.features_list());
    features.append(&mut self.feature_ro_compat.features_list());
    if self.feature_compat.unknown_bits
      || self.feature_incompat.unknown_bits
      || self.feature_ro_compat.unknown_bits
    {
      features.push("(unknown_bits)");
    }
    features
  }
}

impl TryFrom<SuperblockRaw> for Superblock
{
  type Error = Error;

  fn try_from(raw: SuperblockRaw) -> Result<Self, Self::Error>
  {
    Ok(Self {
      inodes_count: raw.inodes_count,
      blocks_count_lo: raw.blocks_count_lo,
      r_blocks_count_lo: raw.r_blocks_count_lo,
      free_blocks_count_lo: raw.free_blocks_count_lo,
      free_inodes_count: raw.free_inodes_count,
      first_data_block: raw.first_data_block,
      log_block_size: raw.log_block_size,
      log_cluster_size: raw.log_cluster_size,
      blocks_per_group: raw.blocks_per_group,
      clusters_per_group: raw.clusters_per_group,
      inodes_per_group: raw.inodes_per_group,
      mtime: Utc.timestamp(raw.mtime as i64, 0),
      wtime: Utc.timestamp(raw.wtime as i64, 0),
      mnt_count: raw.mnt_count,
      max_mnt_count: raw.max_mnt_count,
      magic: raw.magic,
      state: State::from_raw(raw.state),
      errors: ErrorPolicy::from_raw(raw.errors),
      minor_rev_level: raw.minor_rev_level,
      lastcheck: Utc.timestamp(raw.lastcheck as i64, 0),
      checkinterval: Duration::seconds(raw.checkinterval as i64),
      creator_os: Creator::from_raw(raw.creator_os),
      rev_level: RevisionLevel::from_raw(raw.rev_level),
      def_resuid: raw.def_resuid,
      def_resgid: raw.def_resgid,
      first_ino: raw.first_ino,
      inode_size: raw.inode_size,
      block_group_nr: raw.block_group_nr,
      feature_compat: FeatureCompat::from_raw(raw.feature_compat),
      feature_incompat: FeatureIncompat::from_raw(raw.feature_incompat),
      feature_ro_compat: ReadOnlyFeatureCompat::from_raw(raw.feature_ro_compat),
      uuid: Uuid::from(raw.uuid),
      volume_name: String::from_utf8(raw.volume_name.to_vec())?,
      last_mounted: String::from_utf8(raw.last_mounted.to_vec())?,
      algorithm_usage_bitmap: raw.algorithm_usage_bitmap,
      prealloc_blocks: raw.prealloc_blocks,
      prealloc_dir_blocks: raw.prealloc_dir_blocks,
      reserved_gdt_blocks: raw.reserved_gdt_blocks,
      journal_uuid: Uuid::from(raw.journal_uuid),
      journal_inum: raw.journal_inum,
      journal_dev: raw.journal_dev,
      last_orphan: raw.last_orphan,
      hash_seed: Uuid::from(raw.hash_seed),
      def_hash_version: HashVersion::from_raw(raw.def_hash_version),
      jnl_backup_type: raw.jnl_backup_type,
      desc_size: raw.desc_size,
      default_mount_opts: DefaultMountOptions::from_raw(raw.default_mount_opts),
      first_meta_bg: raw.first_meta_bg,
      mkfs_time: Utc.timestamp(raw.mkfs_time as i64, 0),
      jnl_blocks: raw.jnl_blocks,
      blocks_count_hi: raw.blocks_count_hi,
      r_blocks_count_hi: raw.r_blocks_count_hi,
      free_blocks_count_hi: raw.free_blocks_count_hi,
      min_extra_isize: raw.min_extra_isize,
      want_extra_isize: raw.want_extra_isize,
      flags: Flags::from_raw(raw.flags),
      raid_stride: raw.raid_stride,
      mmp_interval: Duration::seconds(raw.mmp_interval as i64),
      mmp_block: raw.mmp_block,
      raid_stripe_width: raw.raid_stripe_width,
      log_groups_per_flex: raw.log_groups_per_flex,
      checksum_type: ChecksumType::from_raw(raw.checksum_type),
      kbytes_written: raw.kbytes_written,
      snapshot_inum: raw.snapshot_inum,
      snapshot_id: raw.snapshot_id,
      snapshot_r_blocks_count: raw.snapshot_r_blocks_count,
      snapshot_list: raw.snapshot_list,
      error_count: raw.error_count,
      first_error_time: Utc.timestamp(raw.first_error_time as i64, 0),
      first_error_ino: raw.first_error_ino,
      first_error_block: raw.first_error_block,
      first_error_func: String::from_utf8(raw.first_error_func.to_vec())?,
      first_error_line: raw.first_error_line,
      last_error_time: Utc.timestamp(raw.last_error_time as i64, 0),
      last_error_ino: raw.last_error_ino,
      last_error_line: raw.last_error_line,
      last_error_block: raw.last_error_block,
      last_error_func: String::from_utf8(raw.last_error_func.to_vec())?,
      mount_opts: String::from_utf8(raw.mount_opts.to_vec())?,
      usr_quota_inum: raw.usr_quota_inum,
      grp_quota_inum: raw.grp_quota_inum,
      overhead_blocks: raw.overhead_blocks,
      backup_bgs: raw.backup_bgs,
      encrypt_algos: EncryptionMode::from_modes(&raw.encrypt_algos),
      encrypt_pw_salt: Uuid::from(raw.encrypt_pw_salt),
      lpf_ino: raw.lpf_ino,
      prj_quota_inum: raw.prj_quota_inum,
      checksum_seed: raw.checksum_seed,
      wtime_hi: raw.wtime_hi,
      mtime_hi: raw.mtime_hi,
      mkfs_time_hi: raw.mkfs_time_hi,
      lastcheck_hi: raw.lastcheck_hi,
      first_error_time_hi: raw.first_error_time_hi,
      last_error_time_hi: raw.last_error_time_hi,
      encoding: CharEncoding::from(raw.encoding),
      encoding_flags: raw.encoding_flags,
      checksum: raw.checksum,
    })
  }
}

#[derive(Debug)]
pub enum Error
{
  IOError(io::Error),
  StringError(std::string::FromUtf8Error),
  Signature(u16),
}

impl From<io::Error> for Error
{
  fn from(e: io::Error) -> Self
  {
    Self::IOError(e)
  }
}

impl From<std::string::FromUtf8Error> for Error
{
  fn from(error: std::string::FromUtf8Error) -> Self
  {
    Self::StringError(error)
  }
}

impl std::fmt::Display for Error
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
      }
    )
  }
}
