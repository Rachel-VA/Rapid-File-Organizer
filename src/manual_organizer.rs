
use std::fs;//working w/ file operating sys
use std::io::{self, Write};//importing both the entire std::io module and writing trait to use flush()
use std::path::PathBuf;//working with filesystem paths
use log::{info, error};//macros from the log crate for logging to keep track on the code functionality
use ansi_term::Colour; // Import the Colour enum 
//use env_logger;


//public func control the main logic of the file organizer and handle error
pub fn organize_files() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\n");
    display_introduction(); // Call the introduction function
    println!("\n");
    // Prompt the user for the source directory path
    print!("{}", Colour::Cyan.paint("Enter the path to the folder you want to organize: "));
    let source_dir = get_input("")?;
    //let source_dir = get_input("Enter the path to the folder you want to organize: ")?;

    // Prompt the user for the destination directory path
    print!("{}", Colour::Purple.paint("Enter the path to the folder you want to organize: "));//add in color
    let dest_dir = get_input("")?;
    //let dest_dir = get_input("Enter the path to the destination foolder: ")?; 

    // Convert the destination path to a `PathBuf`
    let dest_dir = PathBuf::from(dest_dir);

    // Create the destination directory if it doesn't exist
    if let Err(err) = fs::create_dir_all(&dest_dir) {
        eprintln!(//apply color
            "{} {}",
            Colour::Red.paint("Error creating destination directory:"),
            err
        );
        return Err(err.into());
        //error!("Error creating destination directory: {}", err);
        //return Err(err.into());
    }

    // Read the source directory and iterate over its contents
    for entry in fs::read_dir(&source_dir)? {
        let entry = entry?;
        let file_path = entry.path();

        // Ignore directories (you can modify this logic if you want to handle them)
        if file_path.is_file() {
            // Get the file extension
            let file_extension = file_path
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_lowercase); // Convert extension to lowercase for uniformity

            match file_extension {
                Some(extension) => {
                    // Create a folder with the extension name if it doesn't exist
                    let dest_subdir = dest_dir.join(extension);
                    if !dest_subdir.exists() {
                        if let Err(err) = fs::create_dir(&dest_subdir) {
                            error!("Error creating destination subdirectory: {}", err);
                            continue;
                        }
                    }

                    // Move the file into the corresponding folder
                    let dest_file_path = dest_subdir.join(file_path.file_name().unwrap());   
                    if let Err(err) = fs::rename(&file_path, &dest_file_path) {
                       
                        error!("ERROR MOVING FILE: {}", err);

                    } else {
                        info!("Moved file {} to {}", file_path.display(), dest_file_path.display());
                    }
                }
                None => {
                    // Handle files without extensions (move them to a folder named "misc")
                    let dest_subdir = dest_dir.join("misc");
                    if !dest_subdir.exists() {
                        if let Err(err) = fs::create_dir(&dest_subdir) {
                            error!("ERROR CREATING DESTINATION FOLDER DIRECTORY: {}", err);
                            continue;
                        }
                    }

                    let dest_file_path = dest_subdir.join(file_path.file_name().unwrap());
                    if let Err(err) = fs::rename(&file_path, &dest_file_path) {
                        error!("ERROR MOVING FILE: {}", err);
                    } else {
                        info!("Moved file {} to {}", file_path.display(), dest_file_path.display());
                    }
                }
            }
        }
    }

    //println!("File auto-organization has completed!");
    println!("{}", Colour::Fixed(215).paint("File organization is completed!"));
    Ok(())//return result
}

// read user input to get the paths input from the user
fn get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);// Print the prompt text without a newline
    io::stdout().flush()?;// Flush the stdout buffer to make sure the prompt is displayed
    let mut input = String::new();// Create a mutable String to store user input
    io::stdin().read_line(&mut input)?;// Read the user input and store it in the 'input' variable
    Ok(input.trim().to_string())// Trim the input (remove leading/trailing whitespace) and return it as a Result
}


fn display_introduction() {
    let introduction = "\n\t\tOVERVIEW AND INSTRUCTIONS\n\n\
                        This app organizes files based on their extensions.\n\n\
                        \t🦋🦋🦋 Follow the instructions below: 🦋🦋🦋\n\n\
                        1. Enter the path to the folder containing files which you want to organize.\n\
                        2. Enter the path to the destination folder where you want to keep the files.\n\
                        3. Go to the destination folder to find the sub-folders containing the organized items\n";
    println!("{}", Colour::Fixed(180).paint(introduction));
}