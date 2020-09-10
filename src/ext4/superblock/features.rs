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
  pub unknown_bits: bool,
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

  pub fn from_raw(feature: u32) -> Self
  {
    Self {
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
      unknown_bits: feature
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
        != 0,
    }
  }

  pub fn features_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    if self.dir_prealloc {
      output.push("dir_prealloc");
    }
    if self.imagic_inodes {
      output.push("imagic_inodes");
    }
    if self.has_journal {
      output.push("has_journal");
    }
    if self.ext_attr {
      output.push("ext_attr");
    }
    if self.resize_inode {
      output.push("resize_inode");
    }
    if self.dir_index {
      output.push("dir_index");
    }
    if self.lazy_bg {
      output.push("lazy_bg");
    }
    if self.exclude_inode {
      output.push("exclude_inode");
    }
    if self.exclude_bitmap {
      output.push("exclude_bitmap");
    }
    if self.sparse_super2 {
      output.push("sparse_super2");
    }
    output
  }
}

impl std::fmt::Display for FeatureCompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.features_list()))
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
  pub extent: bool,
  pub bit64: bool,
  pub mmp: bool,
  pub flex_bg: bool,
  pub ea_inode: bool,
  pub dirdata: bool,
  pub csum_seed: bool,
  pub largedir: bool,
  pub inline_data: bool,
  pub encrypt: bool,
  pub casefold: bool,
  pub unknown_bits: bool,
}

impl FeatureIncompat
{
  const COMPRESSION: u32 = 0x1;
  const FILETYPE: u32 = 0x2;
  const RECOVER: u32 = 0x4;
  const JOURNAL_DEV: u32 = 0x8;
  const META_BG: u32 = 0x10;
  const EXTENT: u32 = 0x40;
  const BIT64: u32 = 0x80;
  const MMP: u32 = 0x100;
  const FLEX_BG: u32 = 0x200;
  const EA_INODE: u32 = 0x400;
  const DIRDATA: u32 = 0x1000;
  const CSUM_SEED: u32 = 0x2000;
  const LARGEDIR: u32 = 0x4000;
  const INLINE_DATA: u32 = 0x8000;
  const ENCRYPT: u32 = 0x10000;
  const CASEFOLD: u32 = 0x20000;

  pub fn from_raw(feature: u32) -> Self
  {
    Self {
      compression: feature & Self::COMPRESSION != 0,
      filetype: feature & Self::FILETYPE != 0,
      recover: feature & Self::RECOVER != 0,
      journal_dev: feature & Self::JOURNAL_DEV != 0,
      meta_bg: feature & Self::META_BG != 0,
      extent: feature & Self::EXTENT != 0,
      bit64: feature & Self::BIT64 != 0,
      mmp: feature & Self::MMP != 0,
      flex_bg: feature & Self::FLEX_BG != 0,
      ea_inode: feature & Self::EA_INODE != 0,
      dirdata: feature & Self::DIRDATA != 0,
      csum_seed: feature & Self::CSUM_SEED != 0,
      largedir: feature & Self::LARGEDIR != 0,
      inline_data: feature & Self::INLINE_DATA != 0,
      encrypt: feature & Self::ENCRYPT != 0,
      casefold: feature & Self::CASEFOLD != 0,
      unknown_bits: feature
        & !(Self::COMPRESSION
          | Self::FILETYPE
          | Self::RECOVER
          | Self::JOURNAL_DEV
          | Self::META_BG
          | Self::EXTENT
          | Self::BIT64
          | Self::MMP
          | Self::FLEX_BG
          | Self::EA_INODE
          | Self::DIRDATA
          | Self::CSUM_SEED
          | Self::LARGEDIR
          | Self::INLINE_DATA
          | Self::ENCRYPT
          | Self::CASEFOLD)
        != 0,
    }
  }

  pub fn features_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    if self.compression {
      output.push("compression");
    }
    if self.filetype {
      output.push("filetype");
    }
    if self.recover {
      output.push("recover");
    }
    if self.journal_dev {
      output.push("journal_dev");
    }
    if self.meta_bg {
      output.push("meta_bg");
    }
    if self.extent {
      output.push("extent");
    }
    if self.bit64 {
      output.push("64bit");
    }
    if self.mmp {
      output.push("mmp");
    }
    if self.flex_bg {
      output.push("flex_bg");
    }
    if self.ea_inode {
      output.push("ea_inode");
    }
    if self.dirdata {
      output.push("dirdata");
    }
    if self.csum_seed {
      output.push("csum_seed");
    }
    if self.largedir {
      output.push("largedir");
    }
    if self.inline_data {
      output.push("inline_data");
    }
    if self.encrypt {
      output.push("encrypt");
    }
    if self.casefold {
      output.push("casefold")
    }
    output
  }
}

impl std::fmt::Display for FeatureIncompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.features_list()))
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
  pub unknown_bits: bool,
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

  pub fn from_raw(feature: u32) -> Self
  {
    Self {
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
      unknown_bits: feature
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
        != 0,
    }
  }

  pub fn features_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    if self.sparse_super {
      output.push("sparse_super");
    }
    if self.large_file {
      output.push("large_file");
    }
    if self.btree_dir {
      output.push("btree_dir");
    }
    if self.huge_file {
      output.push("huge_file");
    }
    if self.gdt_csum {
      output.push("gdt_csum");
    }
    if self.dir_nlink {
      output.push("dir_nlink");
    }
    if self.extra_isize {
      output.push("extra_isize");
    }
    if self.has_snapshot {
      output.push("has_snapshot");
    }
    if self.quota {
      output.push("quota");
    }
    if self.bigalloc {
      output.push("bigalloc");
    }
    if self.metadata_csum {
      output.push("metadata_csum");
    }
    if self.replica {
      output.push("replica");
    }
    if self.readonly {
      output.push("readonly");
    }
    if self.project {
      output.push("project");
    }
    if self.verity {
      output.push("verity");
    }
    output
  }
}

impl std::fmt::Display for ReadOnlyFeatureCompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.features_list()))
  }
}
