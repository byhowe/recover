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
  pub feature_compat: FeatureCompat, // 92 - 96
  /// Incompatible feature set. If the kernel or fsck doesn’t understand one of
  /// these bits, it should stop.
  pub feature_incompat: FeatureIncompat, // 96 - 100
  /// Readonly-compatible feature set. If the kernel doesn’t understand one of
  /// these bits, it can still mount read-only.
  pub feature_ro_compat: ReadOnlyFeatureCompat, // 100 - 104
  /// 128-bit UUID for volume.
  pub uuid: u128, // 104 - 120
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
      feature_compat: FeatureCompat::from_raw(LittleEndian::read_u32(&block[92..96]))?,
      feature_incompat: FeatureIncompat::from_raw(LittleEndian::read_u32(&block[96..100]))?,
      feature_ro_compat: ReadOnlyFeatureCompat::from_raw(LittleEndian::read_u32(&block[100..104]))?,
      uuid: LittleEndian::read_u128(&block[104..120]),
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

impl std::fmt::Display for Superblock
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "inodes_count            {}\n\
       blocks_count_lo         {}\n\
       r_blocks_count_lo       {}\n\
       free_blocks_count_lo    {}\n\
       free_inodes_count       {}\n\
       first_data_block        {}\n\
       log_block_size          {}\n\
       log_cluster_size        {}\n\
       blocks_per_group        {}\n\
       clusters_per_group      {}\n\
       inodes_per_group        {}\n\
       mtime                   {}\n\
       wtime                   {}\n\
       mnt_count               {}\n\
       max_mnt_count           {}\n\
       magic                   {:#06X}\n\
       state                   {}\n\
       errors                  {}\n\
       minor_rev_level         {}\n\
       lastcheck               {}\n\
       checkinterval           {}\n\
       creator_os              {}\n\
       rev_level               {}\n\
       def_resuid              {}\n\
       def_resgid              {}\n\
       first_ino               {}\n\
       inode_size              {}\n\
       block_group_nr          {}\n\
       feature_compat          {}\n\
       feature_incompat        {}\n\
       feature_ro_compat       {}\n\
       uuid                    {:#X}\n\
       volume_name             {}\n\
       last_mounted            {}\n\
       algorithm_usage_bitmap  {}\n\
       prealloc_blocks         {}\n\
       prealloc_dir_blocks     {}\n\
       reserved_gdt_blocks     {}\n\
       journal_uuid            {:?}\n\
       journal_inum            {}\n\
       journal_dev             {}\n\
       last_orphan             {}\n\
       hash_seed               {:?}\n\
       def_hash_version        {}\n\
       jnl_backup_type         {}\n\
       desc_size               {}\n\
       default_mount_opts      {}\n\
       first_meta_bg           {}\n\
       mkfs_time               {}\n\
       jnl_blocks              {:?}\n\
       blocks_count_hi         {}\n\
       r_blocks_count_hi       {}\n\
       free_blocks_count_hi    {}\n\
       min_extra_isize         {}\n\
       want_extra_isize        {}\n\
       flags                   {}\n\
       raid_stride             {}\n\
       mmp_interval            {}\n\
       mmp_block               {}\n\
       raid_stripe_width       {}\n\
       log_groups_per_flex     {}\n\
       checksum_type           {}\n\
       reserved_pad            {}\n\
       kbytes_written          {}\n\
       snapshot_inum           {}\n\
       snapshot_id             {}\n\
       snapshot_r_blocks_count {}\n\
       snapshot_list           {}\n\
       error_count             {}\n\
       first_error_time        {}\n\
       first_error_ino         {}\n\
       first_error_block       {}\n\
       first_error_func        {:?}\n\
       first_error_line        {}\n\
       last_error_time         {}\n\
       last_error_ino          {}\n\
       last_error_line         {}\n\
       last_error_block        {}\n\
       last_error_func         {:?}\n\
       mount_opts              {:?}\n\
       usr_quota_inum          {}\n\
       grp_quota_inum          {}\n\
       overhead_blocks         {}\n\
       backup_bgs              {:?}\n\
       encrypt_algos           {:?}\n\
       encrypt_pw_salt         {:?}\n\
       lpf_ino                 {}\n\
       prj_quota_inum          {}\n\
       checksum_seed           {}\n\
       wtime_hi                {}\n\
       mtime_hi                {}\n\
       mkfs_time_hi            {}\n\
       lastcheck_hi            {}\n\
       first_error_time_hi     {}\n\
       last_error_time_hi      {}\n\
       pad                     {:?}\n\
       encoding                {}\n\
       encoding_flags          {}\n\
       checksum                {}",
      self.inodes_count,
      self.blocks_count_lo,
      self.r_blocks_count_lo,
      self.free_blocks_count_lo,
      self.free_inodes_count,
      self.first_data_block,
      self.log_block_size,
      self.log_cluster_size,
      self.blocks_per_group,
      self.clusters_per_group,
      self.inodes_per_group,
      self.mtime,
      self.wtime,
      self.mnt_count,
      self.max_mnt_count,
      self.magic,
      self.state,
      self.errors,
      self.minor_rev_level,
      self.lastcheck,
      self.checkinterval,
      self.creator_os,
      self.rev_level,
      self.def_resuid,
      self.def_resgid,
      self.first_ino,
      self.inode_size,
      self.block_group_nr,
      self.feature_compat,
      self.feature_incompat,
      self.feature_ro_compat,
      self.uuid,
      self.volume_name,
      self.last_mounted,
      self.algorithm_usage_bitmap,
      self.prealloc_blocks,
      self.prealloc_dir_blocks,
      self.reserved_gdt_blocks,
      self.journal_uuid,
      self.journal_inum,
      self.journal_dev,
      self.last_orphan,
      self.hash_seed,
      self.def_hash_version,
      self.jnl_backup_type,
      self.desc_size,
      self.default_mount_opts,
      self.first_meta_bg,
      self.mkfs_time,
      self.jnl_blocks,
      self.blocks_count_hi,
      self.r_blocks_count_hi,
      self.free_blocks_count_hi,
      self.min_extra_isize,
      self.want_extra_isize,
      self.flags,
      self.raid_stride,
      self.mmp_interval,
      self.mmp_block,
      self.raid_stripe_width,
      self.log_groups_per_flex,
      self.checksum_type,
      self.reserved_pad,
      self.kbytes_written,
      self.snapshot_inum,
      self.snapshot_id,
      self.snapshot_r_blocks_count,
      self.snapshot_list,
      self.error_count,
      self.first_error_time,
      self.first_error_ino,
      self.first_error_block,
      self.first_error_func,
      self.first_error_line,
      self.last_error_time,
      self.last_error_ino,
      self.last_error_line,
      self.last_error_block,
      self.last_error_func,
      self.mount_opts,
      self.usr_quota_inum,
      self.grp_quota_inum,
      self.overhead_blocks,
      self.backup_bgs,
      self.encrypt_algos,
      self.encrypt_pw_salt,
      self.lpf_ino,
      self.prj_quota_inum,
      self.checksum_seed,
      self.wtime_hi,
      self.mtime_hi,
      self.mkfs_time_hi,
      self.lastcheck_hi,
      self.first_error_time_hi,
      self.last_error_time_hi,
      self.pad,
      self.encoding,
      self.encoding_flags,
      self.checksum,
    )
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

