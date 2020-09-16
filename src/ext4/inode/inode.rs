use super::InodeRaw;
use std::io;

#[derive(Debug)]
pub struct Inode
{
    raw: InodeRaw
}

impl Inode
{
    pub const RAW_WIDTH: usize = InodeRaw::WIDTH;

    pub fn new<R>(inner: &mut R) -> Result<Self, Error>
        where R: io::Read
    {
        let mut block: [u8; Self::RAW_WIDTH] = [0; Self::RAW_WIDTH];
        inner.read_exact(&mut block)?;
        Ok(InodeRaw::from(&block).into())
    }
}

impl From<InodeRaw> for Inode
{
    fn from(raw: InodeRaw) -> Self {
        Self {
            raw
        }
    }
}

#[derive(Debug)]
pub enum Error
{
    IO(io::Error)
}

impl From<io::Error> for Error
{
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::IO(error) => format!("An IO error occurred while reading the inode table: {}", error)
        })
    }
}
