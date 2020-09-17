use chrono::{DateTime, TimeZone, Utc};

#[macro_export]
macro_rules! add_to_list {
  ($self:ident, $list:ident, $item:expr, $flag_name:ident) => {
    if $self.contains(Self::$flag_name) {
      $list.push($item);
    }
  };
}

#[inline(always)]
pub fn get_string_list<'a>(list: &'a Vec<&str>) -> String
{
  if list.len() == 0 {
    String::from("(none)")
  } else {
    list.join(" ")
  }
}

#[inline(always)]
pub fn get_string(string: &str) -> &str
{
  if string.bytes().nth(0).unwrap_or(0) == 0 {
    "<not available>"
  } else {
    string
  }
}

#[inline(always)]
pub fn get_datetime(datetime: DateTime<Utc>) -> String
{
  if datetime == Utc.timestamp(0, 0) {
    String::from("n/a")
  } else {
    datetime.format("%a %b %d %T %Y").to_string()
  }
}

#[inline(always)]
pub fn kbytes_to_human_readable(kbytes: u64) -> String
{
  if kbytes < 2u64.pow(13) {
    format!("{} kB", kbytes)
  } else if kbytes < 2u64.pow(23) {
    format!("{} MB", (kbytes + 2u64.pow(9)) >> 10)
  } else if kbytes < 2u64.pow(33) {
    format!("{} GB", (kbytes + 2u64.pow(19)) >> 20)
  } else if kbytes < 2u64.pow(43) {
    format!("{} TB", (kbytes + 2u64.pow(29)) >> 30)
  } else {
    format!("{} PB", (kbytes + 2u64.pow(39)) >> 40)
  }
}

#[cfg(target_family = "unix")]
#[inline(always)]
pub fn get_user(uid: u16) -> String
{
  use std::ffi::CString;
  let pw: *mut libc::passwd = unsafe { libc::getpwuid(uid as u32) };
  format!(
    "{} (user {})",
    uid,
    if pw.is_null() {
      String::from("unknown")
    } else {
      unsafe { CString::from_raw((*pw).pw_name) }
        .into_string()
        .unwrap_or(String::from("unknown"))
    }
  )
}

#[cfg(target_family = "unix")]
#[inline(always)]
pub fn get_group(gid: u16) -> String
{
  use std::ffi::CString;
  let gr: *mut libc::group = unsafe { libc::getgrgid(gid as u32) };
  format!(
    "{} (group {})",
    gid,
    if gr.is_null() {
      String::from("unknown")
    } else {
      unsafe { CString::from_raw((*gr).gr_name) }
        .into_string()
        .unwrap_or(String::from("unknown"))
    }
  )
}

#[cfg(target_family = "windows")]
#[inline(always)]
pub fn get_user(uid: u16) -> String
{
  format!(
    "{} (user {})",
    uid,
    if uid == 0 { "root" } else { "unknown" }
  )
}

#[cfg(target_family = "windows")]
#[inline(always)]
pub fn get_group(gid: u16) -> String
{
  format!(
    "{} (group {})",
    gid,
    if gid == 0 { "root" } else { "unknown" }
  )
}
