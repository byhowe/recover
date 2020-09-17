use crate::ext4::{FileSystem, GroupDesc};
use std::io::{self, Seek, SeekFrom};

pub struct GroupDescIter<'fs, R>
{
  fs: &'fs mut FileSystem<R>,
  count: u32,
  idx: u32,
  first_desc_offset: u32,
}

impl<'fs, R> GroupDescIter<'fs, R>
{
  pub fn new(fs: &'fs mut FileSystem<R>) -> Self
  {
    let count: u32 = fs.sb.inodes_count / fs.sb.inodes_per_group;
    let block_size: u32 = fs.sb.get_block_size();
    let first_desc_offset: u32 = (1024 / block_size + 1) * block_size;
    Self {
      fs,
      count,
      idx: 0,
      first_desc_offset,
    }
  }
}

impl<R> Iterator for GroupDescIter<'_, R>
where
  R: io::Read + io::Seek,
{
  type Item = GroupDesc;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.idx == self.count {
      None
    } else {
      let group_desc_offset: u32 = self.first_desc_offset
        + self.idx
          * if self.fs.sb.feature_incompat.bit64 {
            64
          } else {
            32
          };
      self.idx += 1;
      if let Err(_) = self.fs.seek(SeekFrom::Start(group_desc_offset as u64)) {
        return None;
      }
      GroupDesc::new(&mut self.fs.inner, self.fs.sb.feature_incompat.bit64).ok()
    }
  }

  fn count(self) -> usize
  where
    Self: Sized,
  {
    self.count as usize
  }
}