impl std::fmt::Display for State
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let mut output = String::new();
    if self.cleanly_unmounted {
      output.push_str("clean, ");
    }
    if self.errors_detected {
      output.push_str("errors detected, ");
    }
    if self.orphans_being_recovered {
      output.push_str("orphans being recovered, ");
    }
    output.pop();
    output.pop();
    write!(f, "{}", output)
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

impl std::fmt::Display for ErrorPolicy
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Continue => "Continue",
        Self::RemountReadOnly => "Remount Read-only",
        Self::Panic => "Panic",
      }
    )
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

impl std::fmt::Display for RevisionLevel
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      match self {
        Self::Original => "0 (Original format)",
        Self::Dynamic => "1 (v2 format w/ dynamic inode sizes)",
      }
    )
  }
}

#[derive(Debug)]
pub struct FeatureCompat
{
  pub dir_prealloc: bool,
  pub imagic_inodes: bool,
  pub has_journal: bool,
  pub ext_attr: bool,
  pub resize_inode: bool,
  pub dir_index: bool,
  pub lazy_bg: bool,
  pub exclude_inode: bool,
  pub exclude_bitmap: bool,
  pub sparse_super2: bool,
}

impl FeatureCompat
{
  const DIR_PREALLOC: u32 = 0x1;
  const IMAGIC_INODE: u32 = 0x2;
  const HAS_JOURNAL: u32 = 0x4;
  const EXT_ATTR: u32 = 0x8;
  const RESIZE_INODE: u32 = 0x10;
  const DIR_INDEX: u32 = 0x20;
  const LAZY_BG: u32 = 0x40;
  const EXCLUDE_INODE: u32 = 0x80;
  const EXCLUDE_BITMAP: u32 = 0x100;
  const SPARSE_SUPER2: u32 = 0x200;

