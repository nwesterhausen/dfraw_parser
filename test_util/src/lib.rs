//! Utility to gather vanilla raws to use for testing.
use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::PathBuf;
use zip::ZipArchive;

const VANILLA_RAW_URL: &str = "https://build-deps.ci.nwest.one/dwarffortress/vanilla_latest.zip";
const TEST_DATA_DIR: &str = "test-data";
const TEST_INNER_DIR: &str = "data/vanilla";

/// Ensures the vanilla raw files are available for testing.
/// Returns the path to the directory containing the raws.
///
/// # Panics
///
/// This will panic if failed to download, unzip, or create the folder/files.
///
/// # Returns
///
/// The analog for the DF dir
#[must_use]
pub fn ensure_vanilla_raws() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let Some(root_dir) = manifest_dir.parent() else {
        panic!("Unable to grab correct directory");
    };
    let test_data_dir = root_dir.join(TEST_DATA_DIR);
    let target_dir = test_data_dir.join(TEST_INNER_DIR);

    if target_dir.exists() {
        return test_data_dir;
    }

    println!("Downloading vanilla raws for testing...");
    let response = reqwest::blocking::get(VANILLA_RAW_URL)
        .expect("Failed to download vanilla raws")
        .bytes()
        .expect("Failed to read response bytes");

    println!("Extracting vanilla raws...");
    let mut archive = ZipArchive::new(Cursor::new(response)).expect("Failed to parse zip archive");

    fs::create_dir_all(&target_dir).expect("Failed to create target directory");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failure to open the zip file.");
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).expect("Failure to create output directory");
        } else {
            if let Some(p) = outpath.parent()
                && !p.exists()
            {
                fs::create_dir_all(p).expect("Failure to create output directory");
            }
            let mut outfile = File::create(&outpath).expect("Failure to create output file");
            io::copy(&mut file, &mut outfile).expect("Failure to copy from zip to output file");
        }
    }

    test_data_dir
}
