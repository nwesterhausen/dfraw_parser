use dfraw_json_parser::generate_bindings;

#[test]
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn generate_ts_bindings() {
    // get our current working directory
    let cwd = std::env::current_dir().expect("Failed to get current working directory");
    // set lib/bindings/AllBindings.d.ts as the output file
    let output_dir = cwd.join("bindings");
    // make sure output dir exists
    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    eprintln!("Output dir: {:?}", &output_dir);

    generate_bindings(&output_dir.join("DFRawParser.d.ts")).expect("Saving bindings failed");
}
