#[repr(C)]
#[derive(Debug)]
pub(crate) struct GroupDescRaw32
{
  /// Location of block bitmap.
  pub(crate) bg_block_bitmap: u32, // 0 - 4
  /// Location of inode bitmap.
  pub(crate) bg_inode_bitmap: u32, // 4 - 8
  /// Location of inode table.
  pub(crate) bg_inode_table: u32, // 8 - 12
  /// Free block count.
  pub(crate) bg_free_blocks_count: u16, // 12 - 14
  /// Free inode count.
  pub(crate) bg_free_inodes_count: u16, // 14 - 16
  /// Directory count.
  pub(crate) bg_used_dirs_count: u16, // 16 - 18
  /// Block group flags. See the bgflags table below.
  pub(crate) bg_flags: u16, // 18 - 20
  /// Location of snapshot exclusion bitmap.
  pub(crate) bg_exclude_bitmap: u32, // 20 - 24
  /// The block bitmap checksum.
  pub(crate) bg_block_bitmap_csum: u16, // 24 - 26
  /// The inode bitmap checksum.
  pub(crate) bg_inode_bitmap_csum: u16, // 26 - 28
  /// Unused inode count. If set, we needn't scan past the
  /// (sb.s_inodes_per_group - gdt.bg_itable_unused)th entry in the inode table
  /// for this group.
  pub(crate) bg_itable_unused: u16, // 28 - 30
  /// Group descriptor checksum; crc16(sb_uuid+group_num+bg_desc) if the
  /// RO_COMPAT_GDT_CSUM feature is set, or crc32c(sb_uuid+group_num+bg_desc) &
  /// 0xFFFF if the RO_COMPAT_METADATA_CSUM feature is set. The bg_checksum
  /// field in bg_desc is skipped when calculating crc16 checksum, and set to
  /// zero if crc32c checksum is used.
  pub(crate) bg_checksum: u16, // 30 - 32
}

impl GroupDescRaw32
{
  pub(crate) const WIDTH: usize = 32;
}

impl From<&[u8; Self::WIDTH]> for GroupDescRaw32
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
    raw.bg_block_bitmap = u32::from_le(raw.bg_block_bitmap);
    raw.bg_inode_bitmap = u32::from_le(raw.bg_inode_bitmap);
    raw.bg_inode_table = u32::from_le(raw.bg_inode_table);
    raw.bg_free_blocks_count = u16::from_le(raw.bg_free_blocks_count);
    raw.bg_free_inodes_count = u16::from_le(raw.bg_free_inodes_count);
    raw.bg_used_dirs_count = u16::from_le(raw.bg_used_dirs_count);
    raw.bg_flags = u16::from_le(raw.bg_flags);
    raw.bg_exclude_bitmap = u32::from_le(raw.bg_exclude_bitmap);
    raw.bg_block_bitmap_csum = u16::from_le(raw.bg_block_bitmap_csum);
    raw.bg_inode_bitmap_csum = u16::from_le(raw.bg_inode_bitmap_csum);
    raw.bg_itable_unused = u16::from_le(raw.bg_itable_unused);
    raw.bg_checksum = u16::from_le(raw.bg_checksum);
    raw
  }
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct GroupDescRaw64
{
  /// Lower 32-bits of location of block bitmap.
  pub(crate) bg_block_bitmap_lo: u32, // 0 - 4
  /// Lower 32-bits of location of inode bitmap.
  pub(crate) bg_inode_bitmap_lo: u32, // 4 - 8
  /// Lower 32-bits of location of inode table.
  pub(crate) bg_inode_table_lo: u32, // 8 - 12
  /// Lower 16-bits of free block count.
  pub(crate) bg_free_blocks_count_lo: u16, // 12 - 14
  /// Lower 16-bits of free inode count.
  pub(crate) bg_free_inodes_count_lo: u16, // 14 - 16
  /// Lower 16-bits of directory count.
  pub(crate) bg_used_dirs_count_lo: u16, // 16 - 18
  /// Block group flags. See the bgflags table below.
  pub(crate) bg_flags: u16, // 18 - 20
  /// Lower 32-bits of location of snapshot exclusion bitmap.
  pub(crate) bg_exclude_bitmap_lo: u32, // 20 - 24
  /// Lower 16-bits of the block bitmap checksum.
  pub(crate) bg_block_bitmap_csum_lo: u16, // 24 - 26
  /// Lower 16-bits of the inode bitmap checksum.
  pub(crate) bg_inode_bitmap_csum_lo: u16, // 26 - 28
  /// Lower 16-bits of unused inode count. If set, we needn't scan past the
  /// (sb.s_inodes_per_group - gdt.bg_itable_unused)th entry in the inode table
  /// for this group.
  pub(crate) bg_itable_unused_lo: u16, // 28 - 30
  /// Group descriptor checksum; crc16(sb_uuid+group_num+bg_desc) if the
  /// RO_COMPAT_GDT_CSUM feature is set, or crc32c(sb_uuid+group_num+bg_desc) &
  /// 0xFFFF if the RO_COMPAT_METADATA_CSUM feature is set. The bg_checksum
  /// field in bg_desc is skipped when calculating crc16 checksum, and set to
  /// zero if crc32c checksum is used.
  pub(crate) bg_checksum: u16, // 30 - 32

  // These fields only exist if the 64bit feature is enabled and s_desc_size > 32.
  /// Upper 32-bits of location of block bitmap.
  pub(crate) bg_block_bitmap_hi: u32, // 32 - 36
  /// Upper 32-bits of location of inodes bitmap.
  pub(crate) bg_inode_bitmap_hi: u32, // 36 - 40
  /// Upper 32-bits of location of inodes table.
  pub(crate) bg_inode_table_hi: u32, // 40 - 44
  /// Upper 16-bits of free block count.
  pub(crate) bg_free_blocks_count_hi: u16, // 44 - 46
  /// Upper 16-bits of free inode count.
  pub(crate) bg_free_inodes_count_hi: u16, // 46 - 48
  /// Upper 16-bits of directory count.
  pub(crate) bg_used_dirs_count_hi: u16, // 48 - 50
  /// Upper 16-bits of unused inode count.
  pub(crate) bg_itable_unused_hi: u16, // 50 - 52
  /// Upper 32-bits of location of snapshot exclusion bitmap.
  pub(crate) bg_exclude_bitmap_hi: u32, // 52 - 56
  /// Upper 16-bits of the block bitmap checksum.
  pub(crate) bg_block_bitmap_csum_hi: u16, // 56 - 58
  /// Upper 16-bits of the inode bitmap checksum.
  pub(crate) bg_inode_bitmap_csum_hi: u16, // 58 - 60
  /// Padding to 64 bytes.
  pub(crate) bg_reserved: u32, // 60 - 64
}

