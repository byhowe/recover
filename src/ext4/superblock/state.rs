#[derive(Debug)]
pub struct State
{
  pub cleanly_unmounted: bool,
  pub errors_detected: bool,
  pub orphans_being_recovered: bool,
  pub unknown_bits: bool,
}

impl State
{
  const CLEANLY_UNMOUNTED: u16 = 0x0001;
  const ERRORS_DETECTED: u16 = 0x0002;
  const ORPHANS_BEING_RECOVERED: u16 = 0x0004;

  pub fn from_raw(state: u16) -> Self
  {
    Self {
      cleanly_unmounted: state & Self::CLEANLY_UNMOUNTED != 0,
      errors_detected: state & Self::ERRORS_DETECTED != 0,
      orphans_being_recovered: state & Self::ORPHANS_BEING_RECOVERED != 0,
      unknown_bits: state
        & !(Self::CLEANLY_UNMOUNTED | Self::ERRORS_DETECTED | Self::ORPHANS_BEING_RECOVERED)
        != 0,
    }
  }
}

impl std::fmt::Display for State
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let mut output = Vec::new();
    if self.cleanly_unmounted {
      output.push("clean");
    } else {
      output.push("not clean");
    }
    if self.errors_detected {
      output.push("errors detected");
    }
    if self.orphans_being_recovered {
      output.push("orphans being recovered");
    }
    if self.unknown_bits {
      output.push("unknown bits were found");
    }
    write!(f, "{}", output.join(", "))
  }
}
