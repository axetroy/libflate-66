#![deny(warnings)]

use core::result::Result;
use std::fs::File;
use std::path::{Path, PathBuf};

use eyre::Report;
use tar::Archive;

pub fn extract(
    tar_file_path: &Path,
    extract_file_name: &str,
    dest_dir: &Path,
) -> Result<PathBuf, Report> {
    let output_file_path = dest_dir.join(extract_file_name);

    let tar_file = File::open(&tar_file_path)?;
    let mut archive = Archive::new(tar_file);

    archive.set_unpack_xattrs(true);
    archive.set_overwrite(true);
    archive.set_preserve_permissions(true);
    archive.set_preserve_mtime(true);

    let files = archive.entries()?;

    for entry in files {
        let mut file = entry?;

        let file_path = file.path()?;

        if let Some(file_name) = file_path.file_name() {
            if file_name.to_str().unwrap() == extract_file_name {
                file.unpack(&output_file_path)?;
                break;
            }
        }
    }

    Ok(output_file_path)
}
