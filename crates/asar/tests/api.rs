use std::ffi::OsStr;
use std::fs;

mod util;

use util::{comp_dir, comp_file, resolve};

use anyhow::Result;
use asar_rs::*;

#[test]
pub fn should_create_archive_from_directory() -> Result<()> {
  let out = resolve("tmp/packthis-api.asar");
  create_package(resolve("tests/input/packthis"), &out)?;
  assert!(comp_file(&out, resolve("tests/expected/packthis.asar"))?);
  Ok(())
}

#[test]
pub fn should_create_archive_from_directory_with_link() -> Result<()> {
  let out = resolve("tmp/packthis-api-link.asar");
  create_package(resolve("tests/input/packthis-link"), &out)?;
  if cfg!(windows) {
    assert!(comp_file(
      &out,
      resolve("tests/expected/packthis-link-win.asar")
    )?);
  } else {
    assert!(comp_file(
      &out,
      resolve("tests/expected/packthis-link-unix.asar")
    )?);
  }
  Ok(())
}

#[test]
pub fn should_create_archive_from_directory_without_hidden_files() -> Result<()> {
  let out = resolve("tmp/packthis-without-hidden-api.asar");
  let mut options = CreateOptions::new();
  options.dot = Some(false);
  create_package_with_options(resolve("tests/input/packthis"), &out, &options)?;
  assert!(comp_file(
    &out,
    resolve("tests/expected/packthis-without-hidden.asar")
  )?);
  Ok(())
}

#[test]
pub fn should_create_archive_from_directory_with_transformed_files() -> Result<()> {
  let out = resolve("tmp/packthis-api-transformed.asar");
  let mut options = CreateOptions::new();

  struct Reverser {
    flushed: bool,
    data: String,
  }

  impl std::io::Read for Reverser {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
      if self.flushed {
        let ret = self.data.as_bytes();
        let len = ret.len();
        buf[0..len].copy_from_slice(&ret[0..]);
        Ok(len)
      } else {
        Ok(0)
      }
    }
  }

  impl std::io::Write for Reverser {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
      self.data += &String::from_utf8(buf.to_vec()).unwrap();
      Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
      self.data = self.data.chars().rev().collect();
      self.flushed = true;
      Ok(())
    }
  }

  impl Transform for Reverser {}

  options.transform = Some(|filename| -> Option<Box<dyn Transform>> {
    if std::path::Path::new(filename).file_name().unwrap() == OsStr::new("file0.txt") {
      return Some(Box::new(Reverser {
        flushed: false,
        data: "".to_owned(),
      }));
    }
    None
  });
  create_package_with_options(resolve("tests/input/packthis"), &out, &options)?;
  assert!(comp_file(
    &out,
    resolve("tests/expected/packthis-transformed.asar")
  )?);
  Ok(())
}

#[test]
pub fn should_create_archive_from_directory_with_nothing_packed() -> Result<()> {
  let out = resolve("tmp/packthis-api-unpacked.asar");
  let mut options = CreateOptions::new();
  options.unpack_dir = Some("**".to_owned());
  create_package_with_options(resolve("tests/input/packthis"), &out, &options)?;
  assert!(comp_file(
    &out,
    resolve("tests/expected/packthis-all-unpacked.asar")
  )?);
  Ok(())
}

#[test]
pub fn list_files_in_archive() -> Result<()> {
  let list: Vec<String> = list_package(resolve("tests/input/extractthis.asar"))?;
  let filelist_content = fs::read_to_string(resolve("tests/expected/extractthis-filelist.txt"))?;
  #[cfg(target_os = "windows")]
  let filelist_content = filelist_content.replace("/", "\\");
  assert_eq!(
    list,
    filelist_content
      .lines()
      .map(|line| line.to_owned())
      .collect::<Vec<String>>()
  );
  Ok(())
}

#[test]
pub fn list_files_in_archive_with_option() -> Result<()> {
  let list = list_package_with_options(
    resolve("tests/input/extractthis-unpack-dir.asar"),
    &ListOptions { is_pack: true },
  )?;
  let filelist_content = fs::read_to_string(resolve(
    "tests/expected/extractthis-filelist-with-option.txt",
  ))?;
  #[cfg(target_os = "windows")]
  let filelist_content = filelist_content.replace("/", "\\");
  assert_eq!(
    list,
    filelist_content
      .lines()
      .map(|line| line.to_owned())
      .collect::<Vec<String>>()
  );
  Ok(())
}

