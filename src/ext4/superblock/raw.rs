use crate::uuid::UuidRaw;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct SuperblockRaw
{
  /// Total inode count.
  pub(crate) s_inodes_count: u32, // 0 - 4
  /// Total block count.
  pub(crate) s_blocks_count_lo: u32, // 4 - 8
  /// This number of blocks can only be allocated by the super-user.
  pub(crate) s_r_blocks_count_lo: u32, // 8 - 12
  /// Free block count.
  pub(crate) s_free_blocks_count_lo: u32, // 12 - 16
  /// Free inode count.
  pub(crate) s_free_inodes_count: u32, // 16 - 20
  /// First data block. This must be at least 1 for 1k-block filesystems and is
  /// typically 0 for all other block sizes.
  pub(crate) s_first_data_block: u32, // 20 - 24
  /// Block size is 2 ^ (10 + s_log_block_size).
  pub(crate) s_log_block_size: u32, // 24 - 28
  /// Cluster size is 2 ^ (10 + s_log_cluster_size) blocks if bigalloc is
  /// enabled. Otherwise s_log_cluster_size must equal s_log_block_size.
  pub(crate) s_log_cluster_size: u32, // 28 - 32
  /// Blocks per group.
  pub(crate) s_blocks_per_group: u32, // 32 - 36
  /// Clusters per group, if bigalloc is enabled. Otherwise s_clusters_per_group
  /// must equal s_blocks_per_group.
  pub(crate) s_clusters_per_group: u32, // 36 - 40
  /// Inodes per group.
  pub(crate) s_inodes_per_group: u32, // 40 - 44
  /// Mount time, in seconds since the epoch.
  pub(crate) s_mtime: u32, // 44 - 48
  /// Write time, in seconds since the epoch.
  pub(crate) s_wtime: u32, // 48 - 52
  /// Number of mounts since the last fsck.
  pub(crate) s_mnt_count: u16, // 52 - 54
  /// Number of mounts beyond which a fsck is needed.
  pub(crate) s_max_mnt_count: u16, // 54 - 56
  /// Magic signature, 0xEF53
  pub(crate) s_magic: u16, // 56 - 58
  /// File system state. See super_state for more info.
  pub(crate) s_state: u16, // 58 - 60
  /// Behaviour when detecting errors. See super_errors for more info.
  pub(crate) s_errors: u16, // 60 - 62
  /// Minor revision level.
  pub(crate) s_minor_rev_level: u16, // 62 - 64
  /// Time of last check, in seconds since the epoch.
  pub(crate) s_lastcheck: u32, // 64 - 68
  /// Maximum time between checks, in seconds.
  pub(crate) s_checkinterval: u32, // 68 - 72
  /// Creator OS. See the table super_creator for more info.
  pub(crate) s_creator_os: u32, // 72 - 76
  /// Revision level. See the table super_revision for more info.
  pub(crate) s_rev_level: u32, // 76 - 80
  /// Default uid for reserved blocks.
  pub(crate) s_def_resuid: u16, // 80 - 82
  /// Default gid for reserved blocks.
  pub(crate) s_def_resgid: u16, // 82 - 84

  // These fields are for EXT4_DYNAMIC_REV superblocks only.

  // Note: the difference between the compatible feature set and the incompatible feature set is
  // that if there is a bit set in the incompatible feature set that the kernel doesn't know about,
  // it should refuse to mount the filesystem.

  // e2fsck's requirements are more strict; if it doesn't know about a feature in either the
  // compatible or incompatible feature set, it must abort and not try to meddle with things it
  // doesn't understand...
  /// First non-reserved inode.
  pub(crate) s_first_ino: u32, // 84 - 88
  /// Size of inode structure, in bytes.
  pub(crate) s_inode_size: u16, // 88 - 90
  /// Block group # of this superblock.
  pub(crate) s_block_group_nr: u16, // 90 - 92
  /// Compatible feature set flags. Kernel can still read/write this fs even if
  /// it doesn't understand a flag; fsck should not do that. See the
  /// super_compat table for more info.
  pub(crate) s_feature_compat: u32, // 92 - 96
  /// Incompatible feature set. If the kernel or fsck doesn't understand one of
  /// these bits, it should stop. See the super_incompat table for more info.
  pub(crate) s_feature_incompat: u32, // 96 - 100
  /// Readonly-compatible feature set. If the kernel doesn't understand one of
  /// these bits, it can still mount read-only. See the super_rocompat table for
  /// more info.
  pub(crate) s_feature_ro_compat: u32, // 100 - 104
  /// 128-bit UUID for volume.
  pub(crate) s_uuid: UuidRaw, // 104 - 120
  /// Volume label.
  pub(crate) s_volume_name: [u8; 16], // 120 - 136
  /// Directory where filesystem was last mounted.
  pub(crate) s_last_mounted: [u8; 64], // 136 - 200
  /// For compression (Not used in e2fsprogs/Linux)
  pub(crate) s_algorithm_usage_bitmap: u32, // 200 - 204

  // Performance hints. Directory preallocation should only happen if the
  // EXT4_FEATURE_COMPAT_DIR_PREALLOC flag is on.
  /// #. of blocks to try to preallocate for ... files? (Not used in
  /// e2fsprogs/Linux)
  pub(crate) s_prealloc_blocks: u8, // 204 - 205
  /// #. of blocks to preallocate for directories. (Not used in e2fsprogs/Linux)
  pub(crate) s_prealloc_dir_blocks: u8, // 205 - 206
  /// Number of reserved GDT entries for future filesystem expansion.
  pub(crate) s_reserved_gdt_blocks: u16, // 206 - 208

  // Journalling support is valid only if EXT4_FEATURE_COMPAT_HAS_JOURNAL is set.
  /// UUID of journal superblock
  pub(crate) s_journal_uuid: UuidRaw, // 208 - 224
  /// inode number of journal file.
  pub(crate) s_journal_inum: u32, // 224 - 228
  /// Device number of journal file, if the external journal feature flag is
  /// set.
  pub(crate) s_journal_dev: u32, // 228 - 232
  /// Start of list of orphaned inodes to delete.
  pub(crate) s_last_orphan: u32, // 232 - 236
  /// HTREE hash seed.
  pub(crate) s_hash_seed: UuidRaw, // 236 - 252
  /// Default hash algorithm to use for directory hashes. See super_def_hash for
  /// more info.
  pub(crate) s_def_hash_version: u8, // 252 - 253
  /// If this value is 0 or EXT3_JNL_BACKUP_BLOCKS (1), then the s_jnl_blocks
  /// field contains a duplicate copy of the inode's i_block[] array and i_size.
  pub(crate) s_jnl_backup_type: u8, // 253 - 254
  /// Size of group descriptors, in bytes, if the 64bit incompat feature flag is
  /// set.
  pub(crate) s_desc_size: u16, // 254 - 256
  /// Default mount options. See the super_mountopts table for more info.
  pub(crate) s_default_mount_opts: u32, // 256 - 260
  /// First metablock block group, if the meta_bg feature is enabled.
  pub(crate) s_first_meta_bg: u32, // 260 - 264
  /// When the filesystem was created, in seconds since the epoch.
  pub(crate) s_mkfs_time: u32, // 264 - 268
  /// Backup copy of the journal inode's i_block[] array in the first 15
  /// elements and i_size_high and i_size in the 16th and 17th elements,
  /// respectively.
  pub(crate) s_jnl_blocks: [u32; 17], // 268 - 336

  // 64bit support is valid only if EXT4_FEATURE_COMPAT_64BIT is set.
  /// High 32-bits of the block count.
  pub(crate) s_blocks_count_hi: u32, // 336 - 340
  /// High 32-bits of the reserved block count.
  pub(crate) s_r_blocks_count_hi: u32, // 340 - 344
  /// High 32-bits of the free block count.
  pub(crate) s_free_blocks_count_hi: u32, // 344 - 348
  /// All inodes have at least # bytes.
  pub(crate) s_min_extra_isize: u16, // 348 - 350
  /// New inodes should reserve # bytes.
  pub(crate) s_want_extra_isize: u16, // 350 - 352
  /// Miscellaneous flags. See the super_flags table for more info.
  pub(crate) s_flags: u32, // 352 - 356
  /// RAID stride. This is the number of logical blocks read from or written to
  /// the disk before moving to the next disk. This affects the placement of
  /// filesystem metadata, which will hopefully make RAID storage faster.
  pub(crate) s_raid_stride: u16, // 356 - 358
  /// #. seconds to wait in multi-mount prevention (MMP) checking. In theory,
  /// MMP is a mechanism to record in the superblock which host and device have
  /// mounted the filesystem, in order to prevent multiple mounts. This feature
  /// does not seem to be implemented...
  pub(crate) s_mmp_interval: u16, // 358 - 360
  /// Block # for multi-mount protection data.
  pub(crate) s_mmp_block: u64, // 360 - 368
  /// RAID stripe width. This is the number of logical blocks read from or
  /// written to the disk before coming back to the current disk. This is used
  /// by the block allocator to try to reduce the number of read-modify-write
  /// operations in a RAID5/6.
  pub(crate) s_raid_stripe_width: u32, // 368 - 372
  /// Size of a flexible block group is 2 ^ s_log_groups_per_flex.
  pub(crate) s_log_groups_per_flex: u8, // 372 - 373
  /// Metadata checksum algorithm type. The only valid value is 1 (crc32c).
  pub(crate) s_checksum_type: u8, // 373 - 374
  pub(crate) s_reserved_pad: u16, // 374 - 376
  /// Number of KiB written to this filesystem over its lifetime.
  pub(crate) s_kbytes_written: u64, // 376 - 384
  /// inode number of active snapshot. (Not used in e2fsprogs/Linux.)
  pub(crate) s_snapshot_inum: u32, // 384 - 388
  /// Sequential ID of active snapshot. (Not used in e2fsprogs/Linux.)
  pub(crate) s_snapshot_id: u32, // 388 - 392
  /// Number of blocks reserved for active snapshot's future use. (Not used in
  /// e2fsprogs/Linux.)
  pub(crate) s_snapshot_r_blocks_count: u64, // 392 - 400
  /// inode number of the head of the on-disk snapshot list. (Not used in
  /// e2fsprogs/Linux.)
  pub(crate) s_snapshot_list: u32, // 400 - 404
  /// Number of errors seen.
  pub(crate) s_error_count: u32, // 404 - 408
  /// First time an error happened, in seconds since the epoch.
  pub(crate) s_first_error_time: u32, // 408 - 412
  /// inode involved in first error.
  pub(crate) s_first_error_ino: u32, // 412 - 416
  /// Number of block involved of first error.
  pub(crate) s_first_error_block: u64, // 416 - 424
  /// Name of function where the error happened.
  pub(crate) s_first_error_func: [u8; 32], // 424 - 456
  /// Line number where error happened.
  pub(crate) s_first_error_line: u32, // 456 - 460
  /// Time of most recent error, in seconds since the epoch.
  pub(crate) s_last_error_time: u32, // 460 - 464
  /// inode involved in most recent error.
  pub(crate) s_last_error_ino: u32, // 464 - 468
  /// Line number where most recent error happened.
  pub(crate) s_last_error_line: u32, // 468 - 472
  /// Number of block involved in most recent error.
  pub(crate) s_last_error_block: u64, // 472 - 480
  /// Name of function where the most recent error happened.
  pub(crate) s_last_error_func: [u8; 32], // 480 - 512
  /// ASCIIZ string of mount options.
  pub(crate) s_mount_opts: [u8; 64], // 512 - 576
  /// Inode number of user quota file.
  pub(crate) s_usr_quota_inum: u32, // 576 - 580
  /// Inode number of group quota file.
  pub(crate) s_grp_quota_inum: u32, // 580 - 584
  /// Overhead blocks/clusters in fs. (Huh? This field is always zero, which
  /// means that the kernel calculates it dynamically.)
  pub(crate) s_overhead_blocks: u32, // 584 - 588
  /// Block groups containing superblock backups (if sparse_super2)
  pub(crate) s_backup_bgs: [u32; 2], // 588 - 596
  /// Encryption algorithms in use. There can be up to four algorithms in use at
  /// any time; valid algorithm codes are given in the super_encrypt table
  /// below.
  pub(crate) s_encrypt_algos: [u8; 4], // 596 - 600
  /// Salt for the string2key algorithm for encryption.
  pub(crate) s_encrypt_pw_salt: UuidRaw, // 600 - 616
  /// Inode number of lost+found
  pub(crate) s_lpf_ino: u32, // 616 - 620
  /// Inode that tracks project quotas.
  pub(crate) s_prj_quota_inum: u32, // 620 - 624
  /// Checksum seed used for metadata_csum calculations. This value is
  /// crc32c(~0, $orig_fs_uuid).
  pub(crate) s_checksum_seed: u32, // 624 - 628
  /// Upper 8 bits of the s_wtime field.
  pub(crate) s_wtime_hi: u8, // 628 - 629
  /// Upper 8 bits of the s_mtime field.
  pub(crate) s_mtime_hi: u8, // 629 - 630
  /// Upper 8 bits of the s_mkfs_time field.
  pub(crate) s_mkfs_time_hi: u8, // 630 - 631
  /// Upper 8 bits of the s_lastcheck_hi field.
  pub(crate) s_lastcheck_hi: u8, // 631 - 632
  /// Upper 8 bits of the s_first_error_time_hi field.
  pub(crate) s_first_error_time_hi: u8, // 632 - 633
  /// Upper 8 bits of the s_last_error_time_hi field.
  pub(crate) s_last_error_time_hi: u8, // 633 - 634
  /// Zero padding.
  pub(crate) s_pad: [u8; 2], // 634 - 636
  /// Filename charset encoding.
  pub(crate) s_encoding: u16, // 636 - 638
  /// Filename charset encoding flags.
  pub(crate) s_encoding_flags: u16, // 638 - 640
  /// Padding to the end of the block.
  pub(crate) s_reserved: [u32; 95], // 640 - 1020
  /// Superblock checksum.
  pub(crate) s_checksum: u32, // 1020 - 1024
}

