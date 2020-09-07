use recover::ext4;
use std::io::{Seek, SeekFrom};

fn read_img<P: AsRef<std::path::Path>>(path: P) -> Result<std::fs::File, std::io::Error>
{
  let mut img = std::fs::File::open(path)?;
  img.seek(SeekFrom::Start(1024))?;
  Ok(img)
}

fn main()
{
  println!("Recover");

  let img = read_img("test/test.img").unwrap_or_else(|err| {
    eprintln!("An IO error occurred while reading the volume: {}", err);
    std::process::exit(1);
  });
  let sb = ext4::Superblock::new(img).unwrap_or_else(|err| {
    eprintln!("{}", err);
    std::process::exit(1);
  });

  println!("\n{}\n", sb);
  println!("block size: {}", sb.block_size());
  println!("cluseter size: {}", sb.cluster_size());
}
