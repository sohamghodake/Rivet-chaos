//! Every scenario in the repository parses and validates, so a typo in one fails
//! the tests rather than a run.

use std::{fs, path::Path};

use chaos_core::Scenario;

#[test]
fn every_scenario_file_is_valid() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenarios");
    let mut checked = 0;
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path
            .extension()
            .is_some_and(|extension| extension == "toml")
        {
            let text = fs::read_to_string(&path).unwrap();
            let scenario = Scenario::parse(&text)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
            assert_eq!(
                path.file_stem().unwrap(),
                scenario.name.as_str(),
                "{}",
                path.display()
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 9,
        "found only {checked} scenarios in {}",
        dir.display()
    );
}