impl SuperblockRaw
{
  pub(crate) const WIDTH: usize = 1024;
}

impl From<&[u8; Self::WIDTH]> for SuperblockRaw
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
    raw.s_inodes_count = u32::from_le(raw.s_inodes_count);
    raw.s_blocks_count_lo = u32::from_le(raw.s_blocks_count_lo);
    raw.s_r_blocks_count_lo = u32::from_le(raw.s_r_blocks_count_lo);
    raw.s_free_blocks_count_lo = u32::from_le(raw.s_free_blocks_count_lo);
    raw.s_free_inodes_count = u32::from_le(raw.s_free_inodes_count);
    raw.s_first_data_block = u32::from_le(raw.s_first_data_block);
    raw.s_log_block_size = u32::from_le(raw.s_log_block_size);
    raw.s_log_cluster_size = u32::from_le(raw.s_log_cluster_size);
    raw.s_blocks_per_group = u32::from_le(raw.s_blocks_per_group);
    raw.s_clusters_per_group = u32::from_le(raw.s_clusters_per_group);
    raw.s_inodes_per_group = u32::from_le(raw.s_inodes_per_group);
    raw.s_mtime = u32::from_le(raw.s_mtime);
    raw.s_wtime = u32::from_le(raw.s_wtime);
    raw.s_mnt_count = u16::from_le(raw.s_mnt_count);
    raw.s_max_mnt_count = u16::from_le(raw.s_max_mnt_count);
    raw.s_magic = u16::from_le(raw.s_magic);
    raw.s_state = u16::from_le(raw.s_state);
    raw.s_errors = u16::from_le(raw.s_errors);
    raw.s_minor_rev_level = u16::from_le(raw.s_minor_rev_level);
    raw.s_lastcheck = u32::from_le(raw.s_lastcheck);
    raw.s_checkinterval = u32::from_le(raw.s_checkinterval);
    raw.s_creator_os = u32::from_le(raw.s_creator_os);
    raw.s_rev_level = u32::from_le(raw.s_rev_level);
    raw.s_def_resuid = u16::from_le(raw.s_def_resuid);
    raw.s_def_resgid = u16::from_le(raw.s_def_resgid);
    raw.s_first_ino = u32::from_le(raw.s_first_ino);
    raw.s_inode_size = u16::from_le(raw.s_inode_size);
    raw.s_block_group_nr = u16::from_le(raw.s_block_group_nr);
    raw.s_feature_compat = u32::from_le(raw.s_feature_compat);
    raw.s_feature_incompat = u32::from_le(raw.s_feature_incompat);
    raw.s_feature_ro_compat = u32::from_le(raw.s_feature_ro_compat);
    raw.s_algorithm_usage_bitmap = u32::from_le(raw.s_algorithm_usage_bitmap);
    raw.s_reserved_gdt_blocks = u16::from_le(raw.s_reserved_gdt_blocks);
    raw.s_journal_inum = u32::from_le(raw.s_journal_inum);
    raw.s_journal_dev = u32::from_le(raw.s_journal_dev);
    raw.s_last_orphan = u32::from_le(raw.s_last_orphan);
    raw.s_desc_size = u16::from_le(raw.s_desc_size);
    raw.s_default_mount_opts = u32::from_le(raw.s_default_mount_opts);
    raw.s_first_meta_bg = u32::from_le(raw.s_first_meta_bg);
    raw.s_mkfs_time = u32::from_le(raw.s_mkfs_time);
    raw.s_jnl_blocks = [
      u32::from_le(raw.s_jnl_blocks[0]),
      u32::from_le(raw.s_jnl_blocks[1]),
      u32::from_le(raw.s_jnl_blocks[2]),
      u32::from_le(raw.s_jnl_blocks[3]),
      u32::from_le(raw.s_jnl_blocks[4]),
      u32::from_le(raw.s_jnl_blocks[5]),
      u32::from_le(raw.s_jnl_blocks[6]),
      u32::from_le(raw.s_jnl_blocks[7]),
      u32::from_le(raw.s_jnl_blocks[8]),
      u32::from_le(raw.s_jnl_blocks[9]),
      u32::from_le(raw.s_jnl_blocks[10]),
      u32::from_le(raw.s_jnl_blocks[11]),
      u32::from_le(raw.s_jnl_blocks[12]),
      u32::from_le(raw.s_jnl_blocks[13]),
      u32::from_le(raw.s_jnl_blocks[14]),
      u32::from_le(raw.s_jnl_blocks[15]),
      u32::from_le(raw.s_jnl_blocks[16]),
    ];
    raw.s_blocks_count_hi = u32::from_le(raw.s_blocks_count_hi);
    raw.s_r_blocks_count_hi = u32::from_le(raw.s_r_blocks_count_hi);
    raw.s_free_blocks_count_hi = u32::from_le(raw.s_free_blocks_count_hi);
    raw.s_min_extra_isize = u16::from_le(raw.s_min_extra_isize);
    raw.s_want_extra_isize = u16::from_le(raw.s_want_extra_isize);
    raw.s_flags = u32::from_le(raw.s_flags);
    raw.s_raid_stride = u16::from_le(raw.s_raid_stride);
    raw.s_mmp_interval = u16::from_le(raw.s_mmp_interval);
    raw.s_mmp_block = u64::from_le(raw.s_mmp_block);
    raw.s_raid_stripe_width = u32::from_le(raw.s_raid_stripe_width);
    raw.s_reserved_pad = u16::from_le(raw.s_reserved_pad);
    raw.s_kbytes_written = u64::from_le(raw.s_kbytes_written);
    raw.s_snapshot_inum = u32::from_le(raw.s_snapshot_inum);
    raw.s_snapshot_id = u32::from_le(raw.s_snapshot_id);
    raw.s_snapshot_r_blocks_count = u64::from_le(raw.s_snapshot_r_blocks_count);
    raw.s_snapshot_list = u32::from_le(raw.s_snapshot_list);
    raw.s_error_count = u32::from_le(raw.s_error_count);
    raw.s_first_error_time = u32::from_le(raw.s_first_error_time);
    raw.s_first_error_ino = u32::from_le(raw.s_first_error_ino);
    raw.s_first_error_block = u64::from_le(raw.s_first_error_block);
    raw.s_first_error_line = u32::from_le(raw.s_first_error_line);
    raw.s_last_error_time = u32::from_le(raw.s_last_error_time);
    raw.s_last_error_ino = u32::from_le(raw.s_last_error_ino);
    raw.s_last_error_line = u32::from_le(raw.s_last_error_line);
    raw.s_last_error_block = u64::from_le(raw.s_last_error_block);
    raw.s_usr_quota_inum = u32::from_le(raw.s_usr_quota_inum);
    raw.s_grp_quota_inum = u32::from_le(raw.s_grp_quota_inum);
    raw.s_overhead_blocks = u32::from_le(raw.s_overhead_blocks);
    raw.s_backup_bgs = [
      u32::from_le(raw.s_backup_bgs[0]),
      u32::from_le(raw.s_backup_bgs[1]),
    ];
    raw.s_lpf_ino = u32::from_le(raw.s_lpf_ino);
    raw.s_prj_quota_inum = u32::from_le(raw.s_prj_quota_inum);
    raw.s_checksum_seed = u32::from_le(raw.s_checksum_seed);
    raw.s_encoding = u16::from_le(raw.s_encoding);
    raw.s_encoding_flags = u16::from_le(raw.s_encoding_flags);
    raw.s_checksum = u32::from_le(raw.s_checksum);
    raw
  }
}