  fn from_raw(feature: u32) -> Result<Self, UnexpectedValue>
  {
    let feature_struct = Self {
      dir_prealloc: feature & Self::DIR_PREALLOC != 0,
      imagic_inodes: feature & Self::IMAGIC_INODE != 0,
      has_journal: feature & Self::HAS_JOURNAL != 0,
      ext_attr: feature & Self::EXT_ATTR != 0,
      resize_inode: feature & Self::RESIZE_INODE != 0,
      dir_index: feature & Self::DIR_INDEX != 0,
      lazy_bg: feature & Self::LAZY_BG != 0,
      exclude_inode: feature & Self::EXCLUDE_INODE != 0,
      exclude_bitmap: feature & Self::EXCLUDE_BITMAP != 0,
      sparse_super2: feature & Self::SPARSE_SUPER2 != 0,
    };

    if feature
      & !(Self::DIR_PREALLOC
        | Self::IMAGIC_INODE
        | Self::HAS_JOURNAL
        | Self::EXT_ATTR
        | Self::RESIZE_INODE
        | Self::DIR_INDEX
        | Self::LAZY_BG
        | Self::EXCLUDE_INODE
        | Self::EXCLUDE_BITMAP
        | Self::SPARSE_SUPER2)
      != 0
    {
      Err(UnexpectedValue::FeatureCompat(feature))
    } else {
      Ok(feature_struct)
    }
  }
}

impl std::fmt::Display for FeatureCompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let mut output = String::new();
    if self.dir_prealloc {
      output.push_str("dir_prealloc ");
    }
    if self.imagic_inodes {
      output.push_str("imagic_inodes ");
    }
    if self.has_journal {
      output.push_str("has_journal ");
    }
    if self.ext_attr {
      output.push_str("ext_attr ");
    }
    if self.resize_inode {
      output.push_str("resize_inode ");
    }
    if self.dir_index {
      output.push_str("dir_index ");
    }
    if self.lazy_bg {
      output.push_str("lazy_bg ");
    }
    if self.exclude_inode {
      output.push_str("exclude_inode ");
    }
    if self.exclude_bitmap {
      output.push_str("exclude_bitmap ");
    }
    if self.sparse_super2 {
      output.push_str("sparse_super2 ");
    }
    output.pop();
    write!(f, "{}", output)
  }
}

#[derive(Debug)]
pub struct FeatureIncompat
{
  pub compression: bool,
  pub filetype: bool,
  pub recover: bool,
  pub journal_dev: bool,
  pub meta_bg: bool,
  pub extents: bool,
  pub bit64: bool,
  pub mmp: bool,
  pub flex_bg: bool,
  pub ea_node: bool,
  pub dirdata: bool,
  pub csum_seed: bool,
  pub largedir: bool,
  pub inline_data: bool,
  pub encrypt: bool,
}

