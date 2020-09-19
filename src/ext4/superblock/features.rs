use super::Superblock;
use crate::add_to_list;
use bitflags::bitflags;

macro_rules! feature {
  ($feature_name:ident, $feature_type:ident, $name:ident, $feature_flag:ident) => {
    impl Superblock
    {
      #[inline(always)]
      pub fn $name(&self) -> bool
      {
        self.$feature_name.contains($feature_type::$feature_flag)
      }
    }
  };
}

macro_rules! feature_compat {
  ($name:ident, $feature_flag:ident) => {
    feature!(feature_compat, FeatureCompat, $name, $feature_flag);
  };
}

macro_rules! feature_incompat {
  ($name:ident, $feature_flag:ident) => {
    feature!(feature_incompat, FeatureIncompat, $name, $feature_flag);
  };
}

macro_rules! feature_ro_compat {
  ($name:ident, $feature_flag:ident) => {
    feature!(feature_ro_compat, ReadOnlyFeatureCompat, $name, $feature_flag);
  };
}

bitflags! {
  pub struct FeatureCompat: u32
  {
    const DIR_PREALLOC = 0x1;
    const IMAGIC_INODE = 0x2;
    const HAS_JOURNAL = 0x4;
    const EXT_ATTR = 0x8;
    const RESIZE_INODE = 0x10;
    const DIR_INDEX = 0x20;
    const LAZY_BG = 0x40;
    const EXCLUDE_INODE = 0x80;
    const EXCLUDE_BITMAP = 0x100;
    const SPARSE_SUPER2 = 0x200;
  }
}

impl FeatureCompat
{
  pub fn from_raw(raw: u32) -> Self
  {
    unsafe { Self::from_bits_unchecked(raw) }
  }

  pub fn features_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    add_to_list!(self, output, "dir_prealloc", DIR_PREALLOC);
    add_to_list!(self, output, "imagic_inodes", IMAGIC_INODE);
    add_to_list!(self, output, "has_journal", HAS_JOURNAL);
    add_to_list!(self, output, "ext_attr", EXT_ATTR);
    add_to_list!(self, output, "resize_inode", RESIZE_INODE);
    add_to_list!(self, output, "dir_index", DIR_INDEX);
    add_to_list!(self, output, "lazy_bg", LAZY_BG);
    add_to_list!(self, output, "exclude_inode", EXCLUDE_INODE);
    add_to_list!(self, output, "exclude_bitmap", EXCLUDE_BITMAP);
    add_to_list!(self, output, "sparse_super2", SPARSE_SUPER2);
    output
  }

  pub fn unknown_bits(&self) -> bool
  {
    !self.intersects(Self::all())
  }
}

impl std::fmt::Display for FeatureCompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.features_list()))
  }
}

bitflags! {
  pub struct FeatureIncompat: u32
  {
    const COMPRESSION = 0x1;
    const FILETYPE = 0x2;
    const RECOVER = 0x4;
    const JOURNAL_DEV = 0x8;
    const META_BG = 0x10;
    const EXTENT = 0x40;
    const BIT64 = 0x80;
    const MMP = 0x100;
    const FLEX_BG = 0x200;
    const EA_INODE = 0x400;
    const DIRDATA = 0x1000;
    const CSUM_SEED = 0x2000;
    const LARGEDIR = 0x4000;
    const INLINE_DATA = 0x8000;
    const ENCRYPT = 0x10000;
    const CASEFOLD = 0x20000;
  }
}

impl FeatureIncompat
{
  pub fn from_raw(raw: u32) -> Self
  {
    unsafe { Self::from_bits_unchecked(raw) }
  }

  pub fn features_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    add_to_list!(self, output, "compression", COMPRESSION);
    add_to_list!(self, output, "filetype", FILETYPE);
    add_to_list!(self, output, "recover", RECOVER);
    add_to_list!(self, output, "journal_dev", JOURNAL_DEV);
    add_to_list!(self, output, "meta_bg", META_BG);
    add_to_list!(self, output, "extent", EXTENT);
    add_to_list!(self, output, "64bit", BIT64);
    add_to_list!(self, output, "mmp", MMP);
    add_to_list!(self, output, "flex_bg", FLEX_BG);
    add_to_list!(self, output, "ea_inode", EA_INODE);
    add_to_list!(self, output, "dirdata", DIRDATA);
    add_to_list!(self, output, "csum_seed", CSUM_SEED);
    add_to_list!(self, output, "largedir", LARGEDIR);
    add_to_list!(self, output, "inline_data", INLINE_DATA);
    add_to_list!(self, output, "encrypt", ENCRYPT);
    add_to_list!(self, output, "casefold", CASEFOLD);
    output
  }

  pub fn unknown_bits(&self) -> bool
  {
    !self.intersects(Self::all())
  }
}

impl std::fmt::Display for FeatureIncompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.features_list()))
  }
}

