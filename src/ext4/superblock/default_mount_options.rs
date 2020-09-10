#[derive(Debug)]
pub struct DefaultMountOptions
{
  pub debug: bool,
  pub bsdgroups: bool,
  pub xattr_user: bool,
  pub acl: bool,
  pub uid16: bool,
  pub jmode_data: bool,
  pub jmode_ordered: bool,
  pub jmode_wback: bool,
  pub nobarrier: bool,
  pub block_validity: bool,
  pub discard: bool,
  pub nodealloc: bool,
  pub unknown_bits: bool,
}

impl DefaultMountOptions
{
  const DEBUG: u32 = 0x1;
  const BSDGROUPS: u32 = 0x2;
  const XATTR_USER: u32 = 0x4;
  const ACL: u32 = 0x8;
  const UID16: u32 = 0x10;
  const JMODE_DATA: u32 = 0x20;
  const JMODE_ORDERED: u32 = 0x40;
  const JMODE_WBACK: u32 = 0x60;
  const NOBARRIER: u32 = 0x100;
  const BLOCK_VALIDITY: u32 = 0x200;
  const DISCARD: u32 = 0x400;
  const NODEALLOC: u32 = 0x800;

  pub fn from_raw(opts: u32) -> Self
  {
    Self {
      debug: opts & Self::DEBUG != 0,
      bsdgroups: opts & Self::BSDGROUPS != 0,
      xattr_user: opts & Self::XATTR_USER != 0,
      acl: opts & Self::ACL != 0,
      uid16: opts & Self::UID16 != 0,
      jmode_data: opts & Self::JMODE_DATA != 0,
      jmode_ordered: opts & Self::JMODE_ORDERED != 0,
      jmode_wback: opts & Self::JMODE_WBACK != 0,
      nobarrier: opts & Self::NOBARRIER != 0,
      block_validity: opts & Self::BLOCK_VALIDITY != 0,
      discard: opts & Self::DISCARD != 0,
      nodealloc: opts & Self::NODEALLOC != 0,
      unknown_bits: opts
        & !(Self::DEBUG
          | Self::BSDGROUPS
          | Self::XATTR_USER
          | Self::ACL
          | Self::UID16
          | Self::JMODE_DATA
          | Self::JMODE_ORDERED
          | Self::JMODE_WBACK
          | Self::NOBARRIER
          | Self::BLOCK_VALIDITY
          | Self::DISCARD
          | Self::NODEALLOC)
        != 0,
    }
  }

  pub fn default_mount_opts_list(&self) -> Vec<&str>
  {
    let mut output = Vec::new();
    if self.debug {
      output.push("debug");
    }
    if self.bsdgroups {
      output.push("bsdgroups");
    }
    if self.xattr_user {
      output.push("xattr_user");
    }
    if self.acl {
      output.push("acl");
    }
    if self.uid16 {
      output.push("uid16");
    }
    if self.jmode_data {
      output.push("jmode_data");
    }
    if self.jmode_ordered {
      output.push("jmode_ordered");
    }
    if self.jmode_wback {
      output.push("jmode_wback");
    }
    if self.nobarrier {
      output.push("nobarrier");
    }
    if self.block_validity {
      output.push("block_validity");
    }
    if self.discard {
      output.push("discard");
    }
    if self.nodealloc {
      output.push("nodealloc");
    }
    if self.unknown_bits {
      output.push("(unknown_bits)");
    }
    output
  }
}

impl std::fmt::Display for DefaultMountOptions
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(
      f,
      "{}",
      crate::util::get_string_list(&self.default_mount_opts_list())
    )
  }
}
