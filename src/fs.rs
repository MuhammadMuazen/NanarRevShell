use std::io::Write;

pub fn get_current_dir() -> String {
    
    let current_path: String = std::env::current_dir()
        .expect("Failed to get current directory")
        .to_str().expect("Path is not valid UTF-8").to_string();

    current_path
}

pub fn set_current_path(wanted_path: &str) -> std::io::Result<()> {

    let requested_path: &std::path::Path = std::path::Path::new(wanted_path);

    std::env::set_current_dir(&requested_path)?;

    Ok(())
} 

// read file
pub fn read_file_content(file_path_str: &str) -> String {

    let file_content: String = std::fs::read_to_string(file_path_str).expect("File Content");

    file_content
}

// write file
pub fn write_to_file(file_path_str: &str, content: &str) -> std::io::Result<()> {
    
    let mut file: std::fs::File = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path_str)?;
    
    file.write_all(content.as_bytes())?;
    
    Ok(())
}

// Remvoe file
pub fn remove_file(file_path_str: &str) -> std::io::Result<()> {

    std::fs::remove_file(file_path_str)?;

    Ok(())
}

// Creart directory
pub fn create_dir(dir_path_str: &str) -> std::io::Result<()> {
    
    std::fs::create_dir(dir_path_str)?;

    Ok(())
}

// Remove directory
pub fn remove_dir(dir_path_str: &str) -> std::io::Result<()> {

    std::fs::remove_dir(dir_path_str)?;

    Ok(())
}

// copy file or dir
pub fn copy_file_dir(source_path_str: &str, destination_path_str: &str) -> std::io::Result<()> {

    let source_path: &std::path::Path= std::path::Path::new(&source_path_str);
    let destination_path: &std::path::Path = std::path::Path::new(&destination_path_str);
    
    if source_path.is_dir() {
        
        // Create destination directory if it doesn't exist
        std::fs::create_dir_all(destination_path)?;
        
        // Recursively copy directory contents
        for entry in std::fs::read_dir(source_path)? {
            
            let entry: std::fs::DirEntry = entry?;
            let entry_path: std::path::PathBuf = entry.path();
            let dest_path: std::path::PathBuf = destination_path.join(entry.file_name());
            
            if entry_path.is_dir() {
                copy_file_dir(
                    entry_path.to_str().unwrap(), dest_path.to_str().unwrap())?;
            } else {
                std::fs::copy(entry_path, dest_path)?;
            }
        }
    } else {

        // If source is a file, ensure parent directory exists
        if let Some(parent) = destination_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::copy(source_path, destination_path)?;
    }

    Ok(())
}

// move file or dir
pub fn move_file_dir(source_path_str: &str, destination_path_str: &str) -> std::io::Result<()> {
    
    let source_path: &std::path::Path = std::path::Path::new(source_path_str);
    let destination_path: &std::path::Path = std::path::Path::new(destination_path_str);

    // First rename
    match std::fs::rename(source_path, destination_path) {
        Ok(_) => return Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::CrossesDevices => {}
        Err(e) => return Err(e),
    }

    // Copy then delete (for cross-filesystem moves)
    copy_file_dir(source_path_str, destination_path_str)?;

    // Remove the original
    if source_path.is_dir() {
        std::fs::remove_dir_all(source_path)?;
    } else {
        std::fs::remove_file(source_path)?;
    }

    Ok(())
}