impl FeatureIncompat
{
  const COMPRESSION: u32 = 0x1;
  const FILETYPE: u32 = 0x2;
  const RECOVER: u32 = 0x4;
  const JOURNAL_DEV: u32 = 0x8;
  const META_BG: u32 = 0x10;
  const EXTENTS: u32 = 0x40;
  const BIT64: u32 = 0x80;
  const MMP: u32 = 0x100;
  const FLEX_BG: u32 = 0x200;
  const EA_NODE: u32 = 0x400;
  const DIRDATA: u32 = 0x1000;
  const CSUM_SEED: u32 = 0x2000;
  const LARGEDIR: u32 = 0x4000;
  const INLINE_DATA: u32 = 0x8000;
  const ENCRYPT: u32 = 0x10000;

  fn from_raw(feature: u32) -> Result<Self, UnexpectedValue>
  {
    let feature_struct = Self {
      compression: feature & Self::COMPRESSION != 0,
      filetype: feature & Self::FILETYPE != 0,
      recover: feature & Self::RECOVER != 0,
      journal_dev: feature & Self::JOURNAL_DEV != 0,
      meta_bg: feature & Self::META_BG != 0,
      extents: feature & Self::EXTENTS != 0,
      bit64: feature & Self::BIT64 != 0,
      mmp: feature & Self::MMP != 0,
      flex_bg: feature & Self::FLEX_BG != 0,
      ea_node: feature & Self::EA_NODE != 0,
      dirdata: feature & Self::DIRDATA != 0,
      csum_seed: feature & Self::CSUM_SEED != 0,
      largedir: feature & Self::LARGEDIR != 0,
      inline_data: feature & Self::INLINE_DATA != 0,
      encrypt: feature & Self::ENCRYPT != 0,
    };

    if feature
      & !(Self::COMPRESSION
        | Self::FILETYPE
        | Self::RECOVER
        | Self::JOURNAL_DEV
        | Self::META_BG
        | Self::EXTENTS
        | Self::BIT64
        | Self::MMP
        | Self::FLEX_BG
        | Self::EA_NODE
        | Self::DIRDATA
        | Self::CSUM_SEED
        | Self::LARGEDIR
        | Self::INLINE_DATA
        | Self::ENCRYPT)
      != 0
    {
      Err(UnexpectedValue::FeatureIncompat(feature))
    } else {
      Ok(feature_struct)
    }
  }
}

impl std::fmt::Display for FeatureIncompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let mut output = String::new();
    if self.compression {
      output.push_str("compression ");
    }
    if self.filetype {
      output.push_str("filetype ");
    }
    if self.recover {
      output.push_str("recover ");
    }
    if self.journal_dev {
      output.push_str("journal_dev ");
    }
    if self.meta_bg {
      output.push_str("meta_bg ");
    }
    if self.extents {
      output.push_str("extents ");
    }
    if self.bit64 {
      output.push_str("64bit ");
    }
    if self.mmp {
      output.push_str("mmp ");
    }
    if self.flex_bg {
      output.push_str("flex_bg ");
    }
    if self.ea_node {
      output.push_str("ea_node ");
    }
    if self.dirdata {
      output.push_str("dirdata ");
    }
    if self.csum_seed {
      output.push_str("csum_seed ");
    }
    if self.largedir {
      output.push_str("largedir ");
    }
    if self.inline_data {
      output.push_str("inline_data ");
    }
    if self.encrypt {
      output.push_str("encrypt ");
    }
    output.pop();
    write!(f, "{}", output)
  }
}

#[derive(Debug)]
pub struct ReadOnlyFeatureCompat
{
  pub sparse_super: bool,
  pub large_file: bool,
  pub btree_dir: bool,
  pub huge_file: bool,
  pub gdt_csum: bool,
  pub dir_nlink: bool,
  pub extra_isize: bool,
  pub has_snapshot: bool,
  pub quota: bool,
  pub bigalloc: bool,
  pub metadata_csum: bool,
  pub replica: bool,
  pub readonly: bool,
  pub project: bool,
  pub verity: bool,
}

