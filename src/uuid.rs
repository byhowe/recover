#[derive(Debug, Eq, PartialEq)]
pub struct Uuid
{
  time_low: u32,
  time_mid: u16,
  time_hi_and_version: u16,
  clock_seq: u16,
  node: [u8; 6],
}

impl Uuid
{
  pub fn is_null(&self) -> bool
  {
    if self.time_low == 0
      && self.time_mid == 0
      && self.time_hi_and_version == 0
      && self.clock_seq == 0
      && self.node[0] == 0
      && self.node[1] == 0
      && self.node[2] == 0
      && self.node[3] == 0
      && self.node[4] == 0
      && self.node[5] == 0
    {
      true
    } else {
      false
    }
  }
}

impl From<(u32, u16, u16, u16, [u8; 6])> for Uuid
{
  fn from(uuid: (u32, u16, u16, u16, [u8; 6])) -> Self
  {
    Self {
      time_low: uuid.0,
      time_mid: uuid.1,
      time_hi_and_version: uuid.2,
      clock_seq: uuid.3,
      node: uuid.4,
    }
  }
}

impl std::fmt::Display for Uuid
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    if self.is_null() {
      write!(f, "<none>")
    } else {
      write!(
        f,
        "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
        self.time_low,
        self.time_mid,
        self.time_hi_and_version,
        self.clock_seq >> 8,
        self.clock_seq & 0xFF,
        self.node[0],
        self.node[1],
        self.node[2],
        self.node[3],
        self.node[4],
        self.node[5]
      )
    }
  }
}
