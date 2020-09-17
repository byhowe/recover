use crate::add_to_list;
use bitflags::bitflags;

bitflags! {
    pub struct FeatureCompat: u32 {
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