impl ReadOnlyFeatureCompat
{
  const SPARSE_SUPER: u32 = 0x1;
  const LARGE_FILE: u32 = 0x2;
  const BTREE_DIR: u32 = 0x4;
  const HUGE_FILE: u32 = 0x8;
  const GDT_CSUM: u32 = 0x10;
  const DIR_NLINK: u32 = 0x20;
  const EXTRA_ISIZE: u32 = 0x40;
  const HAS_SNAPSHOT: u32 = 0x80;
  const QUOTA: u32 = 0x100;
  const BIGALLOC: u32 = 0x200;
  const METADATA_CSUM: u32 = 0x400;
  const REPLICA: u32 = 0x800;
  const READONLY: u32 = 0x1000;
  const PROJECT: u32 = 0x2000;
  const VERITY: u32 = 0x8000;

  fn from_raw(feature: u32) -> Result<Self, UnexpectedValue>
  {
    let feature_struct = Self {
      sparse_super: feature & Self::SPARSE_SUPER != 0,
      large_file: feature & Self::LARGE_FILE != 0,
      btree_dir: feature & Self::BTREE_DIR != 0,
      huge_file: feature & Self::HUGE_FILE != 0,
      gdt_csum: feature & Self::GDT_CSUM != 0,
      dir_nlink: feature & Self::DIR_NLINK != 0,
      extra_isize: feature & Self::EXTRA_ISIZE != 0,
      has_snapshot: feature & Self::HAS_SNAPSHOT != 0,
      quota: feature & Self::QUOTA != 0,
      bigalloc: feature & Self::BIGALLOC != 0,
      metadata_csum: feature & Self::METADATA_CSUM != 0,
      replica: feature & Self::REPLICA != 0,
      readonly: feature & Self::READONLY != 0,
      project: feature & Self::PROJECT != 0,
      verity: feature & Self::VERITY != 0,
    };

    if feature
      & !(Self::SPARSE_SUPER
        | Self::LARGE_FILE
        | Self::BTREE_DIR
        | Self::HUGE_FILE
        | Self::GDT_CSUM
        | Self::DIR_NLINK
        | Self::EXTRA_ISIZE
        | Self::HAS_SNAPSHOT
        | Self::QUOTA
        | Self::BIGALLOC
        | Self::METADATA_CSUM
        | Self::REPLICA
        | Self::READONLY
        | Self::PROJECT
        | Self::VERITY)
      != 0
    {
      Err(UnexpectedValue::ReadOnlyFeatureCompat(feature))
    } else {
      Ok(feature_struct)
    }
  }
}

impl std::fmt::Display for ReadOnlyFeatureCompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let mut output = String::new();
    if self.sparse_super {
      output.push_str("sparse_super ");
    }
    if self.large_file {
      output.push_str("large_file ");
    }
    if self.btree_dir {
      output.push_str("btree_dir ");
    }
    if self.huge_file {
      output.push_str("huge_file ");
    }
    if self.gdt_csum {
      output.push_str("gdt_csum ");
    }
    if self.dir_nlink {
      output.push_str("dir_nlink ");
    }
    if self.extra_isize {
      output.push_str("extra_isize ");
    }
    if self.has_snapshot {
      output.push_str("has_snapshot ");
    }
    if self.quota {
      output.push_str("quota ");
    }
    if self.bigalloc {
      output.push_str("bigalloc ");
    }
    if self.metadata_csum {
      output.push_str("metadata_csum ");
    }
    if self.replica {
      output.push_str("replica ");
    }
    if self.readonly {
      output.push_str("readonly ");
    }
    if self.project {
      output.push_str("project ");
    }
    if self.verity {
      output.push_str("verity ");
    }
    output.pop();
    write!(f, "{}", output)
  }
}

#[derive(Debug)]
pub enum UnexpectedValue
{
  State(u16),
  ErrorPolicy(u16),
  Creator(u32),
  RevisionLevel(u32),
  FeatureCompat(u32),
  FeatureIncompat(u32),
  ReadOnlyFeatureCompat(u32),
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
        Self::FeatureCompat(feature) =>
          format!("Unknown feature compat flag value: {:#034b}", feature),
        Self::FeatureIncompat(feature) =>
          format!("Unknown feature incompat flag value: {:#034b}", feature),
        Self::ReadOnlyFeatureCompat(feature) => format!(
          "Unknown read-only feature compat flag value: {:#034b}",
          feature
        ),
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
