//notify crate to monitor comp sytem and use the neccessary traits
use notify::{Watcher, EventHandler, Config, RecursiveMode, Event, Error};
use std::collections::HashMap;//data structure that stores key-value pairs/collection
use std::fs;//working with file system
use std::path::{Path, PathBuf};//paths to files and directories
use ansi_term::Color;
//create a struct w/ 2 fileds to hold the path of the organized folder destination
struct FileOrganizer {
    organized_folder: PathBuf,
    extension_map: HashMap<String, String>, //to map various extensions
}//implement the eventhandler for the struct
impl EventHandler for FileOrganizer {
    fn handle_event(&mut self, result: Result<Event, Error>) {
        if let Ok(event) = result {//pattern matching to extract event from the Result
            match event.kind {
                // Handle specific kinds of events
                notify::EventKind::Create(notify::event::CreateKind::File) | //detect file new files are created
                notify::EventKind::Create(notify::event::CreateKind::Any) |
                notify::EventKind::Modify(_) => {
                    //detect file created with name changed
                    if let Some(extension) = event.paths[0].extension() {//check extension
                        let extension_str = extension.to_str().unwrap().to_lowercase();//convert extensions to lowercase to ensure match
                        if let Some(folder_name) = self.extension_map.get(&extension_str) {//try to get th extensions in the map
                            let dest_folder = self.organized_folder.join(folder_name);
                            if !dest_folder.exists() {//check if no folder exist, create ones to sort files into different folders
                                fs::create_dir_all(&dest_folder).unwrap();
                                println!("\nCreated directory: {:?}", dest_folder);
                            }
                            //constructs the destination path for the file by joining the destination folder path and the file name from the affected path
                            let dest_path = dest_folder.join(event.paths[0].file_name().unwrap());
                            if let Err(e) = fs::rename(&event.paths[0], &dest_path) {
                                eprintln!("\nERROR MOVING FILE: {:?}", e);
                            } else {
                                println!("\nFILES MOVED TO : {:?}", dest_path);
                            }
                        }
                    }
                }
                _ => {} // Ignore other kinds of events. If we want to printout use this: println!("Unhandled event kind: {:?}", event.kind);
            }
        } else if let Err(e) = result {
            eprintln!("\nError processing event: {:?}", e);
        }
    }
}


//declare using public func so that we can call it in main
pub fn watch_and_organize() {
    display_introduction(); //call the display_instruction func here similar to main()
    println!("\n\n");
    // The path where organized files will be kept
    let organized_folder = "C:\\fileAutoOrganized"; 

    // Mapping from file extensions to folder names
    let mut extension_map: HashMap<String, String> = HashMap::new();
    extension_map.insert("jpg".to_string(), "Pictures".to_string());
    extension_map.insert("png".to_string(), "Pictures".to_string());
    extension_map.insert("txt".to_string(), "Documents".to_string());
    extension_map.insert("pdf".to_string(), "Document".to_string());
    extension_map.insert("docx".to_string(), "Document".to_string());
    extension_map.insert("mov".to_string(), "Videos".to_string());
    //add more as needed

    // Initialize the FileOrganizer with the organized_folder and extension_map
    let handler = FileOrganizer { 
        organized_folder: PathBuf::from(organized_folder),
        extension_map
    };

    // Set up the watcher
    let mut watcher = notify::ReadDirectoryChangesWatcher::new(handler, Config::default()).unwrap();

    // Watching Downloads directory
    let downloads_path = Path::new("C:\\Users\\13039\\Downloads");
    println!("\nWatching the download path: {:?}", downloads_path);
    watcher.watch(&downloads_path, RecursiveMode::Recursive).unwrap();

    // Watching Desktop directory
    let desktop_path = Path::new("C:\\Users\\13039\\Desktop");
    println!("Watching the desktop path: {:?}", desktop_path);
    watcher.watch(&desktop_path, RecursiveMode::Recursive).unwrap();

    // create a loop so that the app will continuously monitor every 5 secs. we can change as desire
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}


fn display_instructions(instructions: &str) {
    println!("{}", Color::Fixed(218).paint(instructions));//apply color
}
fn display_introduction() {
    let instructions = "\n\t\tOVERVIEW AND INSTRUCTIONS\n\
                        While this app is running, it's monitoring the DOWNLOAD and DESKTOP folders.\n
                        The app will: \n
                        - Detect new files with extension: jpg, png, pdf, txt, dox , mov \n
                        - Move the items into the destination folder named fileAutoOrganized in the C drive.\n
                        - Items will be sort into sub-folders based on extension types\n\n
                        ***************** TO STOP THE APP: click X or press Ctrl+C *************************
                        ";
    
    display_instructions(instructions);
}