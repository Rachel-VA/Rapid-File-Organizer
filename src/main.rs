mod auto_organizer_watch;//import auto_organizer_watch module
mod manual_organizer; //import manual_organizer module
mod file_renamer; // import file_renamer module
mod splash_screen; //import splash_screen module

use ansi_term::Color;//to use colors
use std::io;


fn main() {
    // Call the splash screen function
    splash_screen::display_splash_screen();

    println!("\n\n\n\n");//blank lines

    println!("{}", introduction());//call the intro func
    println!("\n\n");
    //create a loop to let the user keep using it until they choose to exit
    loop {
        let choice = menu(); //create a var to store the menu options 

        match choice {
            1 => {
                // Call the auto_file_organizer func from the module
                auto_organizer_watch::watch_and_organize();
            },
            2 => {
                // Call the manual_file_organizer func from the module that imported above
                if let Err(err) = manual_organizer::organize_files() {
                    eprintln!("Error: {}", err);
                }
            },
            3 => {
                // Call the file_renamer func from the module
                file_renamer::run_renamer();  
            },
            4 => {
                println!("{}", Color::Blue.paint("\nYou've exited the program. Thank you for using the app!\n"));
                break;
            }
            _ => {
                println!("{}", Color::Red.paint("Invalid option. Please try again."));
            }
        }
    }
}

//func to hold the menu
fn menu() -> i32 {
    println!("\n🌸🌸🌸 SELECT AN OPTION: 🌸🌸🌸\n");
    println!("1. Auto organize files.");
    println!("2. Manually organize files.");
    println!("3. File Renamer");
    println!("4. Exit.");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    
    choice.trim().parse().unwrap_or(0)
}

//introduction func
fn introduction() -> String {
    let intro = format!(
        "{}\n{}\n{}\n{}\n",
        Color::Fixed(190).paint("======================================== OVERVIEW  ========================================\n"),
        Color::Fixed(211).paint(
            "\tRapid File Organizer is your go-to solution for maintaining a clean and organized file system.\n\n\
            OPTION 1: \
            Equipped with features to automatically organize files by monitoring your download folder and desktop\n\
            so that each time a new file is created, it'll be automatically detected and placed in a the destination folder.\n\n\
            OPTION 2: \
            It provides option to pass in a path to a folder and separate files based on extensions.\n\n\
            OPTION 3: \
            You can mass rename file all at once. \n\n\
            In short, this app is designed to let you manage your files effortlessly.\
            \n\n\n"
        ),
        Color::Purple.paint("\t\t👉 👉 👉 👉 👉 👉 INSTRUCTIONS 👈 👈 👈 👈 👈 👈\n"), // This is where the color is changed
        Color::Fixed(120).paint(
            "1. 😊 Auto File Organizer: Let the app monitor and automatically organize files from your desktop and download folders.\n\n\
            2. 😉 Manual Organize: Choose a specific folder you want to organize.\n\n\
            3. 😄 File Renamer: Rename files in bulk based on specific patterns.\n\n\
            Simply follow the on-screen prompts and let Rapid File Organizer handle the rest!\n\n\
            Happy organizing!👍👍"
        )
    );

    intro
}
