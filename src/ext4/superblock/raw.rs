use crate::uuid::UuidRaw;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct SuperblockRaw
{
  pub(crate) inodes_count: u32,
  pub(crate) blocks_count_lo: u32,
  pub(crate) r_blocks_count_lo: u32,
  pub(crate) free_blocks_count_lo: u32,
  pub(crate) free_inodes_count: u32,
  pub(crate) first_data_block: u32,
  pub(crate) log_block_size: u32,
  pub(crate) log_cluster_size: u32,
  pub(crate) blocks_per_group: u32,
  pub(crate) clusters_per_group: u32,
  pub(crate) inodes_per_group: u32,
  pub(crate) mtime: u32,
  pub(crate) wtime: u32,
  pub(crate) mnt_count: u16,
  pub(crate) max_mnt_count: u16,
  pub(crate) magic: u16,
  pub(crate) state: u16,
  pub(crate) errors: u16,
  pub(crate) minor_rev_level: u16,
  pub(crate) lastcheck: u32,
  pub(crate) checkinterval: u32,
  pub(crate) creator_os: u32,
  pub(crate) rev_level: u32,
  pub(crate) def_resuid: u16,
  pub(crate) def_resgid: u16,
  pub(crate) first_ino: u32,
  pub(crate) inode_size: u16,
  pub(crate) block_group_nr: u16,
  pub(crate) feature_compat: u32,
  pub(crate) feature_incompat: u32,
  pub(crate) feature_ro_compat: u32,
  pub(crate) uuid: UuidRaw,
  pub(crate) volume_name: [u8; 16],
  pub(crate) last_mounted: [u8; 64],
  pub(crate) algorithm_usage_bitmap: u32,
  pub(crate) prealloc_blocks: u8,
  pub(crate) prealloc_dir_blocks: u8,
  pub(crate) reserved_gdt_blocks: u16,
  pub(crate) journal_uuid: UuidRaw,
  pub(crate) journal_inum: u32,
  pub(crate) journal_dev: u32,
  pub(crate) last_orphan: u32,
  pub(crate) hash_seed: UuidRaw,
  pub(crate) def_hash_version: u8,
  pub(crate) jnl_backup_type: u8,
  pub(crate) desc_size: u16,
  pub(crate) default_mount_opts: u32,
  pub(crate) first_meta_bg: u32,
  pub(crate) mkfs_time: u32,
  pub(crate) jnl_blocks: [u32; 17],
  pub(crate) blocks_count_hi: u32,
  pub(crate) r_blocks_count_hi: u32,
  pub(crate) free_blocks_count_hi: u32,
  pub(crate) min_extra_isize: u16,
  pub(crate) want_extra_isize: u16,
  pub(crate) flags: u32,
  pub(crate) raid_stride: u16,
  pub(crate) mmp_interval: u16,
  pub(crate) mmp_block: u64,
  pub(crate) raid_stripe_width: u32,
  pub(crate) log_groups_per_flex: u8,
  pub(crate) checksum_type: u8,
  pub(crate) reserved_pad: u16,
  pub(crate) kbytes_written: u64,
  pub(crate) snapshot_inum: u32,
  pub(crate) snapshot_id: u32,
  pub(crate) snapshot_r_blocks_count: u64,
  pub(crate) snapshot_list: u32,
  pub(crate) error_count: u32,
  pub(crate) first_error_time: u32,
  pub(crate) first_error_ino: u32,
  pub(crate) first_error_block: u64,
  pub(crate) first_error_func: [u8; 32],
  pub(crate) first_error_line: u32,
  pub(crate) last_error_time: u32,
  pub(crate) last_error_ino: u32,
  pub(crate) last_error_line: u32,
  pub(crate) last_error_block: u64,
  pub(crate) last_error_func: [u8; 32],
  pub(crate) mount_opts: [u8; 64],
  pub(crate) usr_quota_inum: u32,
  pub(crate) grp_quota_inum: u32,
  pub(crate) overhead_blocks: u32,
  pub(crate) backup_bgs: [u32; 2],
  pub(crate) encrypt_algos: [u8; 4],
  pub(crate) encrypt_pw_salt: UuidRaw,
  pub(crate) lpf_ino: u32,
  pub(crate) prj_quota_inum: u32,
  pub(crate) checksum_seed: u32,
  pub(crate) wtime_hi: u8,
  pub(crate) mtime_hi: u8,
  pub(crate) mkfs_time_hi: u8,
  pub(crate) lastcheck_hi: u8,
  pub(crate) first_error_time_hi: u8,
  pub(crate) last_error_time_hi: u8,
  pub(crate) pad: [u8; 2],
  pub(crate) encoding: u16,
  pub(crate) encoding_flags: u16,
  pub(crate) reserved: [u32; 95],
  pub(crate) checksum: u32,
}

