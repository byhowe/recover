use crate::add_to_list;
use bitflags::bitflags;

bitflags! {
  pub struct State: u16
  {
    const CLEANLY_UNMOUNTED = 0x0001;
    const ERRORS_DETECTED = 0x0002;
    const ORPHANS_BEING_RECOVERED = 0x0004;
  }
}

impl State
{
  pub fn from_raw(state: u16) -> Self
  {
    unsafe { Self::from_bits_unchecked(state) }
  }
}

impl std::fmt::Display for State
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    let mut output = Vec::new();
    if self.contains(Self::CLEANLY_UNMOUNTED) {
      output.push("clean");
    } else {
      output.push("not clean");
    }
    add_to_list!(self, output, "errors detected", ERRORS_DETECTED);
    add_to_list!(self, output, "orphans being recovered", ORPHANS_BEING_RECOVERED);
    if !self.intersects(Self::all()) {
      output.push("unknown bits were found");
    }
    write!(f, "{}", output.join(", "))
  }
}
