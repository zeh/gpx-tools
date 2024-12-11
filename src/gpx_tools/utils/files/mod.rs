use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use expanduser::expanduser;
use glob::glob;
use gpx::{Gpx, read};

/// Based on a file location/mask, returns the path for all files found
pub fn get_files_from_mask(mask: &str) -> Result<Vec<PathBuf>, &str> {
	let files = glob(expanduser(mask).expect("Cannot expand mask").to_str().unwrap()); // .expect("Failed to read glob pattern")
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
		None => Ok(results.iter().map(|m| m.clone().unwrap()).flatten().collect::<Vec<PathBuf>>()),
	}
}

/// Given a file path, read that as a GPX
pub fn read_gpx_from_file(filename: &PathBuf) -> Gpx {
    let data = File::open(filename).unwrap();
    let reader = BufReader::new(data);
    read(reader).unwrap()
}

/// Given a list of file paths, read that as a list of GPX
pub fn read_gpx_from_files(filenames: &Vec<PathBuf>) -> Vec<Gpx> {
    filenames.iter().map(|f| read_gpx_from_file(f)).collect::<Vec<Gpx>>()
}
