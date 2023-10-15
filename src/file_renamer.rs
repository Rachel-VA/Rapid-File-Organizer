use ansi_term::Color;
use regex::Regex;//used for pattern matching and manipulation of filenames.
use std::fs;//working with file system operations
use std::io;
use std::path::PathBuf;//working with filesystem paths

//public func control the main logic of the file renaming process 
pub fn run_renamer() {
    display_introduction();//call the intro func

    loop {
        let dir_path = prompt_for_directory_path();
        let pattern = prompt_for_pattern();
        let new_name_or_pattern = prompt_for_new_name();

        rename_files(&dir_path, &pattern, &new_name_or_pattern);

        if !ask_to_continue() {
            println!("{}", Color::Blue.paint("\nThank you for using the file_renamer app. Goodbye!\n"));
            break;
        }
    }
}

fn display_introduction() {
    let introduction = format!(
        "{}\n{}",
        Color::Green.paint("\n\n\n========================= WELCOME TO THE FILE RENAMER APP =================================\n\n"),
        Color::Yellow.paint("This user-friendly app enables you to easily mass rename files/images and return them with numerical order in a directory.\n\
                            Whether you wish to rename files based on specific patterns or criteria,\n\
                            the File Renamer app is your go-to tool for effortless managing file names.\n\
                            \n\n\
                            ************************* INSTRUCTIONS: *********************************\n\n\
                            1. Enter the path to the folder containing the files you want to rename.\n\
                            2. Input the regular expression pattern to match the section of filenames you wish to modify.\n\
                            3. Provide the new name or regular expression pattern for renaming the files.\n\
                            \n\
                            \n\
                            EXAMPLE:\n\
                            To rename files in the folder 'C:\\Users\\Username\\Pictures\\Test' that begin with 'IMG' and replace\n\
                            it with 'Seatle', follow the prompts.\n\n\
                            Happy file renaming!\n")
    );
    println!("{}", introduction);
}

//ask user to enter a path and accept it logic
fn prompt_for_directory_path() -> String {
    println!("{}", Color::Cyan.paint("Enter the path to the folder containing the files you want to rename:"));
    let mut dir_path_input = String::new();
    io::stdin().read_line(&mut dir_path_input).expect("Failed to read input");
    dir_path_input.trim().to_string()
}
//ask user to enter the name of the pattern and accept it logic
fn prompt_for_pattern() -> String {
    println!("Enter the pattern name that you want to rename:");
    let mut pattern_input = String::new();
    io::stdin().read_line(&mut pattern_input).expect("Failed to read input");
    pattern_input.trim().to_string()
}
//ask user to enter the new name logic 
fn prompt_for_new_name() -> String {
    println!("{}", Color::Purple.paint("Enter the new name:"));
    let mut new_name_or_pattern_input = String::new();
    io::stdin().read_line(&mut new_name_or_pattern_input).expect(&Color::Red.paint("Failed to read input").to_string());
    new_name_or_pattern_input.trim().to_string()
}
// Rename files based on user inputs
fn rename_files(dir_path: &str, pattern: &str, new_name_or_pattern: &str) {
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => {
            eprintln!(
                "{} {}",
                Color::Red.paint("Error: Invalid regex pattern:"),
                e
            );
            return;
        }
    };
//attempts to read the contents of the directory specified by the dir_path variable
    let files = match fs::read_dir(dir_path) {
        Ok(files) => files,//if read succeed, then go over the entire folder
        Err(e) => {
            eprintln!(
                "{} {}",
                Color::Red.paint("Error: Failed to read file directory:"),
                e
            );
            return;
        }
    };

    let mut counter = 1;//start from 1 to all of the items in the folder

    for file in files {
        if let Ok(entry) = file {
            //if succeed, returns the filename of the directory entry as an OsString
            let file_name = entry.file_name();
            let old_name = file_name.to_string_lossy().to_string();
        //check the name of the pattern, if matched, then change name
            if re.is_match(&old_name) {
                let ext = match entry.path().extension().and_then(|e| e.to_str()) {
                    Some(ext) => ext.to_owned(),
                    None => {
                        eprintln!(
                            "{} {}",
                            Color::Red.paint("Error: Failed to get file extension."),
                            &old_name
                        );
                        continue;
                    }
                };
            //rename logic
                let new_name = format!("{}_{:04}.{}", new_name_or_pattern, counter, ext);

                counter += 1;//label with number
                //create a new path to the folder to store the items after rename them
                let new_path = PathBuf::from(dir_path).join(&new_name);
                if let Err(e) = fs::rename(entry.path(), new_path) {
                    eprintln!("{} {}", Color::Red.paint("Error renaming file:"), e);
                } else {
                    println!("Renamed: {} -> {}", old_name, new_name);
                }
            }
        }
    }

    println!("{}", Color::Fixed(211).paint("Files renamed successfully!"));
}

// Ask whether to continue renaming
fn ask_to_continue() -> bool {
    println!("\nDo you want to rename more files? (y/n)\n");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read input");
    let answer = answer.trim().to_lowercase();
    answer == "y" || answer == "yes"
}