#[test]
pub fn should_extract_a_text_file_from_archive() -> Result<()> {
  let actual = String::from_utf8(extract_file(
    resolve("tests/input/extractthis.asar"),
    "dir1/file1.txt",
  )?)?;
  let expected = fs::read_to_string(resolve("tests/expected/extractthis/dir1/file1.txt"))?;
  assert_eq!(actual, expected);
  Ok(())
}

#[test]
pub fn should_extract_a_binary_file_from_archive() -> Result<()> {
  let actual = extract_file(resolve("tests/input/extractthis.asar"), "dir2/file2.png")?;
  let expected = fs::read(resolve("tests/expected/extractthis/dir2/file2.png"))?;
  assert_eq!(actual, expected);
  Ok(())
}

#[test]
pub fn should_extract_a_binary_file_from_archive_with_unpacked_files() -> Result<()> {
  let actual = extract_file(
    resolve("tests/input/extractthis-unpack.asar"),
    "dir2/file2.png",
  )?;
  let expected = fs::read(resolve("tests/expected/extractthis/dir2/file2.png"))?;
  assert_eq!(actual, expected);
  Ok(())
}

#[test]
pub fn should_extract_an_archive() -> Result<()> {
  let out = resolve("tmp/extractthis-api");
  extract_all(resolve("tests/input/extractthis.asar"), &out)?;
  comp_dir(&out, resolve("tests/expected/extractthis"))?;
  Ok(())
}

#[test]
pub fn should_extract_an_archive_with_unpacked_files() -> Result<()> {
  let out = resolve("tmp/extractthis-unpack-api");
  extract_all(resolve("tests/input/extractthis-unpack.asar"), &out)?;
  comp_dir(&out, resolve("tests/expected/extractthis"))?;
  Ok(())
}

#[test]
pub fn should_extract_a_text_file_from_archive_with_unpacked_files() -> Result<()> {
  let actual = extract_file(
    resolve("tests/input/extractthis-unpack-dir.asar"),
    "dir1/file1.txt",
  )?;
  let expected = fs::read(resolve("tests/expected/extractthis/dir1/file1.txt"))?;
  assert_eq!(actual, expected);
  Ok(())
}

#[test]
pub fn should_extract_an_archive_with_unpacked_dirs() -> Result<()> {
  let out = resolve("tmp/extractthis-unpack-dir-api");
  extract_all(resolve("tests/input/extractthis-unpack-dir.asar"), &out)?;
  comp_dir(&out, resolve("tests/expected/extractthis"))?;
  Ok(())
}

#[test]
pub fn should_handle_multibyte_characters_in_paths() -> Result<()> {
  let out = resolve("tmp/packthis-unicode-path.asar");
  let options = CreateOptions::new();
  create_package_with_options(resolve("tests/input/packthis-unicode-path"), &out, &options)?;
  assert!(comp_file(
    &out,
    resolve("tests/expected/packthis-unicode-path.asar")
  )?);
  Ok(())
}

#[test]
pub fn should_extract_a_text_file_from_archive_with_multibyte_characters_in_path() -> Result<()> {
  let actual = String::from_utf8(extract_file(
    resolve("tests/expected/packthis-unicode-path.asar"),
    "dir1/女の子.txt",
  )?)?;
  let expected = fs::read_to_string(resolve("tests/input/packthis-unicode-path/dir1/女の子.txt"))?;
  assert_eq!(actual, expected);
  Ok(())
}

#[test]
pub fn should_create_files_or_directories_whose_names_are_properties_of_object_prototype() -> Result<()> {
  create_package("tests/input/packthis-object-prototype/", "tmp/packthis-object-prototype.asar")?;
  comp_file("tmp/packthis-object-prototype.asar", "tests/expected/packthis-object-prototype.asar")?;
  Ok(())
}

#[test]
pub fn should_extract_files_or_directories_whose_names_are_properties_of_object_prototype() -> Result<()> {
  extract_all("tests/expected/packthis-object-prototype.asar", "tmp/packthis-object-prototype/")?;
  comp_dir("tests/input/packthis-object-prototype/", "tmp/packthis-object-prototype")?;
  Ok(())
}