impl From<[u8; 1024]> for SuperblockRaw
{
  #[cfg(target_endian = "little")]
  fn from(block: [u8; 1024]) -> Self
  {
    unsafe { std::mem::transmute(block) }
  }

  #[cfg(target_endian = "big")]
  fn from(block: [u8; 1024]) -> Self
  {
    let mut raw: SuperblockRaw = unsafe { std::mem::transmute(block) };
    raw.inodes_count = u32::from_le(raw.inodes_count);
    raw.blocks_count_lo = u32::from_le(raw.blocks_count_lo);
    raw.r_blocks_count_lo = u32::from_le(raw.r_blocks_count_lo);
    raw.free_blocks_count_lo = u32::from_le(raw.free_blocks_count_lo);
    raw.free_inodes_count = u32::from_le(raw.free_inodes_count);
    raw.first_data_block = u32::from_le(raw.first_data_block);
    raw.log_block_size = u32::from_le(raw.log_block_size);
    raw.log_cluster_size = u32::from_le(raw.log_cluster_size);
    raw.blocks_per_group = u32::from_le(raw.blocks_per_group);
    raw.clusters_per_group = u32::from_le(raw.clusters_per_group);
    raw.inodes_per_group = u32::from_le(raw.inodes_per_group);
    raw.mtime = u32::from_le(raw.mtime);
    raw.wtime = u32::from_le(raw.wtime);
    raw.mnt_count = u16::from_le(raw.mnt_count);
    raw.max_mnt_count = u16::from_le(raw.max_mnt_count);
    raw.magic = u16::from_le(raw.magic);
    raw.state = u16::from_le(raw.state);
    raw.errors = u16::from_le(raw.errors);
    raw.minor_rev_level = u16::from_le(raw.minor_rev_level);
    raw.lastcheck = u32::from_le(raw.lastcheck);
    raw.checkinterval = u32::from_le(raw.checkinterval);
    raw.creator_os = u32::from_le(raw.creator_os);
    raw.rev_level = u32::from_le(raw.rev_level);
    raw.def_resuid = u16::from_le(raw.def_resuid);
    raw.def_resgid = u16::from_le(raw.def_resgid);
    raw.first_ino = u32::from_le(raw.first_ino);
    raw.inode_size = u16::from_le(raw.inode_size);
    raw.block_group_nr = u16::from_le(raw.block_group_nr);
    raw.feature_compat = u32::from_le(raw.feature_compat);
    raw.feature_incompat = u32::from_le(raw.feature_incompat);
    raw.feature_ro_compat = u32::from_le(raw.feature_ro_compat);
    raw.algorithm_usage_bitmap = u32::from_le(raw.algorithm_usage_bitmap);
    raw.reserved_gdt_blocks = u16::from_le(raw.reserved_gdt_blocks);
    raw.journal_inum = u32::from_le(raw.journal_inum);
    raw.journal_dev = u32::from_le(raw.journal_dev);
    raw.last_orphan = u32::from_le(raw.last_orphan);
    raw.hash_seed = [
      u32::from_le(raw.hash_seed[0]),
      u32::from_le(raw.hash_seed[1]),
      u32::from_le(raw.hash_seed[2]),
      u32::from_le(raw.hash_seed[3]),
    ];
    raw.desc_size = u16::from_le(raw.desc_size);
    raw.default_mount_opts = u32::from_le(raw.default_mount_opts);
    raw.first_meta_bg = u32::from_le(raw.first_meta_bg);
    raw.mkfs_time = u32::from_le(raw.mkfs_time);
    raw.jnl_blocks = [
      u32::from_le(raw.jnl_blocks[0]),
      u32::from_le(raw.jnl_blocks[1]),
      u32::from_le(raw.jnl_blocks[2]),
      u32::from_le(raw.jnl_blocks[3]),
      u32::from_le(raw.jnl_blocks[4]),
      u32::from_le(raw.jnl_blocks[5]),
      u32::from_le(raw.jnl_blocks[6]),
      u32::from_le(raw.jnl_blocks[7]),
      u32::from_le(raw.jnl_blocks[8]),
      u32::from_le(raw.jnl_blocks[9]),
      u32::from_le(raw.jnl_blocks[10]),
      u32::from_le(raw.jnl_blocks[11]),
      u32::from_le(raw.jnl_blocks[12]),
      u32::from_le(raw.jnl_blocks[13]),
      u32::from_le(raw.jnl_blocks[14]),
      u32::from_le(raw.jnl_blocks[15]),
      u32::from_le(raw.jnl_blocks[16]),
    ];
    raw.blocks_count_hi = u32::from_le(raw.blocks_count_hi);
    raw.r_blocks_count_hi = u32::from_le(raw.r_blocks_count_hi);
    raw.free_blocks_count_hi = u32::from_le(raw.free_blocks_count_hi);
    raw.min_extra_isize = u16::from_le(raw.min_extra_isize);
    raw.want_extra_isize = u16::from_le(raw.want_extra_isize);
    raw.flags = u32::from_le(raw.flags);
    raw.raid_stride = u16::from_le(raw.raid_stride);
    raw.mmp_interval = u16::from_le(raw.mmp_interval);
    raw.mmp_block = u64::from_le(raw.mmp_block);
    raw.raid_stripe_width = u32::from_le(raw.raid_stripe_width);
    raw.reserved_pad = u16::from_le(raw.reserved_pad);
    raw.kbytes_written = u64::from_le(raw.kbytes_written);
    raw.snapshot_inum = u32::from_le(raw.snapshot_inum);
    raw.snapshot_id = u32::from_le(raw.snapshot_id);
    raw.snapshot_r_blocks_count = u64::from_le(raw.snapshot_r_blocks_count);
    raw.snapshot_list = u32::from_le(raw.snapshot_list);
    raw.error_count = u32::from_le(raw.error_count);
    raw.first_error_time = u32::from_le(raw.first_error_time);
    raw.first_error_ino = u32::from_le(raw.first_error_ino);
    raw.first_error_block = u64::from_le(raw.first_error_block);
    raw.first_error_line = u32::from_le(raw.first_error_line);
    raw.last_error_time = u32::from_le(raw.last_error_time);
    raw.last_error_ino = u32::from_le(raw.last_error_ino);
    raw.last_error_line = u32::from_le(raw.last_error_line);
    raw.last_error_block = u64::from_le(raw.last_error_block);
    raw.usr_quota_inum = u32::from_le(raw.usr_quota_inum);
    raw.grp_quota_inum = u32::from_le(raw.grp_quota_inum);
    raw.overhead_blocks = u32::from_le(raw.overhead_blocks);
    raw.backup_bgs = [
      u32::from_le(raw.backup_bgs[0]),
      u32::from_le(raw.backup_bgs[1]),
    ];
    raw.lpf_ino = u32::from_le(raw.lpf_ino);
    raw.prj_quota_inum = u32::from_le(raw.prj_quota_inum);
    raw.checksum_seed = u32::from_le(raw.checksum_seed);
    raw.encoding = u16::from_le(raw.encoding);
    raw.encoding_flags = u16::from_le(raw.encoding_flags);
    raw.checksum = u32::from_le(raw.checksum);
    raw
  }
}
