use std::fs::File;
use std::io::prelude::*;

use serde::Serialize;
use serde::Deserialize;
use serde_json::{Result, Value};
use serde_json as json;

use colored::*;
use std::convert::TryInto;


use std::io::stdout;
use std::io::stdin;
use sha2::digest::DynDigest;
use sha2::Digest;
use std::borrow::{Borrow, BorrowMut};


#[derive(Debug, Serialize, Deserialize)]
struct HotelZimmer {
    zimmer_nummer: u64,
    password: Option<String>,
    besetzt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Etage {
    name: String,
    zimmer: Vec<HotelZimmer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hotel {
    name: String,
    filename: String,
    etagen: Vec<Etage>,
}


impl Etage {
    #[allow(dead_code)] // Komich, compiler beschwert sich obwohl es in der Hotel impl benutzt wird
    pub fn new(raum_num: u64, hotel_ref: &Hotel) -> Self {
        let next_etagen_num = hotel_ref.etagen.len() as u64;
        let etagen_string = String::from("E") + &next_etagen_num.to_string();

        let mut zimmer: Vec<HotelZimmer> = Vec::new();

        for i in 1..raum_num {
            zimmer.push( HotelZimmer {
                zimmer_nummer: i,
                password: None,
                besetzt: false,
            });
        }

        Self {
            name: etagen_string,
            zimmer,
        }
    }
}

impl Hotel {
    pub fn new(etagen_num: u64, zimmer_in_etage: u64, name: String, filename: String) -> Self {
        let mut etagen: Vec<Etage> = Vec::new();

        for e in 0..etagen_num {
            let mut etagen_string = String::from("E") + &e.to_string();
            if e == 0 {
                etagen_string = String::from("EG");
            }

            let mut zimmer: Vec<HotelZimmer> = Vec::new();

            for z in 0..zimmer_in_etage {
                let curr_zimmer = HotelZimmer {
                    zimmer_nummer: z + 1,
                    password: None,
                    besetzt: false,
                };

                zimmer.push(curr_zimmer);
            }
            etagen.push(Etage {
                name: etagen_string,
                zimmer,
            });
        }

        Self {
            name,
            filename,
            etagen,
        }
    }

    pub fn reserv_room(&mut self, etage_index: usize, zimmer_index: usize, password: String) -> bool {
        let mut success = false;
        use sha2::{Sha512, Digest};
        let mut zimmer = self.etagen.get_mut(etage_index).unwrap().zimmer.get_mut(zimmer_index).unwrap();

        // Falls das ausgewählte Zimmer noch nicht bereits besetzt ist wird dies auf besetzt gestellt und ein vom Nutzer gewähltes passwort gespeichert
        if !zimmer.besetzt {
            let mut hasher = Sha512::new();

//            hasher.update(password.as_bytes());
            Digest::update(&mut hasher, password.as_bytes());
            let result: String = format!("{:X}", hasher.finalize());

            zimmer.besetzt = true;
            zimmer.password = Some(result);

            success = true;
        }

        return success;
    }

    pub fn load_json(filename: &str) -> Self {
        let mut file = File::open(filename).unwrap();
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);

        let h: Hotel = json::from_str(&buffer).unwrap();

        return h;
    }

    pub fn save_to_file(&self) -> Result<()> {
        let json_hotel = json::to_string(&self)?;

        let mut file = File::create(self.filename.as_str()).unwrap();
        file.write_all(json_hotel.as_bytes()).unwrap();

        Ok(())
    }

    pub fn display_waiter() {
        for i in vec![3, 2, 1] {
            print!("\r{}", i);
            let _ = stdout().flush();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    pub fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn reservieren_menu(&mut self) {
        let mut buffer = String::new();
        Self::clear_terminal();

        println!("{}", "Raum reservieren".underline().bold().blue());
        println!("{}", "Wählen sie eine Etage aus:".bold());

        for (i, e) in self.etagen.iter().enumerate() {
            println!("{}. {}", i, e.name.bold());
        }

        print!("{}", ">> ".bold());
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut buffer).unwrap();

        let etage_index: usize = buffer.trim().parse().unwrap();
        let curr_etage = self.etagen.get(etage_index).unwrap();

        buffer.clear();
        Self::clear_terminal();

        println!("{}", format!("Bitte wählen sie ein Raum aus [Ziffer {}-{}]", "0".red(), (curr_etage.zimmer.len() - 1).to_string().red() ));
        print!("{}", ">> ".bold());
        let _ = stdout().flush();

        let _ = stdin().read_line(&mut buffer).unwrap();
        let zimmer_index: usize = buffer.trim().parse().unwrap();
        buffer.clear();

        println!("{}", "Bitte wählen sie ein Passwort");
        print!("{}", ">> ".bold());
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut buffer).unwrap();

        let password = buffer.trim();


        let success = self.reserv_room(etage_index, zimmer_index, password.to_string());
        if !success {
            println!("{}", "Dieses Zimmer ist schon reserviert!".red().underline().bold());
            Self::display_waiter();
        }

        let _ = stdout().flush();

        let _ = self.save_to_file();
    }

    fn show_free_rooms(&self) {

    }

    #[allow(dead_code)] // Obwohl Funktion in der impl hier benutzt wird, beschwert sich der compiler
    fn add_etage_menu(&mut self) {
        let mut buffer = String::new();

        Self::clear_terminal();
        println!("{}", "Etage hinzufügen".underline().bold().blue());
        println!("{}", "Wie viele Räume auf etage?".underline());
        print!("{}", ">> ".bold());
        let _ = stdout().flush();

        let _ = stdin().read_line(&mut buffer).unwrap();

        let raum_zahl: u64 = buffer.trim().parse().unwrap();
        self.etagen.push(Etage::new(raum_zahl, &self));
        let _ = self.save_to_file();
    }

    #[allow(dead_code)] // Obwohl Funktion in der impl hier benutzt wird, beschwert sich der compiler
    fn etagen_menu(&mut self) {
        let mut buffer = String::new();

        loop {
            buffer.clear();
            Self::clear_terminal();

            println!("{}", "Etagen managen".underline().bold().blue());
            println!("{}", "  1. Etage hinzufügen".green());
            println!("{}", "  2. Etagen entfernen [TODO]".green());
            println!("{}", "  3. Zurück".green());
            print!("{}", ">> ".bold());
            let _ = stdout().flush();

            let _ = stdin().read_line(&mut buffer).unwrap();
            match buffer.trim() {
                "1" => {
                    self.add_etage_menu();
                    Self::clear_terminal();
                },
                "2" => {
                    todo!();
                },
                "3" => {
                    break;
                }
                _ => {
                    println!("{}", "Invalid choice".bold().red());
                    Self::display_waiter();
                }
            }
        }
    }

    pub fn main_menu(&mut self) {
        let mut buffer = String::new();

        loop {
            buffer.clear();

            Self::clear_terminal();

            println!("{}", "Hotelmanager".underline().bold().blue());
            println!("{}", "  1. Zimmer reservieren".green());
            println!("{}", "  2. Freie zimmer anzeigen".green());
            println!("{}", "  3. Exit".green());
            print!("{}", ">> ".bold());
            let _ = stdout().flush();

            let _ = stdin().read_line(&mut buffer).unwrap();
            match buffer.trim() {
                "1" => {
                    self.reservieren_menu();
                }
                "2" => {
                    self.show_free_rooms();
                }
                "3" => {
                    Self::clear_terminal();
                    break;
                }
                _ => {
                    println!("{}", "Invalid choice".red().bold());
                    Self::display_waiter();
                }
            }
        }
    }
}