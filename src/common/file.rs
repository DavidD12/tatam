use std::path::Path;
use std::{fs, io};

// Fonction pour copier un fichier d'un emplacement source vers une destination
pub fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    fs::copy(src, dest)?;
    Ok(())
}

// Fonction pour copier un répertoire et son contenu récursivement
pub fn copy_directory(src: &Path, dest: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "src must be a folder"));
    }

    if !dest.exists() {
        fs::create_dir(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            copy_directory(&entry_path, &dest_path)?;
        } else {
            copy_file(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}
