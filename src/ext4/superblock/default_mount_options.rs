use crate::add_to_list;
use bitflags::bitflags;

bitflags! {
  pub struct DefaultMountOptions: u32
  {
    const DEBUG = 0x1;
    const BSDGROUPS = 0x2;
    const XATTR_USER = 0x4;
    const ACL = 0x8;
    const UID16 = 0x10;
    const JMODE_DATA = 0x20;
    const JMODE_ORDERED = 0x40;
    const JMODE_WBACK = 0x60;
    const NOBARRIER = 0x100;
    const BLOCK_VALIDITY = 0x200;
    const DISCARD = 0x400;
    const NODEALLOC = 0x800;
  }
}

impl DefaultMountOptions
{
  pub fn from_raw(raw: u32) -> Self
  {
    unsafe { Self::from_bits_unchecked(raw) }
  }

  pub fn default_mount_opts_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    add_to_list!(self, output, "debug", DEBUG);
    add_to_list!(self, output, "bsdgroups", BSDGROUPS);
    add_to_list!(self, output, "xattr_user", XATTR_USER);
    add_to_list!(self, output, "acl", ACL);
    add_to_list!(self, output, "uid16", UID16);
    add_to_list!(self, output, "jmode_data", JMODE_DATA);
    add_to_list!(self, output, "jmode_ordered", JMODE_ORDERED);
    add_to_list!(self, output, "jmode_wback", JMODE_WBACK);
    add_to_list!(self, output, "nobarrier", NOBARRIER);
    add_to_list!(self, output, "block_validity", BLOCK_VALIDITY);
    add_to_list!(self, output, "discard", DISCARD);
    add_to_list!(self, output, "nodealloc", NODEALLOC);
    if !self.intersects(Self::all()) {
      output.push("(unknown_bits)");
    }
    output
  }
}

impl std::fmt::Display for DefaultMountOptions
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", crate::util::get_string_list(&self.default_mount_opts_list()))
  }
}
