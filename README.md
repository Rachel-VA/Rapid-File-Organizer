# Rapid File Organizer

Rapid File Organizer is a command-line application for efficiently organizing and managing your files. It provides both automated and manual file organization, along with a bulk file renaming feature.

## Modules

This Rust project is divided into several modules:

- `auto_organizer_watch`: This module handles automatic file organization by monitoring specified directories for new files and categorizing them based on their extensions.

- `manual_organizer`: This module offers manual file organization, allowing you to specify source and destination directories and organize files manually.

- `file_renamer`: The file renamer module provides functionality for bulk file renaming, where you can specify patterns for renaming multiple files at once.

- `splash_screen`: The splash screen module displays a colorful introductory splash screen when you run the application.

## Dependencies

- `ansi_term`: Used for adding colors to the terminal output.

- `std::io`: Standard input and output for user interactions.

- `regex`: For pattern matching and manipulation of filenames in the file renaming module.

- `notify`: Enables monitoring the file system for changes and organizing files automatically.

## Usage

1. Run the program.

2. You will see a splash screen displaying the application name and a brief overview.

3. Choose an option from the menu:

    - Option 1: Auto organize files.
    
    - Option 2: Manually organize files.
    
    - Option 3: File Renamer.
    
    - Option 4: Exit.

4. Follow the on-screen instructions for each option.

5. Enjoy a clean and organized file system!

## Building and Running

To build and run the program, make sure you have Rust and Cargo installed. Then, execute the following commands:

```bash
# Build the project
cargo build

# Run the project
cargo run
