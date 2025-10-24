//! Integration test that verifies parsing of successive XML files

use iso10383_parser::MicList;
use quick_xml::de;
use std::{fs::File, io::BufReader, path::PathBuf};

const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");
const TESTS_DIR: &str = "tests";

#[yare::parameterized(
    feb2025 = {"2025-02-10.xml", 2733},
    mar2025 = {"2025-03-10.xml", 2745},
    may2025 = {"2025-05-12.xml", 2767},
    jun2025 = {"2025-06-10.xml", 2776},
    aug2025 = {"2025-08-11.xml", 2794},
    sep2025 = {"2025-09-08.xml", 2796},
    oct2025 = {"2025-10-13.xml", 2798}
)]
fn all_historical(file: &str, count: usize) {
    let mut filepath = PathBuf::from(BASE_PATH);
    filepath.push(TESTS_DIR);
    filepath.push(file);

    let handle = File::open(&filepath).expect("Could not create XML Reader");
    let reader = BufReader::new(handle);

    let output: MicList = de::from_reader(reader).expect("Could not parse XML");

    assert_eq!(count, output.len());
}
