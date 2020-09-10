use recover::ext4::Superblock;
use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::path::PathBuf;

pub(crate) struct Dump
{
  pub(crate) path: PathBuf,
}

impl Dump
{
  pub(crate) fn run(&self)
  {
    let img = self.read_img().unwrap_or_else(|err| {
      eprintln!("An IO error has occurred: {}", err);
      std::process::exit(1);
    });
    let sb = Superblock::new(img).unwrap_or_else(|err| {
      eprintln!("Superblock error has occured: {}", err);
      std::process::exit(1);
    });

    if let Some(err) = sb.check_signature() {
      eprintln!("{}", err);
      eprintln!("This dump information may not be accurate.");
    }

    print!("{}", sb);
  }

  fn read_img(&self) -> io::Result<File>
  {
    let mut img = File::open(self.path.as_path())?;
    img.seek(SeekFrom::Start(1024))?;
    Ok(img)
  }
}
