use std::path::Path;

mod asar;
mod crawlfs;
mod disk;
pub mod error;
pub mod ffi;
mod filesystem;
mod integrity;
pub mod node;

pub use crate::asar::*;
use error::Result;

pub fn get_raw_header<T: AsRef<Path>>(archive: T) -> Result<(String, node::Node, usize)> {
  let (header, json_value, size, _) = disk::read_archive_header(archive)?;
  Ok((header, json_value, size))
}

pub fn stat_file<T: AsRef<Path>>(
  archive: T,
  filename: &str,
  follow_links: Option<bool>,
) -> Result<node::Node> {
  let mut asar = AsarFile::open(archive)?;
  Ok(asar.stat_file(filename, follow_links)?.clone())
}

pub fn list_package<T: AsRef<Path>>(archive: T) -> error::Result<Vec<String>> {
  list_package_with_options(archive, &ListOptions::new())
}

pub fn list_package_with_options<T: AsRef<Path>>(
  archive: T,
  options: &ListOptions,
) -> error::Result<Vec<String>> {
  let asar = AsarFile::open(archive)?;
  asar.list(options)
}

pub fn extract_file<T: AsRef<Path>>(archive: T, filename: &str) -> error::Result<Vec<u8>> {
  let mut asar = AsarFile::open(archive)?;
  Ok(asar.read_file(filename)?)
}

pub fn extract_all<T: AsRef<Path>, U: AsRef<Path>>(archive: T, dest: U) -> error::Result<()> {
  let mut asar = AsarFile::open(archive)?;
  Ok(asar.extract_all(dest)?)
}
