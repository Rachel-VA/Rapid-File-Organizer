use std::{thread, time, io::{self, Write}};
//use rand::prelude::*;
use rand;
use ansi_term::Color;

//public func control the main logic of the code, it's used to be called from main()
pub fn display_splash_screen() {
    //use const keywork for customazation width & height
    const WIDTH: usize = 40;
    const HEIGHT: usize = 5;
    const DISPLAY_DURATION: time::Duration = time::Duration::from_secs(5);//total duration 
    const FRAME_INTERVAL: time::Duration = time::Duration::from_millis(200); // duration between each frame
    //vec store the icons and colors to be randomly mixed
    let icons = vec!["❤️", "❤️", "❤️", "❤️", "❤️", "❤️", "❤️", "❤️"];
    let colors = vec![Color::Cyan, Color::Yellow, Color::Red, Color::Green, Color::White]; // Colors for the title
    let mut color_index = 0;//initiate the color index from 0
    let  _rng = rand::thread_rng();//random generator. the underscore is used to subpress compiler cuz it's unused

    let start_time = time::Instant::now();//record the starting time
    //the loop will run for 5 secs
    while start_time.elapsed() < DISPLAY_DURATION {
        let title = colors[color_index].paint("\t\t\t\tRAPID FILE ORGANIZER").to_string();
        let width = WIDTH + title.len();
        let height = HEIGHT;
        let title_y = height / 2;
        let title_start = (width - title.len()) / 2;

        // Clear the screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        //draw the splash screene  with icons and colors that stored in the vecs above
        for y in 0..height {
            let mut title_chars = title.chars();
            for x in 0..width {
                if y == 0 || y == height - 1 {
                    let icon_index = (x + color_index) % icons.len(); // Create the wave-like motion
                    print!("{}", icons[icon_index]);
                } else if y == title_y && (x >= title_start && x < title_start + title.len()) {
                    print!("{}", title_chars.next().unwrap());
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        //witch between colors in the colors vector
        color_index = (color_index + 1) % colors.len(); // Change the color of the title for the next frame

        io::stdout().flush().unwrap();//flush the output buffer
        thread::sleep(FRAME_INTERVAL);//sleep for the specific time based on the customized number that defined in the FRAME_INTERVAL
    }
}