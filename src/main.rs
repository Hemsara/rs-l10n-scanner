mod analyzer;
mod exporter;
mod scanner;
mod validators;

use analyzer::run_dart_analyzer;
use exporter::export_json;
use scanner::dfs_traverse_dir;
use std::io;
use std::path;
use validators::check_path_exists;

fn main() {
    let mut dir_path = String::new();

    println!("🗂️Please enter the path of your flutter app to scan:");
    io::stdin()
        .read_line(&mut dir_path)
        .expect("Failed to read line");

    dir_path = dir_path.trim().to_string();

    if check_path_exists(&dir_path) {
        let mut dart_files = Vec::new();

        println!("✅The path is a valid Flutter app directory. Continuing...");
        let dir_path = path::Path::new(&dir_path).join("lib");
        dfs_traverse_dir(dir_path.to_str().unwrap().to_string(), &mut dart_files);

        let mut all_extracted_strings = Vec::new();
        println!("📄Found {} Dart files:", dart_files.len());
        for file in &dart_files {
            let extracted_strings = run_dart_analyzer(file);
            println!(
                " - {} strings extracted from {}",
                extracted_strings.len(),
                file
            );
            all_extracted_strings.extend(extracted_strings);
        }
        export_json(all_extracted_strings);
    }
}
