use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use dfraw_parser::traits::RawObject;
use tracing::{error, info, warn};

/// Save a vector of parsed raw objects to a file in JSON format.
///
/// Arguments:
///
/// * `raws_vec`: A vector of boxed objects that implement the `RawObject` trait.
/// * `out_filepath`: A path to the output file.
/// * `pretty_print`: A boolean value indicating whether to pretty print the JSON output.
pub fn write_raw_vec_to_file<P: AsRef<Path>>(
    raws_vec: &Vec<Box<dyn RawObject>>,
    out_filepath: &P,
    pretty_print: bool,
) {
    info!(
        "write_raw_vec_to_file: Writing {} raws to file {:?}",
        raws_vec.len(),
        out_filepath.as_ref().display()
    );

    if raws_vec.is_empty() {
        warn!("write_raw_vec_to_file: Provided raw vector is empty!");
        return;
    }

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "write_raw_vec_to_file: Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    if pretty_print {
        serde_json::to_writer_pretty(out_file, raws_vec).unwrap_or_else(|e| {
            error!(
                "write_raw_vec_to_file: Unable to write to {} \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
        });
    } else {
        serde_json::to_writer(out_file, raws_vec).unwrap_or_else(|e| {
            error!(
                "write_raw_vec_to_file: Unable to write to {} \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
        });
    }
}

/// Save a vector of strings to a file, one string per line.
///
/// Arguments:
///
/// * `parsed_raws_string_vec`: String
/// * `out_filepath`: Path
pub fn write_json_string_vec_to_file<P: AsRef<Path>>(strings_vec: &[String], out_filepath: &P) {
    info!(
        "write_json_string_vec_to_file: Writing {} strings to file {:?}",
        strings_vec.len(),
        out_filepath.as_ref().display()
    );

    if strings_vec.is_empty() {
        warn!("write_json_string_vec_to_file: Provided string vector is empty!");
        return;
    }

    let out_file = match File::create(out_filepath) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "write_json_string_vec_to_file: Unable to open {} for writing \n{:?}",
                out_filepath.as_ref().display(),
                e
            );
            return;
        }
    };

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!(
        "write_json_string_vec_to_file: Unable to write to {}",
        out_filepath.as_ref().to_string_lossy()
    );

    if strings_vec.len() == 1 {
        match writeln!(stream, "{}", strings_vec.first().unwrap_or(&String::new())) {
            Ok(_x) => (),
            Err(e) => {
                error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                return;
            }
        }
        match stream.flush() {
            Ok(_x) => (),
            Err(e) => {
                error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
            }
        }
        return;
    }

    let strings_vec = strings_vec.iter();
    // Write the first value with an open bracket '[' at the beginning
    // Write all next values with a comma ',' in front
    // Finish with a closing bracket ']'
    for (i, string) in strings_vec.enumerate() {
        match i {
            0 => match write!(stream, "[{string}") {
                Ok(_x) => (),
                Err(e) => {
                    error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
            _ => match write!(stream, ",{string}") {
                Ok(_x) => (),
                Err(e) => {
                    error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
                    return;
                }
            },
        }
    }

    match writeln!(stream, "]") {
        Ok(_x) => (),
        Err(e) => {
            error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
            return;
        }
    }
    match stream.flush() {
        Ok(_x) => (),
        Err(e) => {
            error!("write_json_string_vec_to_file: {}\n{:?}", write_error, e);
        }
    }
}

/// The function `raws_to_string` converts a vector of raw objects into a JSON string representation.
///
/// Arguments:
///
/// * `raws`: The `raws` parameter is a vector of `Box<dyn RawObject>`.
///
/// Returns:
///
/// The function `raws_to_string` returns a `String` that represents the input `Vec<Box<dyn RawObject>>`
/// as a JSON array.
pub fn raws_to_string(raws: Vec<Box<dyn RawObject>>) -> String {
    // It should be an array, so start with '[' character,
    // then add each raw object, separated by a comma.
    // Finally add the closing ']' character.
    // (The last item cannot have a comma before ']')
    let mut json = String::from('[');
    for raw in raws {
        json.push_str(serde_json::to_string(&raw).unwrap_or_default().as_str());
        json.push(',');
    }
    json.pop(); // remove trailing comma
    json.push(']');
    json
}
