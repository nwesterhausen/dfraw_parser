use encoding_rs_io::DecodeReaderBytesBuilder;
use tracing::{error, trace};

use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::constants::DF_ENCODING;
use crate::metadata::{ObjectType, OBJECT_TOKEN_MAP};
use crate::regex::RAW_TOKEN_RE;
use crate::utilities::try_get_file;
use crate::ParserError;

/// It reads a file, line by line, and checks the first line for the filename, reads lines until it encounters the
/// \[OBJECT:(type)] tag in the file.
///
/// Arguments:
///
/// * `input_path`: Path to the file to be read
/// * `module_info`: Information about the raw module `input_path` is within
///
/// Returns:
///
/// `RawObjectKind` for the type of \[OBJECT\] tag encountered, and `RawObjectKind::None` if it is unsupported.
#[allow(clippy::too_many_lines)]
#[allow(dead_code)]
pub fn read_raw_file_type<P: AsRef<Path>>(input_path: &P) -> Result<ObjectType, ParserError> {
    // Open the file
    let file = try_get_file(input_path)?;

    // Setup a file reader for the encoding used by DF
    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(*DF_ENCODING))
        .build(file);
    let reader = BufReader::new(decoding_reader);

    // String to store the parsed filename in
    let mut raw_filename = String::new();

    // Read in lines until we encounter the \[OBJECT tag\] or complete the file.
    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            error!(
                "read_raw_file_type: Error processing {}:{}",
                input_path.as_ref().display(),
                index
            );
            continue;
        }
        // Match the line so we can check the pieces of it
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                error!("read_raw_file_type: Line-reading error\n{:?}", e);
                continue;
            }
        };
        // The filename is always the top line of a DF raw file
        if index == 0 {
            raw_filename = String::from(&line);
            continue;
        }
        // Multiple matches can occur in a single line, so we loop over all captures within the match
        // for this line.
        for cap in RAW_TOKEN_RE.captures_iter(&line) {
            let captured_key = match cap.get(2) {
                Some(v) => v.as_str(),
                _ => {
                    continue;
                }
            };
            let captured_value = match cap.get(3) {
                Some(v) => v.as_str(),
                _ => {
                    continue;
                }
            };

            trace!(
                "read_raw_file_type: Key: {} Value: {}",
                captured_key,
                captured_value
            );

            // Match the front part of the tag (right now we only want the \[OBJECT\] key)
            // If we need to check for more later, use a match statment instead of this if.
            if captured_key == "OBJECT" {
                trace!(
                    "read_raw_file_type: {} is a {} raw file",
                    raw_filename,
                    captured_value
                );
                return Ok(OBJECT_TOKEN_MAP
                    .get(captured_value)
                    .cloned()
                    .unwrap_or_default());
            }
        }
    }

    // Reading through the entire file and not finding an \[OBJECT\] tag means the raw file is invalid
    Err(ParserError::InvalidRawFile(
        "No [OBJECT] tag found".to_string(),
    ))
}
