use super::{HashVersion, RevisionLevel, Superblock};
use chrono::{TimeZone, Utc};

impl std::fmt::Display for Superblock
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let inode_blocks_per_group: u32 =
      ((self.inodes_per_group * self.get_inode_size() as u32) + self.get_block_size() - 1)
        / self.get_block_size();

    writeln!(
      f,
      "Filesystem volume name:   {}",
      crate::util::get_string(&self.volume_name)
    )?;
    writeln!(
      f,
      "Last mounted on:          {}",
      crate::util::get_string(&self.last_mounted)
    )?;
    writeln!(f, "Filesystem UUID:          {}", self.uuid)?;
    writeln!(f, "Filesystem magic number:  {:#X}", self.magic)?;
    writeln!(f, "Filesystem revision #:    {}", self.rev_level)?;
    writeln!(
      f,
      "Filesystem features:      {}",
      crate::util::get_string_list(&self.get_features())
    )?;
    writeln!(f, "Filesystem flags:         {}", self.flags)?;
    writeln!(f, "Default mount options:    {}", self.default_mount_opts)?;
    writeln!(
      f,
      "Mount options:            {}",
      crate::util::get_string(&self.mount_opts)
    )?;
    writeln!(f, "Filesystem state:         {}", self.state)?;
    writeln!(f, "Errors behaviour:         {}", self.errors)?;
    writeln!(f, "Filesystem OS type:       {}", self.creator_os)?;
    writeln!(f, "Inode count:              {}", self.inodes_count)?;
    writeln!(f, "Block count:              {}", self.get_blocks_count())?;
    writeln!(
      f,
      "Reserved block count:     {}",
      self.get_reserved_blocks_count()
    )?;

    if self.overhead_blocks != 0 {
      writeln!(f, "Overhead clusters:        {}", self.overhead_blocks)?;
    }

    writeln!(
      f,
      "Free blocks:              {}",
      self.get_free_blocks_count()
    )?;
    writeln!(f, "Free inodes:              {}", self.free_inodes_count)?;
    writeln!(f, "First block:              {}", self.first_data_block)?;
    writeln!(f, "Block size:               {}", self.get_block_size())?;

    if self.feature_ro_compat.bigalloc {
      writeln!(f, "Cluster size:             {}", self.get_cluster_size())?;
    } else {
      writeln!(f, "Fragment size:            {}", self.get_cluster_size())?;
    }

    if self.feature_incompat.bit64 {
      writeln!(f, "Group descriptor size:    {}", self.desc_size)?;
    }

    if self.reserved_gdt_blocks != 0 {
      writeln!(f, "Reserved GDT blocks:      {}", self.reserved_gdt_blocks)?;
    }

    writeln!(f, "Blocks per group:         {}", self.blocks_per_group)?;

    if self.feature_ro_compat.bigalloc {
      writeln!(f, "Clusters per group:       {}", self.clusters_per_group)?;
    } else {
      writeln!(f, "Fragments per group:      {}", self.clusters_per_group)?;
    }

    writeln!(f, "Inodes per group:         {}", self.inodes_per_group)?;
    writeln!(f, "Inode blocks per group:   {}", inode_blocks_per_group)?;

    if self.raid_stride != 0 {
      writeln!(f, "RAID stride:              {}", self.raid_stride)?;
    }

    if self.raid_stripe_width != 0 {
      writeln!(f, "RAID stripe width:        {}", self.raid_stripe_width)?;
    }

    if self.first_meta_bg != 0 {
      writeln!(f, "First meta block group:   {}", self.first_meta_bg)?;
    }

    if self.log_groups_per_flex != 0 {
      writeln!(f, "Flex block group size:    {}", self.log_groups_per_flex)?;
    }

    writeln!(
      f,
      "Filesystem created:       {}",
      crate::util::get_datetime(self.mkfs_time)
    )?;
    writeln!(
      f,
      "Last mount time:          {}",
      crate::util::get_datetime(self.mtime)
    )?;
    writeln!(
      f,
      "Last write time:          {}",
      crate::util::get_datetime(self.wtime)
    )?;
    writeln!(f, "Mount count:              {}", self.mnt_count)?;
    writeln!(f, "Maximum mount count:      {}", self.max_mnt_count)?;
    writeln!(
      f,
      "Last checked:             {}",
      crate::util::get_datetime(self.lastcheck)
    )?;
    writeln!(
      f,
      "Check interval:           {} second{} ({})",
      self.checkinterval.num_seconds(),
      if self.checkinterval.num_seconds() == 1 {
        ""
      } else {
        "s"
      },
      self.checkinterval,
    )?;

    if !self.checkinterval.is_zero() {
      writeln!(
        f,
        "Next check after:         {}",
        self.lastcheck + self.checkinterval
      )?;
    }

    if self.kbytes_written != 0 {
      writeln!(
        f,
        "Lifetime writes:          {}",
        crate::util::kbytes_to_human_readable(self.kbytes_written)
      )?;
    }

    writeln!(
      f,
      "Reserved blocks uid:      {}",
      crate::util::get_user(self.def_resuid)
    )?;
    writeln!(
      f,
      "Reserved blocks gid:      {}",
      crate::util::get_group(self.def_resgid)
    )?;

    if self.rev_level == RevisionLevel::Dynamic {
      writeln!(f, "First inode:              {}", self.first_ino)?;
      writeln!(f, "Inode size:               {}", self.inode_size)?;
      if self.min_extra_isize != 0 {
        writeln!(f, "Required extra isize:     {}", self.min_extra_isize)?;
      }
      if self.want_extra_isize != 0 {
        writeln!(f, "Desired extra isize:      {}", self.want_extra_isize)?;
      }
    }

    if !self.journal_uuid.is_null() {
      writeln!(f, "Journal UUID:             {}", self.journal_uuid)?;
    }

    if self.journal_inum != 0 {
      writeln!(f, "Journal inode:            {}", self.journal_inum)?;
    }

    if self.journal_dev != 0 {
      writeln!(f, "Journal device:           {:#06X}", self.journal_dev)?;
    }

    if self.last_orphan != 0 {
      writeln!(f, "First orphan inode:       {}", self.last_orphan)?;
    }

    if self.feature_compat.dir_index || self.def_hash_version != HashVersion::Legacy {
      writeln!(f, "Default directory hash:   {}", self.def_hash_version)?;
    }

    if !self.hash_seed.is_null() {
      writeln!(f, "Directory Hash Seed:      {}", self.hash_seed)?;
    }

    if self.jnl_backup_type != 0 {
      writeln!(
        f,
        "Journal backup:           {}",
        match self.jnl_backup_type {
          1 => String::from("inode blocks"),
          _ => format!("type {}", self.jnl_backup_type),
        }
      )?;
    }

    if self.backup_bgs[0] != 0 || self.backup_bgs[1] != 0 {
      let mut output = Vec::new();
      if self.backup_bgs[0] != 0 {
        output.push(format!("{}", self.backup_bgs[0]));
      }
      if self.backup_bgs[1] != 0 {
        output.push(format!("{}", self.backup_bgs[1]));
      }
      writeln!(f, "Backup block groups:      {}", output.join(" "))?;
    }

    if self.snapshot_inum != 0 {
      writeln!(f, "Snapshot inode:           {}", self.snapshot_inum)?;
      writeln!(f, "Snapshot ID:              {}", self.snapshot_id)?;
      writeln!(
        f,
        "Snapshot reserved blocks: {}",
        self.snapshot_r_blocks_count
      )?;
    }

    if self.snapshot_list != 0 {
      writeln!(f, "Snapshot list head:       {}", self.snapshot_list)?;
    }

    if self.error_count != 0 {
      writeln!(f, "FS Error count:           {}", self.error_count)?;
    }

    if self.first_error_time != Utc.timestamp(0, 0) {
      writeln!(
        f,
        "First error time:         {}",
        crate::util::get_datetime(self.first_error_time)
      )?;
      writeln!(f, "First error function:     {}", self.first_error_func)?;
      writeln!(f, "First error line #:       {}", self.first_error_line)?;
      writeln!(f, "First error inode #:      {}", self.first_error_ino)?;
      writeln!(f, "First error block #:      {}", self.first_error_block)?;
    }

    if self.last_error_time != Utc.timestamp(0, 0) {
      writeln!(
        f,
        "Last error time:          {}",
        crate::util::get_datetime(self.last_error_time)
      )?;
      writeln!(f, "Last error function:      {}", self.last_error_func)?;
      writeln!(f, "Last error line #:        {}", self.last_error_line)?;
      writeln!(f, "Last error inode #:       {}", self.last_error_ino)?;
      writeln!(f, "Last error block #:       {}", self.last_error_block)?;
    }

    if self.feature_incompat.mmp {
      writeln!(f, "MMP block number:         {}", self.mmp_block)?;
      writeln!(f, "MMP update interval:      {}", self.mmp_interval)?;
    }

    if self.feature_ro_compat.metadata_csum {
      writeln!(f, "Checksum type:            {}", self.checksum_type)?;
      writeln!(f, "Checksum:                 {:#010X}", self.checksum)?;
    }

    if !self.encrypt_pw_salt.is_null() {
      writeln!(f, "Encryption PW Salt:       {}", self.encrypt_pw_salt)?;
    }

    if self.feature_incompat.csum_seed {
      writeln!(f, "Checksum seed:            {:#010X}", self.checksum_seed)?;
    }

    if self.feature_incompat.casefold {
      writeln!(f, "Character encoding:       {}", self.encoding)?;
    }

    Ok(())
  }
}
