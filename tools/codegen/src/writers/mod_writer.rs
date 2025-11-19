use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn write(path_dir: &str, module_name: &str) -> std::io::Result<()> {
    let mod_path = format!("{}/mod.rs", path_dir);
    let mod_file = Path::new(&mod_path);

    // Ensure directory exists
    if let Some(parent) = mod_file.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create file if missing
    if !mod_file.exists() {
        fs::write(&mod_path, "")?;
    }

    // Read file
    let mut contents = String::new();
    {
        let mut file = OpenOptions::new().read(true).open(&mod_path)?;
        file.read_to_string(&mut contents)?;
    }

    // Already contains module
    let mod_line = format!("pub mod {};", module_name);
    if contents.contains(&mod_line) {
        return Ok(());
    }

    // Append line
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&mod_path)?;
    writeln!(file, "pub mod {};", module_name)?;

    Ok(())
}
