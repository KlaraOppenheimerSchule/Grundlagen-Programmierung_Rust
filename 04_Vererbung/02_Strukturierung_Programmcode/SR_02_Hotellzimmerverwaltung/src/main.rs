#![allow(unused_imports)]

use colored::*;
use structure::Hotel;
use std::time::Duration;


use clap::{App, load_yaml};
use std::io::Write;

mod structure;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();



    if let Some(path) = matches.value_of("FILE") {
        println!("File supplied: {}", path);

        let idk = std::path::Path::new(path).exists();
        if !idk {
            println!("But file does not exist");
        } else {
            let mut my_hotel = Hotel::load_json(path);
            my_hotel.main_menu();
        }
    } else {
        let mut hotel_name = String::new();
        let mut file_name = String::new();

        println!("{}", "Data file not supplied, creating new one".bold().underline().red());
        println!("Choose a name for the file: [name].json");
        print!(">> ");
        let _ = std::io::stdout().flush(); // If not used, the ">>" would not be displayed
        let _ = std::io::stdin().read_line(&mut file_name).unwrap();


        println!("Choose a name for the Hotel");
        print!(">> ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut hotel_name).unwrap();


        let my_hotel = Hotel::new(14, 100, hotel_name.trim().to_string(), file_name.trim().to_string());
        let _ = my_hotel.save_to_file();
    }
}