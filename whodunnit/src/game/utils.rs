use std::fs::File;
use std::path::Path;

pub fn open_or_create_file_from_path(path: &Path) -> Result<File, std::io::Error> {
    let file;
    if !path.exists() {
        println!("path not found, file {:?} created", path);
        file =  File::create_new(path)?;
    } else {
        println!("file {:?} exists, opening", path);
        file = File::open(path)?;
    }
    Ok(file)
}
