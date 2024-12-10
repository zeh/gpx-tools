use std::path::PathBuf;
use glob::glob;

/// Based on a file location/mask, returns the path for all files found
pub fn get_files_from_mask(mask: &str) -> Result<Vec<PathBuf>, &str> {
    let files = glob(mask);// .expect("Failed to read glob pattern")
    match files {
        Ok(files) => Ok(files.map(|f| f.unwrap().as_path().to_owned()).collect::<Vec<PathBuf>>()),
        Err(_) => Err("Failed to read glob pattern"),
    }
}

/// Based on file locations/mask, returns the path for all files found on each
pub fn get_files_from_masks(masks: &Vec<String>) -> Result<Vec<PathBuf>, &str> {
    let results = masks.iter().map(|m| get_files_from_mask(m)).collect::<Vec<Result<Vec<PathBuf>, &str>>>();
    match results.iter().find(|m| m.is_err()) {
        Some(m) => Err(m.clone().err().unwrap()),
        None => {
            Ok(results.iter().map(|m| m.clone().unwrap()).flatten().collect::<Vec<PathBuf>>())
        }
    }
}