bitflags! {
  pub struct ReadOnlyFeatureCompat: u32
  {
    const SPARSE_SUPER = 0x1;
    const LARGE_FILE = 0x2;
    const BTREE_DIR = 0x4;
    const HUGE_FILE = 0x8;
    const GDT_CSUM = 0x10;
    const DIR_NLINK = 0x20;
    const EXTRA_ISIZE = 0x40;
    const HAS_SNAPSHOT = 0x80;
    const QUOTA = 0x100;
    const BIGALLOC = 0x200;
    const METADATA_CSUM = 0x400;
    const REPLICA = 0x800;
    const READONLY = 0x1000;
    const PROJECT = 0x2000;
    const VERITY = 0x8000;
  }
}

impl ReadOnlyFeatureCompat
{
  pub fn from_raw(raw: u32) -> Self
  {
    unsafe { Self::from_bits_unchecked(raw) }
  }

  pub fn features_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    add_to_list!(self, output, "sparse_super", SPARSE_SUPER);
    add_to_list!(self, output, "large_file", LARGE_FILE);
    add_to_list!(self, output, "btree_dir", BTREE_DIR);
    add_to_list!(self, output, "huge_file", HUGE_FILE);
    add_to_list!(self, output, "gdt_csum", GDT_CSUM);
    add_to_list!(self, output, "dir_nlink", DIR_NLINK);
    add_to_list!(self, output, "extra_isize", EXTRA_ISIZE);
    add_to_list!(self, output, "has_snapshot", HAS_SNAPSHOT);
    add_to_list!(self, output, "quota", QUOTA);
    add_to_list!(self, output, "bigalloc", BIGALLOC);
    add_to_list!(self, output, "metadata_csum", METADATA_CSUM);
    add_to_list!(self, output, "replica", REPLICA);
    add_to_list!(self, output, "readonly", READONLY);
    add_to_list!(self, output, "project", PROJECT);
    add_to_list!(self, output, "verity", VERITY);
    output
  }

  pub fn unknown_bits(&self) -> bool
  {
    !self.intersects(Self::all())
  }
}

impl std::fmt::Display for ReadOnlyFeatureCompat
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.features_list()))
  }
}

feature_compat!(feature_dir_prealloc, DIR_PREALLOC);
feature_compat!(feature_imagic_inode, IMAGIC_INODE);
feature_compat!(feature_has_journal, HAS_JOURNAL);
feature_compat!(feature_ext_attr, EXT_ATTR);
feature_compat!(feature_resize_inode, RESIZE_INODE);
feature_compat!(feature_dir_index, DIR_INDEX);
feature_compat!(feature_lazy_bg, LAZY_BG);
feature_compat!(feature_exclude_inode, EXCLUDE_INODE);
feature_compat!(feature_exclude_bitmap, EXCLUDE_BITMAP);
feature_compat!(feature_sparse_super2, SPARSE_SUPER2);

feature_incompat!(feature_compression, COMPRESSION);
feature_incompat!(feature_filetype, FILETYPE);
feature_incompat!(feature_recover, RECOVER);
feature_incompat!(feature_journal_dev, JOURNAL_DEV);
feature_incompat!(feature_meta_bg, META_BG);
feature_incompat!(feature_extent, EXTENT);
feature_incompat!(feature_64bit, BIT64);
feature_incompat!(feature_mmp, MMP);
feature_incompat!(feature_flex_bg, FLEX_BG);
feature_incompat!(feature_ea_inode, EA_INODE);
feature_incompat!(feature_dirdata, DIRDATA);
feature_incompat!(feature_csum_seed, CSUM_SEED);
feature_incompat!(feature_largedir, LARGEDIR);
feature_incompat!(feature_inline_data, INLINE_DATA);
feature_incompat!(feature_encrypt, ENCRYPT);
feature_incompat!(feature_casefold, CASEFOLD);

feature_ro_compat!(feature_sparse_super, SPARSE_SUPER);
feature_ro_compat!(feature_large_file, LARGE_FILE);
feature_ro_compat!(feature_btree_dir, BTREE_DIR);
feature_ro_compat!(feature_huge_file, HUGE_FILE);
feature_ro_compat!(feature_gdt_csum, GDT_CSUM);
feature_ro_compat!(feature_dir_nlink, DIR_NLINK);
feature_ro_compat!(feature_extra_isize, EXTRA_ISIZE);
feature_ro_compat!(feature_has_snapshot, HAS_SNAPSHOT);
feature_ro_compat!(feature_quota, QUOTA);
feature_ro_compat!(feature_bigalloc, BIGALLOC);
feature_ro_compat!(feature_metadata_csum, METADATA_CSUM);
feature_ro_compat!(feature_replica, REPLICA);
feature_ro_compat!(feature_readonly, READONLY);
feature_ro_compat!(feature_project, PROJECT);
feature_ro_compat!(feature_verity, VERITY);