impl GroupDescRaw64
{
  pub(crate) const WIDTH: usize = 64;
}

impl From<&[u8; Self::WIDTH]> for GroupDescRaw64
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
    raw.bg_block_bitmap_lo = u32::from_le(raw.bg_block_bitmap_lo);
    raw.bg_inode_bitmap_lo = u32::from_le(raw.bg_inode_bitmap_lo);
    raw.bg_inode_table_lo = u32::from_le(raw.bg_inode_table_lo);
    raw.bg_free_blocks_count_lo = u16::from_le(raw.bg_free_blocks_count_lo);
    raw.bg_free_inodes_count_lo = u16::from_le(raw.bg_free_inodes_count_lo);
    raw.bg_used_dirs_count_lo = u16::from_le(raw.bg_used_dirs_count_lo);
    raw.bg_flags = u16::from_le(raw.bg_flags);
    raw.bg_exclude_bitmap_lo = u32::from_le(raw.bg_exclude_bitmap_lo);
    raw.bg_block_bitmap_csum_lo = u16::from_le(raw.bg_block_bitmap_csum_lo);
    raw.bg_inode_bitmap_csum_lo = u16::from_le(raw.bg_inode_bitmap_csum_lo);
    raw.bg_itable_unused_lo = u16::from_le(raw.bg_itable_unused_lo);
    raw.bg_checksum = u16::from_le(raw.bg_checksum);
    raw.bg_block_bitmap_hi = u32::from_le(raw.bg_block_bitmap_hi);
    raw.bg_inode_bitmap_hi = u32::from_le(raw.bg_inode_bitmap_hi);
    raw.bg_inode_table_hi = u32::from_le(raw.bg_inode_table_hi);
    raw.bg_free_blocks_count_hi = u16::from_le(raw.bg_free_blocks_count_hi);
    raw.bg_free_inodes_count_hi = u16::from_le(raw.bg_free_inodes_count_hi);
    raw.bg_used_dirs_count_hi = u16::from_le(raw.bg_used_dirs_count_hi);
    raw.bg_itable_unused_hi = u16::from_le(raw.bg_itable_unused_hi);
    raw.bg_exclude_bitmap_hi = u32::from_le(raw.bg_exclude_bitmap_hi);
    raw.bg_block_bitmap_csum_hi = u16::from_le(raw.bg_block_bitmap_csum_hi);
    raw.bg_inode_bitmap_csum_hi = u16::from_le(raw.bg_inode_bitmap_csum_hi);
    raw
  }
}
