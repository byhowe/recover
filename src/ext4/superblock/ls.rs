use super::{HashVersion, RevisionLevel, Superblock};
use chrono::{TimeZone, Utc};

impl std::fmt::Display for Superblock
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let inode_blocks_per_group: u32 =
      ((self.inodes_per_group * self.get_inode_size() as u32) + self.get_block_size() - 1)
        / self.get_block_size();

    write!(
      f,
      "Filesystem volume name:   {}\n\
       Last mounted on:          {}\n\
       Filesystem UUID:          {}\n\
       Filesystem magic number:  {:#X}\n\
       Filesystem revision #:    {}\n\
       Filesystem features:      {}\n\
       Filesystem flags:         {}\n\
       Default mount options:    {}\n\
       Mount options:            {}\n\
       Filesystem state:         {}\n\
       Errors behaviour:         {}\n\
       Filesystem OS type:       {}\n\
       Inode count:              {}\n\
       Block count:              {}\n\
       Reserved block count:     {}\n",
      crate::util::get_string(&self.volume_name),
      crate::util::get_string(&self.last_mounted),
      self.uuid,
      self.magic,
      self.rev_level,
      crate::util::get_string_list(&self.get_features()),
      self.flags,
      self.default_mount_opts,
      crate::util::get_string(&self.mount_opts),
      self.state,
      self.errors,
      self.creator_os,
      self.inodes_count,
      self.get_blocks_count(),
      self.get_reserved_blocks_count()
    )?;

    if self.overhead_blocks != 0 {
      write!(f, "Overhead clusters:        {}\n", self.overhead_blocks)?;
    }

    write!(
      f,
      "Free blocks:              {}\n\
       Free inodes:              {}\n\
       First block:              {}\n\
       Block size:               {}\n\
       ",
      self.get_free_blocks_count(),
      self.free_inodes_count,
      self.first_data_block,
      self.get_block_size()
    )?;

    if self.feature_ro_compat.bigalloc {
      write!(f, "Cluster size:             {}\n", self.get_cluster_size())?;
    } else {
      write!(f, "Fragment size:            {}\n", self.get_cluster_size())?;
    }

    if self.feature_incompat.bit64 {
      write!(f, "Group descriptor size:    {}\n", self.desc_size)?;
    }

    if self.reserved_gdt_blocks != 0 {
      write!(
        f,
        "Reserved GDT blocks:      {}\n",
        self.reserved_gdt_blocks
      )?;
    }

    write!(f, "Blocks per group:         {}\n", self.blocks_per_group)?;

    if self.feature_ro_compat.bigalloc {
      write!(f, "Clusters per group:       {}\n", self.clusters_per_group)?;
    } else {
      write!(f, "Fragments per group:      {}\n", self.clusters_per_group)?;
    }

    write!(
      f,
      "Inodes per group:         {}\n\
       Inode blocks per group:   {}\n",
      self.inodes_per_group, inode_blocks_per_group
    )?;

    if self.raid_stride != 0 {
      write!(f, "RAID stride:              {}\n", self.raid_stride)?;
    }

    if self.raid_stripe_width != 0 {
      write!(f, "RAID stripe width:        {}\n", self.raid_stripe_width)?;
    }

    if self.first_meta_bg != 0 {
      write!(f, "First meta block group:   {}\n", self.first_meta_bg)?;
    }

    if self.log_groups_per_flex != 0 {
      write!(
        f,
        "Flex block group size:    {}\n",
        self.log_groups_per_flex
      )?;
    }

    write!(
      f,
      "Filesystem created:       {}\n\
       Last mount time:          {}\n\
       Last write time:          {}\n\
       Mount count:              {}\n\
       Maximum mount count:      {}\n\
       Last checked:             {}\n\
       Check interval:           {} second{} ({})\n\
       ",
      crate::util::get_datetime(self.mkfs_time),
      crate::util::get_datetime(self.mtime),
      crate::util::get_datetime(self.wtime),
      self.mnt_count,
      self.max_mnt_count,
      crate::util::get_datetime(self.lastcheck),
      self.checkinterval.num_seconds(),
      if self.checkinterval.num_seconds() == 1 {
        ""
      } else {
        "s"
      },
      self.checkinterval,
    )?;

    if !self.checkinterval.is_zero() {
      write!(
        f,
        "Next check after:         {}",
        self.lastcheck + self.checkinterval
      )?;
    }

    if self.kbytes_written != 0 {
      write!(
        f,
        "Lifetime writes:          {}",
        crate::util::kbytes_to_human_readable(self.kbytes_written)
      )?;
    }

    write!(
      f,
      "Reserved blocks uid:      {}\n\
       Reserved blocks gid:      {}\n",
      crate::util::get_user(self.def_resuid),
      crate::util::get_group(self.def_resgid)
    )?;

    if self.rev_level == RevisionLevel::Dynamic {
      write!(
        f,
        "First inode:              {}\n\
         Inode size:               {}\n",
        self.first_ino, self.inode_size
      )?;
      if self.min_extra_isize != 0 {
        write!(f, "Required extra isize:     {}\n", self.min_extra_isize)?;
      }
      if self.want_extra_isize != 0 {
        write!(f, "Desired extra isize:      {}\n", self.want_extra_isize)?;
      }
    }

    if !self.journal_uuid.is_null() {
      write!(f, "Journal UUID:             {}\n", self.journal_uuid)?;
    }

    if self.journal_inum != 0 {
      write!(f, "Journal inode:            {}\n", self.journal_inum)?;
    }

    if self.journal_dev != 0 {
      write!(f, "Journal device:           {:#06X}\n", self.journal_dev)?;
    }

    if self.last_orphan != 0 {
      write!(f, "First orphan inode:       {}\n", self.last_orphan)?;
    }

    if self.feature_compat.dir_index || self.def_hash_version != HashVersion::Legacy {
      write!(f, "Default directory hash:   {}\n", self.def_hash_version)?;
    }

    if !self.hash_seed.is_null() {
      write!(f, "Directory Hash Seed:      {}\n", self.hash_seed)?;
    }

    if self.jnl_backup_type != 0 {
      write!(f, "Journal backup:           ")?;
      match self.jnl_backup_type {
        1 => write!(f, "inode blocks")?,
        _ => write!(f, "type {}\n", self.jnl_backup_type)?,
      }
    }

    if self.backup_bgs[0] != 0 || self.backup_bgs[1] != 0 {
      let mut output = Vec::new();
      if self.backup_bgs[0] != 0 {
        output.push(format!("{}", self.backup_bgs[0]));
      }
      if self.backup_bgs[1] != 0 {
        output.push(format!("{}", self.backup_bgs[1]));
      }
      write!(f, "Backup block groups:      {}\n", output.join(" "))?;
    }

    if self.snapshot_inum != 0 {
      write!(
        f,
        "Snapshot inode:           {}\n\
         Snapshot ID:              {}\n\
         Snapshot reserved blocks: {}\n",
        self.snapshot_inum, self.snapshot_id, self.snapshot_r_blocks_count
      )?;
    }

    if self.snapshot_list != 0 {
      write!(f, "Snapshot list head:       {}\n", self.snapshot_list)?;
    }

    if self.error_count != 0 {
      write!(f, "FS Error count:           {}\n", self.error_count)?;
    }

    if self.first_error_time != Utc.timestamp(0, 0) {
      write!(
        f,
        "First error time:         {}\n\
         First error function:     {}\n\
         First error line #:       {}\n\
         First error inode #:      {}\n\
         First error block #:      {}\n",
        self.first_error_time,
        self.first_error_func,
        self.first_error_line,
        self.first_error_ino,
        self.first_error_block
      )?;
    }

    if self.last_error_time != Utc.timestamp(0, 0) {
      write!(
        f,
        "Last error time:          {}\n\
         Last error function:      {}\n\
         Last error line #:        {}\n\
         Last error inode #:       {}\n\
         Last error block #:       {}\n",
        self.last_error_time,
        self.last_error_func,
        self.last_error_line,
        self.last_error_ino,
        self.last_error_block
      )?;
    }

    if self.feature_incompat.mmp {
      write!(
        f,
        "MMP block number:         {}\n\
         MMP update interval:      {}\n",
        self.mmp_block, self.mmp_interval
      )?;
    }

    if self.feature_ro_compat.metadata_csum {
      write!(
        f,
        "Checksum type:            {}\n\
                 Checksum:                 {:#010X}\n",
        self.checksum_type, self.checksum
      )?;
    }

    if !self.encrypt_pw_salt.is_null() {
      write!(f, "Encryption PW Salt:       {}\n", self.encrypt_pw_salt)?;
    }

    if self.feature_incompat.csum_seed {
      write!(
        f,
        "Checksum seed:            {:#010X}\n",
        self.checksum_seed
      )?;
    }

    if self.feature_incompat.casefold {
      write!(f, "Character encoding:       {}\n", self.encoding)?;
    }

    write!(f, "")
  }
}
