use const_gen::{CompileConst, const_declaration};

use std::collections::HashMap;
use std::io::Write;

const IN_FILE_WINDOWS_11: &str = "maps/windows/windows_11/map-release-to-build.json";
const OUT_FILE_WINDOWS_11: &str = "windows_11_build_to_release_map.rs";

const IN_FILE_WINDOWS_10: &str = "maps/windows/windows_10/map-release-to-build.json";
const OUT_FILE_WINDOWS_10: &str = "windows_10_build_to_release_map.rs";


fn main() -> Result<(), serde_json::Error> {
    // Run if build.rs has changed
    println!("cargo:rerun-if-changed=build.rs");
    // Run if input file has changed
    println!("cargo:rerun-if-changed={}", IN_FILE_WINDOWS_11);
    println!("cargo:rerun-if-changed={}", IN_FILE_WINDOWS_10);

    let _ = run(IN_FILE_WINDOWS_11, OUT_FILE_WINDOWS_11);
    let _ = run(IN_FILE_WINDOWS_10, OUT_FILE_WINDOWS_10);

    Ok(())
}

fn run(input: &str, output: &str) -> Result<(), serde_json::Error> {
    // Create output file
    let out_dir = std::env::var("OUT_DIR")
        .expect("Error: env variable OUT_DIR not set.");
    let out_dest = format!("{}/{}", out_dir, output);
    let out_file = std::path::Path::new(&out_dest);
    let mut out_file_handle = std::fs::File::create(&out_file)
        .expect(&format!("Error: unable to create file \"{}\" to write output to", out_dest));

    // Open input file
    let in_file_handle = std::fs::File::open(input)
        .expect(&format!("Error: unable to open file \"{}\" to read input from", input));
    let reader = std::io::BufReader::new(in_file_handle);

    // Deserialize input
    let release_to_builds: HashMap<String, Vec<String>> = serde_json::from_reader(reader)?;

    // Turn release to builds map, into a build to release map
    let build_to_release = release_to_builds.iter()
        .flat_map(|(os, versions)| {
            versions.iter().map(move |version| (version.as_str(), os.as_str()))
        })
        .fold(HashMap::new(), |mut acc, (version, os)| {
            acc.entry(version).or_insert_with(Vec::new).push(os);
            acc
        });

    let release_pattern = format!("{}", release_to_builds.keys().map(|s| s.as_str()).collect::<Vec<_>>().join("|"));

    // Create const declarations
    let const_declarations = vec!
    {
        const_declaration!(BUILD_TO_RELEASE_MAP = build_to_release),
        const_declaration!(RELEASE_PATTERN = release_pattern),
    }.join("\n");

    // Lastly, write to output file
    out_file_handle.write_all(const_declarations.as_bytes())
        .expect(&format!("Error: write to output file \"{}\" failed", out_dest));

    Ok(())
}
