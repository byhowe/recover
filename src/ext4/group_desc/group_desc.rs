use super::{Flags, GroupDescRaw32, GroupDescRaw64};
use crate::concat_lo_hi;
use std::io;

#[derive(Debug)]
pub struct GroupDesc
{
  /// Location of block bitmap.
  pub block_bitmap: u64,
  /// Location of inode bitmap.
  pub inode_bitmap: u64,
  /// Location of inode table.
  pub inode_table: u64,
  /// Free block count.
  pub free_blocks_count: u32,
  /// Free inode count.
  pub free_inodes_count: u32,
  /// Directory count.
  pub used_dirs_count: u32,
  /// Block group flags.
  pub flags: Flags,
  /// Location of snapshot exclusion bitmap.
  pub exclude_bitmap: u64,
  /// The block bitmap checksum.
  pub block_bitmap_csum: u32,
  /// The inode bitmap checksum.
  pub inode_bitmap_csum: u32,
  /// Unused inode count. If set, we needn't scan past the
  /// (sb.s_inodes_per_group - gdt.bg_itable_unused)th entry in the inode table
  /// for this group.
  pub itable_unused: u32,
  /// Group descriptor checksum; crc16(sb_uuid+group_num+bg_desc) if the
  /// RO_COMPAT_GDT_CSUM feature is set, or crc32c(sb_uuid+group_num+bg_desc) &
  /// 0xFFFF if the RO_COMPAT_METADATA_CSUM feature is set. The bg_checksum
  /// field in bg_desc is skipped when calculating crc16 checksum, and set to
  /// zero if crc32c checksum is used.
  pub checksum: u16,
}

impl GroupDesc
{
  pub const RAW_WIDTH32: usize = GroupDescRaw32::WIDTH;
  pub const RAW_WIDTH64: usize = GroupDescRaw64::WIDTH;

  pub fn new<R>(inner: &mut R, bit64: bool) -> Result<Self, Error>
  where
    R: io::Read,
  {
    if bit64 {
      let mut block: [u8; Self::RAW_WIDTH64] = [0; Self::RAW_WIDTH64];
      inner.read_exact(&mut block)?;
      Ok(GroupDescRaw64::from(&block).into())
    } else {
      let mut block: [u8; Self::RAW_WIDTH32] = [0; Self::RAW_WIDTH32];
      inner.read_exact(&mut block)?;
      Ok(GroupDescRaw32::from(&block).into())
    }
  }
}

impl From<GroupDescRaw32> for GroupDesc
{
  fn from(raw: GroupDescRaw32) -> Self
  {
    Self {
      block_bitmap: raw.bg_block_bitmap as u64,
      inode_bitmap: raw.bg_inode_bitmap as u64,
      inode_table: raw.bg_inode_table as u64,
      free_blocks_count: raw.bg_free_blocks_count as u32,
      free_inodes_count: raw.bg_free_inodes_count as u32,
      used_dirs_count: raw.bg_used_dirs_count as u32,
      flags: Flags::from_raw(raw.bg_flags),
      exclude_bitmap: raw.bg_exclude_bitmap as u64,
      block_bitmap_csum: raw.bg_block_bitmap_csum as u32,
      inode_bitmap_csum: raw.bg_inode_bitmap_csum as u32,
      itable_unused: raw.bg_itable_unused as u32,
      checksum: raw.bg_checksum,
    }
  }
}

impl From<GroupDescRaw64> for GroupDesc
{
  fn from(raw: GroupDescRaw64) -> Self
  {
    Self {
      block_bitmap: concat_lo_hi!(u64, raw.bg_block_bitmap_lo, raw.bg_block_bitmap_hi),
      inode_bitmap: concat_lo_hi!(u64, raw.bg_inode_bitmap_lo, raw.bg_inode_bitmap_hi),
      inode_table: concat_lo_hi!(u64, raw.bg_inode_table_lo, raw.bg_inode_table_hi),
      free_blocks_count: concat_lo_hi!(u32, raw.bg_free_blocks_count_lo, raw.bg_free_blocks_count_hi),
      free_inodes_count: concat_lo_hi!(u32, raw.bg_free_inodes_count_lo, raw.bg_free_inodes_count_hi),
      used_dirs_count: concat_lo_hi!(u32, raw.bg_used_dirs_count_lo, raw.bg_used_dirs_count_hi),
      flags: Flags::from_raw(raw.bg_flags),
      exclude_bitmap: concat_lo_hi!(u64, raw.bg_exclude_bitmap_lo, raw.bg_exclude_bitmap_hi),
      block_bitmap_csum: concat_lo_hi!(u32, raw.bg_block_bitmap_csum_lo, raw.bg_block_bitmap_csum_hi),
      inode_bitmap_csum: concat_lo_hi!(u32, raw.bg_inode_bitmap_csum_lo, raw.bg_inode_bitmap_csum_hi),
      itable_unused: concat_lo_hi!(u32, raw.bg_itable_unused_lo, raw.bg_itable_unused_hi),
      checksum: raw.bg_checksum,
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
      "Group descriptor error: {}",
      match self {
        Self::IO(error) => format!("An IO error occurred: {}", error),
      }
    )
  }
}
