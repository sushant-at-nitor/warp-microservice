use std::fs;
use std::path::Path;

/// Write file, creating directories automatically.
/// If file exists, overwrite.
pub fn write_file(path: &str, content: &str) -> std::io::Result<()> {
  let path_obj = Path::new(path);

  if let Some(parent) = path_obj.parent() {
    fs::create_dir_all(parent)?;
  }

  fs::write(path_obj, content)?;
  Ok(())
}
