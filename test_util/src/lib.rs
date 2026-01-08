//! Utility to gather vanilla raws to use for testing.
use dfraw_parser::metadata::{ParserOptions, RawModuleLocation};
use dfraw_parser::parse;
use dfraw_parser_sqlite_lib::{ClientOptions, DbClient};
use std::fs::{self, File};
use std::io::{self, Cursor};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use zip::ZipArchive;

const VANILLA_RAW_URL: &str = "https://build-deps.ci.nwest.one/dwarffortress/vanilla_latest.zip";
const TEST_DATA_DIR: &str = "test-data";
const TEST_INNER_DIR: &str = "data/vanilla";
const TEST_DB_NAME: &str = "test.db";

// We store a Result so that tests can check if setup worked.
// We use Arc so multiple tests can own a reference to the client.
static SHARED_CLIENT: OnceLock<Result<Arc<Mutex<DbClient>>, String>> = OnceLock::new();

/// Get a shared test dbclient that is only initialized once
///
/// # Panics
///
/// Will panic if some part of setting up the test database errors out
pub fn get_test_client() -> Arc<Mutex<DbClient>> {
    // get_or_init ensures the setup runs exactly once
    let result = SHARED_CLIENT.get_or_init(|| {
        // Setup test data
        let vanilla_path = ensure_vanilla_raws();

        // Initialize the DbClient
        let options = ClientOptions {
            reset_database: true,
            overwrite_raws: true,
        };

        let mut client =
            DbClient::init_db(TEST_DB_NAME, options).map_err(|e| format!("DB Init Error: {e}"))?;

        // Parse and Insert
        let mut parser_options = ParserOptions::default();
        parser_options.add_location_to_parse(RawModuleLocation::Vanilla);
        parser_options.set_dwarf_fortress_directory(&vanilla_path);

        let parse_results = parse(&parser_options).map_err(|e| format!("Parse Error: {e}"))?;
        let num_info_files = parse_results.info_files.len();

        client
            .insert_parse_results(parse_results)
            .map_err(|e| format!("DB Insert Error: {e}"))?;

        println!("Sucessfully inserted {num_info_files} modules.");
        Ok(Arc::new(Mutex::new(client)))
    });

    match result {
        Ok(client_mutex) => Arc::clone(client_mutex),
        Err(e) => panic!("Global test setup failed: {e}"),
    }
}

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
