//! Repository conventions, checked by `cargo test` (and by `npm test`).
//!
//! These tests are cheap and keep the exercise set consistent: every binary
//! in `src/bin/` must follow the `exNN.rs` naming, contain its own test
//! module, and have a guide in `docs/exercises/`.

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn exercise_files() -> Vec<(String, String)> {
    let bin_dir = root().join("src").join("bin");
    fs::read_dir(bin_dir)
        .expect("src/bin/ exists")
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            let stem = name.strip_suffix(".rs")?.to_string();
            if stem.is_empty() || stem == ".gitkeep" {
                return None;
            }
            let source = fs::read_to_string(entry.path()).expect("exercise file is readable");
            Some((stem, source))
        })
        .collect()
}

fn all_guides() -> String {
    let docs_dir = root().join("docs").join("exercises");
    fs::read_dir(docs_dir)
        .expect("docs/exercises/ exists")
        .flatten()
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            name.ends_with(".md") && name != "README.md"
        })
        .map(|entry| fs::read_to_string(entry.path()).expect("guide is readable"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn package_is_the_exercise_workspace() {
    let manifest = fs::read_to_string(root().join("Cargo.toml")).expect("Cargo.toml is readable");
    assert!(
        manifest.contains("name = \"rust-exercise\""),
        "the package must be named rust-exercise"
    );
}

#[test]
fn target_directory_is_ignored() {
    let gitignore = fs::read_to_string(root().join(".gitignore")).expect(".gitignore is readable");
    assert!(
        gitignore.contains("/target"),
        ".gitignore must ignore /target"
    );
}

#[test]
fn exercises_follow_the_naming_convention() {
    for (stem, _) in exercise_files() {
        let valid = stem.len() == 4
            && stem.starts_with("ex")
            && stem[2..].chars().all(|c| c.is_ascii_digit());

        assert!(
            valid,
            "src/bin/{stem}.rs must be named exNN.rs (for example ex01.rs)"
        );
    }
}

#[test]
fn every_exercise_has_a_test() {
    for (stem, source) in exercise_files() {
        assert!(
            source.contains("mod tests"),
            "src/bin/{stem}.rs must contain a `#[cfg(test)] mod tests` block"
        );
        assert!(
            source.contains("#[test]"),
            "src/bin/{stem}.rs must contain at least one #[test]"
        );
    }
}

#[test]
fn every_exercise_has_a_guide() {
    let guides = all_guides();

    for (stem, _) in exercise_files() {
        let number = &stem[2..4];
        assert!(
            guides.contains(&format!("Ex {number} —")),
            "src/bin/{stem}.rs has no guide: expected \"Ex {number} —\" in docs/exercises/"
        );
    }
}
