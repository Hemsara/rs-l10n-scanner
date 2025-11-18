use std::env;
use std::process::Command;

pub fn run_dart_analyzer(file_path: &str) -> Vec<String> {
    // vector for all expected extracted strings
    let mut extracted_strings: Vec<String> = Vec::new();

    let dart_path = get_dart_path();
    let output = Command::new("dart")
        .arg("run")
        .arg(format!("{}/bin/extract_strings.dart", dart_path))
        .arg(file_path)
        .output()
        .expect("Failed to run Dart script");
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        for line in result.lines() {
            extracted_strings.push(line.to_string());
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Error running Dart script on {}:\n{}", file_path, error);
    }
    extracted_strings
}

fn get_dart_path() -> String {
    // Get the current executable path
    let exe_path = env::current_exe().expect("Failed to get current executable path");

    // Get the parent directory (where the executable is located)
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");

    // Go up to the project root and then to extract_strings
    let project_root = exe_dir
        .parent()
        .expect("Failed to get project root");

    let dart_path = project_root.join("extract_strings");

    dart_path
        .to_str()
        .expect("Failed to convert path to string")
        .to_string()
}
