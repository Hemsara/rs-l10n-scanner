# Flutter String Scanner

A Rust CLI tool that scans Flutter/Dart projects to extract translatable strings, making internationalization (i18n) easier by automatically identifying all string literals in your codebase.

## Overview

This is my first Rust project! It combines the power of Rust's performance and safety with Dart's analysis capabilities to help Flutter developers prepare their apps for localization. The scanner traverses your Flutter project's source code, extracts all string literals, and exports them to a structured JSON format.

## Features

- **Automatic Discovery**: Recursively scans all `.dart` files in your Flutter project's `lib` directory
- **Smart Validation**: Verifies the target directory is a valid Flutter project (checks for `pubspec.yaml` and `lib` folder)
- **String Extraction**: Uses a Dart analyzer script to identify and extract translatable strings
- **JSON Export**: Outputs all extracted strings to a formatted JSON file for easy processing
- **Clean Architecture**: Modular design with separate concerns (scanning, analyzing, exporting, validation)

## Project Structure

```
scanner/
├── src/
│   ├── analyzer/          # Runs Dart analyzer to extract strings
│   │   ├── mod.rs
│   │   └── dart_analyzer.rs
│   ├── scanner/           # File system traversal logic
│   │   ├── mod.rs
│   │   └── file_scanner.rs
│   ├── exporter/          # JSON export functionality
│   │   ├── mod.rs
│   │   └── json_exporter.rs
│   ├── validators/        # Path and directory validation
│   │   ├── mod.rs
│   │   └── path.rs
│   └── main.rs           # Entry point
├── extract_strings/       # Dart analyzer script
└── Cargo.toml
```

## Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Dart SDK**: Required to run the string extraction script

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd scanner
```

2. Build the project:
```bash
cargo build --release
```

The compiled binary will be available at `target/release/scanner`

## Usage

1. Run the scanner:
```bash
cargo run
```

2. When prompted, enter the path to your Flutter project:
```
🗂️ Please enter the path of your flutter app to scan:
/path/to/your/flutter/project
```

3. The tool will:
   - Validate the Flutter project structure
   - Scan all `.dart` files in the `lib` directory
   - Extract strings from each file
   - Display progress and statistics
   - Generate `output.json` with all extracted strings

### Example Output

```
✅ The path is a valid Flutter app directory. Continuing...
📄 Found 45 Dart files:
 - 12 strings extracted from /path/to/project/lib/main.dart
 - 8 strings extracted from /path/to/project/lib/screens/home.dart
 - 15 strings extracted from /path/to/project/lib/widgets/button.dart
 ...
```

The generated `output.json` will contain:
```json
{
  "welcome": "Welcome to the app",
  "login": "Login",
  "signup": "Sign Up",
  ...
}
```

## How It Works

1. **Path Validation** (`validators/path.rs`): Checks if the provided path is a valid Flutter project by verifying the existence of `pubspec.yaml` and the `lib` directory

2. **File Scanning** (`scanner/file_scanner.rs`): Uses depth-first search (DFS) to recursively traverse the `lib` directory and collect all `.dart` files

3. **String Extraction** (`analyzer/dart_analyzer.rs`): Executes the Dart analyzer script on each file to extract string literals, filtering out:
   - Strings starting with `$` (interpolated variables)
   - Single-character strings

4. **JSON Export** (`exporter/json_exporter.rs`): Aggregates all extracted strings and exports them to a formatted JSON file using the first word of each string as the key

## Dependencies

- `serde` (1.0): Serialization framework
- `serde_json` (1.0): JSON support for serde

## Learning Notes

As my first Rust project, I learned about:
- Rust's module system and code organization
- File I/O and path handling with `std::fs` and `std::path`
- Process spawning with `Command` to interact with Dart
- Error handling with `Result` and `expect`
- String manipulation and ownership
- Working with vectors and iterators
- Using external crates (serde/serde_json)

## Future Improvements

- [ ] Add command-line arguments support (remove interactive prompt)
- [ ] Support custom output file paths
- [ ] Add progress bars for better UX
- [ ] Handle duplicate strings more intelligently
- [ ] Add configuration file support
- [ ] Implement parallel file processing
- [ ] Add unit tests
- [ ] Support for other file formats (CSV, YAML, etc.)

## License

See [LICENSE.md](LICENSE.md) for details.

## Contributing

This is a learning project, but suggestions and improvements are welcome! Feel free to open an issue or submit a pull request.

---

**Note**: This project requires the `extract_strings` Dart script to be present in the project directory for string extraction to work properly.
