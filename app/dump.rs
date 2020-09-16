use crate::{die, error, info};
use recover::ext4::Superblock;
use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::path::PathBuf;

pub(crate) struct Dump
{
  pub(crate) path: PathBuf,
  pub(crate) offset: u64,
}

impl Dump
{
  pub(crate) fn run(&self)
  {
    if let Err(err) = self.read_img() {
      die!("An IO error has occurred: {}", err);
    }
  }

  fn read_img(&self) -> io::Result<()>
  {
    let mut img = File::open(self.path.as_path())?;
    img.seek(SeekFrom::Start(self.offset))?;
    img.seek(SeekFrom::Current(1024))?;

    let sb = Superblock::new(&mut img).unwrap_or_else(|err| {
      die!("Superblock error: {}", err);
    });

    if let Some(err) = sb.check_signature() {
      error!("Magic error: {}", err);
      info!("This dump information may not be accurate.");
    }

    print!("{}", sb);

    Ok(())
  }
}
