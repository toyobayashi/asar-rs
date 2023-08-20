use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::slice;

#[cfg(not(target_os = "windows"))]
use std::os::unix::ffi::OsStrExt;

use crate::error::ErrorStatus;

#[cfg(not(target_os = "windows"))]
fn cstr_to_path(cstr: &CStr) -> &Path {
  let osstr = std::ffi::OsStr::from_bytes(cstr.to_bytes());
  osstr.as_ref()
}

#[cfg(target_os = "windows")]
unsafe fn cstr_to_path(cstr: &CStr) -> &Path {
  let str = ::std::str::from_utf8_unchecked(cstr.to_bytes());
  str.as_ref()
}

#[no_mangle]
pub unsafe extern "C" fn asar_list_package(
  archive: *const c_char,
  buf: *mut c_char,
  buf_size: *mut usize,
  list: *mut *const c_char,
  list_len: *mut usize,
) -> ErrorStatus {
  if archive.is_null() || buf_size.is_null() || list_len.is_null() {
    return ErrorStatus::InvalidArg;
  }
  let result = crate::list_package(cstr_to_path(CStr::from_ptr(archive)));
  match result {
    Err(err) => err.status(),
    Ok(l) => {
      if buf.is_null() {
        let size = l
          .iter()
          .fold(0usize, |acc, e| return acc + e.as_bytes().len() + 1);
        *buf_size = size;
        *list_len = l.len();
        return ErrorStatus::Success;
      } else {
        let size = *buf_size;
        let buffer = slice::from_raw_parts_mut(buf as *mut u8, size);
        let list_slice = slice::from_raw_parts_mut(list, *list_len);
        let mut pos: usize = 0usize;
        for (index, item) in l.iter().enumerate() {
          list_slice[index] = (buf as usize + pos) as *const c_char;
          let strlen = item.as_bytes().len();
          let left = size - pos;
          if left <= strlen {
            buffer[pos..pos + left].copy_from_slice(&item.as_bytes()[0..left]);
            pos += left;
            break;
          } else {
            buffer[pos..pos + strlen].copy_from_slice(item.as_bytes());
            buffer[pos + strlen] = 0;
            pos += strlen + 1;
          }
        }
        *buf_size = pos;
        *list_len = l.len();
        return ErrorStatus::Success;
      }
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn asar_extract_all(
  archive: *const c_char,
  dest: *const c_char,
) -> ErrorStatus {
  if archive.is_null() || dest.is_null() {
    return ErrorStatus::InvalidArg;
  }
  match crate::extract_all(
    cstr_to_path(CStr::from_ptr(archive)),
    cstr_to_path(CStr::from_ptr(dest)),
  ) {
    Err(err) => err.status(),
    Ok(_) => ErrorStatus::Success,
  }
}

#[no_mangle]
pub unsafe extern "C" fn asar_create_package(
  archive: *const c_char,
  dest: *const c_char,
) -> ErrorStatus {
  if archive.is_null() || dest.is_null() {
    return ErrorStatus::InvalidArg;
  }
  match crate::create_package(
    cstr_to_path(CStr::from_ptr(archive)),
    cstr_to_path(CStr::from_ptr(dest)),
  ) {
    Err(err) => err.status(),
    Ok(_) => ErrorStatus::Success,
  }
}
