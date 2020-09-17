use crate::{die, error, info};
use recover::ext4::FileSystem;
use std::fs::File;
use std::io;
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
    let img = File::open(self.path.as_path())?;

    let mut fs = FileSystem::new(img, self.offset).unwrap_or_else(|err| {
      die!("{}", err);
    });

    if let Some(err) = fs.sb.check_signature() {
      error!("Magic error: {}", err);
      info!("This dump information may not be accurate.");
    }

    print!("{}", fs.sb);

    for i in fs.iter_group_descriptors() {
      println!("{:#?}", i);
    }

    Ok(())
  }
}
