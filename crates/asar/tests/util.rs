use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use anyhow::Result;
use dircmp::Comparison;

pub fn resolve<T: AsRef<Path>>(p: T) -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p)
}

pub fn comp_file<T: AsRef<Path>, U: AsRef<Path>>(a: T, b: U) -> Result<bool> {
  let mut fa = std::fs::File::open(&a)?;
  let mut fb = std::fs::File::open(&b)?;
  let mut hasher_a = Sha256::new();
  let mut hasher_b = Sha256::new();
  let _ = std::io::copy(&mut fa, &mut hasher_a)?;
  let _ = std::io::copy(&mut fb, &mut hasher_b)?;
  Ok(hasher_a.finalize() == hasher_b.finalize())
}

pub fn comp_dir<T: AsRef<Path>, U: AsRef<Path>>(a: T, b: U) -> Result<bool> {
  let cmp = Comparison::default();
  let diff = cmp.compare(a.as_ref(), b.as_ref())?;
  Ok(diff.is_empty())
}
