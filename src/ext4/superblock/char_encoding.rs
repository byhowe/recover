static ENCODING_MAP: [Encoding; 2] = [
  Encoding {
    name: "utf8-12.1",
    encoding_magic: Encoding::UTF8_12_1_MAGIC,
  },
  Encoding {
    name: "utf8",
    encoding_magic: Encoding::UTF8_12_1_MAGIC,
  },
];

struct Encoding<'a>
{
  name: &'a str,
  encoding_magic: u16,
}

impl Encoding<'_>
{
  const UTF8_12_1_MAGIC: u16 = 1;
}

#[derive(Eq, PartialEq)]
pub struct CharEncoding
{
  encoding: u16,
}

impl From<u16> for CharEncoding
{
  fn from(encoding: u16) -> Self
  {
    Self { encoding }
  }
}

impl std::fmt::Display for CharEncoding
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    for e in &ENCODING_MAP {
      if e.encoding_magic == self.encoding {
        return write!(f, "{}", e.name);
      }
    }
    write!(f, "UNKNOWN_ENCODING_{}", self.encoding)
  }
}

impl std::fmt::Debug for CharEncoding
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{:?}", self.encoding)
  }
}
