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

impl From<u128> for Uuid
{
  fn from(uuid: u128) -> Self
  {
    let bytes = [
      uuid as u8,
      (uuid >> 8) as u8,
      (uuid >> 16) as u8,
      (uuid >> 24) as u8,
      (uuid >> 32) as u8,
      (uuid >> 40) as u8,
      (uuid >> 48) as u8,
      (uuid >> 56) as u8,
      (uuid >> 64) as u8,
      (uuid >> 72) as u8,
    ];
    let mut uuid_struct = Self {
      time_low: 0,
      time_mid: 0,
      time_hi_and_version: 0,
      clock_seq: 0,
      node: [0; 6],
    };

    let mut tmp: u32 = bytes[0] as u32;
    tmp = (tmp << 8) | bytes[1] as u32;
    tmp = (tmp << 8) | bytes[2] as u32;
    tmp = (tmp << 8) | bytes[3] as u32;
    uuid_struct.time_low = tmp;

    let mut tmp: u16 = bytes[4] as u16;
    tmp = (tmp << 8) | bytes[5] as u16;
    uuid_struct.time_mid = tmp;

    let mut tmp: u16 = bytes[6] as u16;
    tmp = (tmp << 8) | bytes[7] as u16;
    uuid_struct.time_hi_and_version = tmp;

    let mut tmp: u16 = bytes[8] as u16;
    tmp = (tmp << 8) | bytes[9] as u16;
    uuid_struct.clock_seq = tmp;

    uuid_struct.node = [
      (uuid >> 80) as u8,
      (uuid >> 88) as u8,
      (uuid >> 96) as u8,
      (uuid >> 104) as u8,
      (uuid >> 112) as u8,
      (uuid >> 120) as u8,
    ];

    uuid_struct
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

impl std::fmt::Debug for Uuid
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", self)
  }
}
